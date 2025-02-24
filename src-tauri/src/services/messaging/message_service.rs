use super::{Message, MessageError};
use crate::services::contacts::ContactsService;
use entity::message;
use sea_orm::{
    ActiveValue::NotSet, Condition, DatabaseConnection, QueryOrder, Set, prelude::*,
    sqlx::types::chrono::Utc,
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
        log::info!("Started listening for messages...");
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
        log::info!("Stopped listening for messages.");

        Ok(())
    }

    async fn process_remote_data(
        data: &[u8],
        remote: SocketAddr,
        db: &DatabaseConnection,
        contacts: &ContactsService,
        app_handle: &AppHandle,
    ) -> crate::Result<()> {
        let message: Message = serde_json::from_slice::<Message>(data)?;
        log::debug!("Handling received message: {:?}", message);

        let contact = contacts.get_or_create_with_ip(remote.ip()).await?;
        let self_contact = contacts.get_self().await?;

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

    pub async fn send_message(&self, message: Message, to: Uuid) -> crate::Result<()> {
        let self_contact = self.contacts.get_self().await?;

        let receiver = self
            .contacts
            .get_with_uuid(to)
            .await?
            .ok_or(MessageError::SendToNonExistantContact)?;

        self.net
            .send_to(
                &message.content,
                (
                    Into::<IpAddr>::into(receiver.ip_address),
                    DEFAULT_MESSAGE_PORT,
                ),
            )
            .await?;

        // Save message to db
        entity::message::ActiveModel {
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

        Ok(())
    }

    /// Gets messages from local user to uuid
    pub async fn get_correspondence(&self, to_uuid: Uuid) -> crate::Result<Vec<Message>> {
        let self_contact = self.contacts.get_self().await?;

        // Is true for outgoing messages to the contact, or incoming messages from the contact
        let correspondence_condition = Condition::any()
            .add(
                Condition::all()
                    .add(message::Column::FromUuid.eq(to_uuid))
                    .add(message::Column::ToUuid.eq(self_contact.uuid)),
            )
            .add(
                Condition::all()
                    .add(message::Column::FromUuid.eq(self_contact.uuid))
                    .add(message::Column::ToUuid.eq(to_uuid)),
            );

        let db_messages = message::Entity::find()
            .filter(correspondence_condition)
            .order_by(message::Column::SentAt, sea_orm::Order::Desc)
            .all(&self.db)
            .await?;

        let messages = db_messages.iter().map(Into::into).collect();
        Ok(messages)
    }
}
