use super::{Message, MessageError};
use crate::services::contacts::ContactsService;
use entity::message;
use sea_orm::{
    ActiveValue::NotSet, Condition, DatabaseConnection, IntoActiveModel, QueryOrder, Set,
    TryIntoModel, prelude::*, sqlx::types::chrono::Utc,
};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tauri::{AppHandle, Emitter};
use tokio::{net::UdpSocket, task::JoinHandle};
use uuid::Uuid;

pub const DEFAULT_MESSAGE_PORT: u16 = 45000;

pub struct MessageService {
    listening_task: Option<JoinHandle<()>>,
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
    ) -> crate::Result<Self> {
        log::info!("Setting up message service...");
        let net = UdpSocket::bind(format!("0.0.0.0:{}", DEFAULT_MESSAGE_PORT)).await?;

        let mut this = Self {
            listening_task: None,
            net: Arc::new(net),
            db: db.clone(),
            contacts,
            app_handle,
        };

        this.listen().await?;
        log::info!("Done setting up message service");

        Ok(this)
    }

    async fn listen(&mut self) -> crate::Result<()> {
        let net = Arc::clone(&self.net);
        let contacts = Arc::clone(&self.contacts);
        let db = self.db.clone(); // We can clone it because it's an Arc to a Sqlx pool internally.
        let app_handle = self.app_handle.clone();

        let mut buf = [0; 1024];
        let task = tokio::spawn(async move {
            loop {
                let (len, remote) = match net.recv_from(&mut buf).await {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("Error while reading from message socket! {:?}", e);
                        continue;
                    }
                };

                let data = &buf[..len];
                match Self::process_remote_data(data, remote, &db, &contacts, &app_handle).await {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("Error occurred while processing incoming data! {:?}", e);
                        continue;
                    }
                };
            }
        });
        self.listening_task = Some(task);

        log::info!("Listening for messages...");

        Ok(())
    }

    async fn process_remote_data(
        data: &[u8],
        remote: SocketAddr,
        db: &DatabaseConnection,
        contacts: &ContactsService,
        app_handle: &AppHandle,
    ) -> crate::Result<()> {
        log::debug!("Received message from: {:?}", remote);
        let message: Message = serde_json::from_slice::<Message>(data)?;
        log::debug!("Deserialized received message: {:?}", message);

        let mut contact = contacts
            .get_or_create_with_ip(remote.ip(), Some(message.remote_uuid))
            .await?;
        let self_contact = contacts.get_self().await?;

        // Update local contact uuid to the remote uuid if it's different
        if contact.uuid != message.remote_uuid {
            log::info!("Remote contact UUID changed, updating local contact...");
            let old_contact = contact.clone();

            let mut active_contact = contact.into_active_model();
            active_contact.uuid = Set(message.remote_uuid);

            let updated_contact = contacts.update_contact(active_contact).await?;
            contact = updated_contact.try_into_model()?;

            // Set entities with the old FROM uuid to the new FROM uuid
            message::Entity::update_many()
                .col_expr(message::Column::FromUuid, Expr::value(contact.uuid))
                .filter(message::Column::FromUuid.eq(old_contact.uuid))
                .exec(db)
                .await?;

            // Set entities with the old TO uuid to the new TO uuid
            message::Entity::update_many()
                .col_expr(message::Column::ToUuid, Expr::value(contact.uuid))
                .filter(message::Column::ToUuid.eq(old_contact.uuid))
                .exec(db)
                .await?;

            // Notify frontend
            app_handle.emit("contact-changed", contact.clone())?;
        }

        message::ActiveModel {
            id: NotSet,
            from_uuid: Set(contact.uuid.clone()),
            to_uuid: Set(self_contact.uuid.clone()),
            content_type: Set(message.content_type.clone()),
            content: Set(message.content.clone()),
            received: Set(true),
            sent_at: Set(message.sent_at.naive_utc()),
        }
        .save(db)
        .await?;
        log::debug!("Wrote incoming message to db");

        // Notify frontend
        app_handle.emit("message-received", message)?;

        Ok(())
    }

    pub async fn send_message(&self, message: Message, to: Uuid) -> crate::Result<message::Model> {
        let self_contact = self.contacts.get_self().await?;

        let receiver = self
            .contacts
            .get_with_uuid(to)
            .await?
            .ok_or(MessageError::SendToNonExistantContact)?;

        self.net
            .send_to(
                serde_json::to_vec(&message)?.as_slice(),
                (
                    Into::<IpAddr>::into(receiver.ip_address),
                    DEFAULT_MESSAGE_PORT,
                ),
            )
            .await?;

        // Save message to db
        let active_sent_msg = entity::message::ActiveModel {
            id: NotSet,
            to_uuid: Set(to),
            from_uuid: Set(self_contact.uuid),
            content_type: Set(message.content_type),
            content: Set(message.content),
            received: Set(false),
            sent_at: Set(Utc::now().naive_utc()),
        }
        .save(&self.db)
        .await?;

        let sent_message = active_sent_msg.try_into_model()?.into();

        Ok(sent_message)
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
