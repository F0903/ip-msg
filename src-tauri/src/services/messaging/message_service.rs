use super::{Message, MessageError, net::ReceivedPacket};
use crate::services::{
    contacts::ContactsService,
    messaging::net::{NetworkListener, Packet},
};
use entity::{contact, ip_address::IpAddress, message};
use sea_orm::{
    ActiveValue::NotSet, Condition, DatabaseConnection, IntoActiveModel, QueryOrder, Set,
    TryIntoModel, prelude::*, sqlx::types::chrono::Utc,
};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tauri::{AppHandle, Emitter};
use tokio::net::UdpSocket;

pub const DEFAULT_MESSAGE_PORT: u16 = 45000;

pub struct MessageService {
    net: Arc<UdpSocket>,
    db: DatabaseConnection,
    contacts: Arc<ContactsService>,
    app_handle: AppHandle,
}

impl MessageService {
    pub async fn start(
        db: DatabaseConnection,
        contacts: Arc<ContactsService>,
        app_handle: AppHandle,
    ) -> crate::Result<Arc<Self>> {
        log::info!("Setting up message service...");

        let net = Arc::new(UdpSocket::bind(format!("0.0.0.0:{}", DEFAULT_MESSAGE_PORT)).await?);
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let mut listener = NetworkListener::new(net.clone(), tx);
        listener.start_listen().await?;

        let arc_self = Arc::new(Self {
            net,
            db,
            contacts,
            app_handle,
        });

        let rx_arc_self = Arc::clone(&arc_self);
        tokio::spawn(async move {
            while let Some(ReceivedPacket { remote, packet }) = rx.recv().await {
                if let Err(err) = match packet {
                    Packet::Message(message) => rx_arc_self.process_message(message, remote).await,
                    Packet::MessageReceived(message_id) => {
                        rx_arc_self
                            .process_message_received(message_id, remote)
                            .await
                    }
                } {
                    log::error!("Error while processing received packet: {:?}", err);
                }
            }
        });

        log::info!("Done setting up message service");

        Ok(arc_self)
    }

    async fn assert_contact(
        &self,
        message: &Message,
        mut contact: contact::Model,
    ) -> crate::Result<contact::Model> {
        // Update local contact uuid to the remote uuid if it's different
        if contact.uuid == message.remote_uuid {
            return Ok(contact);
        }

        log::info!("Remote contact UUID changed, updating local contact...");
        let old_contact = contact.clone();

        let mut active_contact = contact.into_active_model();
        active_contact.uuid = Set(message.remote_uuid);

        let updated_contact = self.contacts.update_contact(active_contact).await?;
        contact = updated_contact.try_into_model()?;

        // Set entities with the old FROM uuid to the new FROM uuid
        message::Entity::update_many()
            .col_expr(message::Column::FromUuid, Expr::value(contact.uuid))
            .filter(message::Column::FromUuid.eq(old_contact.uuid))
            .exec(&self.db)
            .await?;

        // Set entities with the old TO uuid to the new TO uuid
        message::Entity::update_many()
            .col_expr(message::Column::ToUuid, Expr::value(contact.uuid))
            .filter(message::Column::ToUuid.eq(old_contact.uuid))
            .exec(&self.db)
            .await?;

        // Notify frontend
        self.app_handle.emit("contact-changed", contact.clone())?;
        Ok(contact)
    }

    async fn process_message(&self, message: Message, remote: SocketAddr) -> crate::Result<()> {
        let contact = self
            .contacts
            .get_or_create_with_ip(remote.ip(), Some(message.remote_uuid))
            .await?;
        let contact = self.assert_contact(&message, contact).await?;

        let self_contact = self.contacts.get_self().await?;

        let active_received_msg = message::ActiveModel {
            id: NotSet,
            from_uuid: Set(contact.uuid.clone()),
            to_uuid: Set(self_contact.uuid.clone()),
            content_type: Set(message.content_type.clone()),
            content: Set(message.content.clone()),
            received: Set(true),
            sent_at: Set(message.sent_at.naive_utc()),
        }
        .save(&self.db)
        .await?;
        log::debug!("Wrote incoming message to db");

        // Notify frontend
        let received_msg = active_received_msg.try_into_model()?;
        self.app_handle.emit("message-received", received_msg)?;

        //self.send_message_received(message_id, to);

        Ok(())
    }

    async fn process_message_received(
        &self,
        message_id: i32,
        remote: SocketAddr,
    ) -> crate::Result<()> {
        // Set the message as received in the db
        message::ActiveModel {
            id: Set(message_id),
            received: Set(true),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        self.send_message_received(message_id, remote.into())
            .await?;

        self.app_handle
            .emit("remote-message-received", message_id)?;

        Ok(())
    }

    async fn send_message_received(&self, message_id: i32, to: IpAddress) -> crate::Result<()> {
        self.net
            .send_to(
                serde_json::to_vec(&Packet::MessageReceived(message_id))
                    .unwrap()
                    .as_slice(),
                (Into::<IpAddr>::into(to), DEFAULT_MESSAGE_PORT),
            )
            .await?;

        Ok(())
    }

    pub async fn send_message(
        &self,
        message: impl IntoActiveModel<message::ActiveModel>,
    ) -> crate::Result<message::Model> {
        let message = message.into_active_model();
        let to = *message
            .to_uuid
            .try_as_ref()
            .ok_or(MessageError::NoRecipient)?;

        let receiver = self
            .contacts
            .get_with_uuid(to)
            .await?
            .ok_or(MessageError::SendToNonExistantContact)?;

        // Save message to db (we have to do this before sending it over the network)
        let self_contact = self.contacts.get_self().await?;
        let db_message = entity::message::ActiveModel {
            from_uuid: Set(self_contact.uuid),
            to_uuid: Set(to),
            received: Set(false),
            sent_at: Set(Utc::now().naive_utc()),
            ..message
        }
        .save(&self.db)
        .await?
        .try_into_model()?;

        let network_message: Message = db_message.clone().into();
        self.net
            .send_to(
                serde_json::to_vec(&network_message)?.as_slice(),
                (
                    Into::<IpAddr>::into(receiver.ip_address),
                    DEFAULT_MESSAGE_PORT,
                ),
            )
            .await?;

        Ok(db_message)
    }

    /// Gets messages from local user to uuid
    pub async fn get_correspondence(&self, to_id: i32) -> crate::Result<Vec<message::Model>> {
        let self_contact = self.contacts.get_self().await?;
        let to_contact = self
            .contacts
            .get_with_id(to_id)
            .await?
            .ok_or(MessageError::ContactNotFound)?;

        // Is true for outgoing messages to the contact, or incoming messages from the contact
        let correspondence_condition = Condition::any()
            .add(
                Condition::all()
                    .add(message::Column::FromUuid.eq(to_contact.uuid))
                    .add(message::Column::ToUuid.eq(self_contact.uuid)),
            )
            .add(
                Condition::all()
                    .add(message::Column::FromUuid.eq(self_contact.uuid))
                    .add(message::Column::ToUuid.eq(to_contact.uuid)),
            );

        let messages = message::Entity::find()
            .filter(correspondence_condition)
            .order_by(message::Column::SentAt, sea_orm::Order::Desc)
            .all(&self.db)
            .await?;

        log::debug!(
            "Got correspondence for: '{:?}' -> '{:?}'\n\tmessages: {:?}",
            self_contact.uuid,
            to_contact.uuid,
            &messages
        );

        Ok(messages)
    }
}
