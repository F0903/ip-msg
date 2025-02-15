use super::{message_error::MessageError, Message, TextMessage};
use entity::message;
use sea_orm::prelude::ActiveModelTrait;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use std::sync::Arc;
use tokio::{net::UdpSocket, task::JoinHandle};

type Result<T> = std::result::Result<T, MessageError>;

pub struct MessageService {
    listening_task: Option<JoinHandle<()>>,
    net: Arc<UdpSocket>,
    db: DatabaseConnection,
}

//TODO: implement sending messages
//TODO: verify message database design
impl MessageService {
    pub async fn start(db: &DatabaseConnection) -> Result<Self> {
        let net = UdpSocket::bind("0.0.0.0:45000").await?;

        let mut this = Self {
            listening_task: None,
            net: Arc::new(net),
            db: db.clone(), // We can clone it because it's an Arc to a Sqlx pool internally.
        };

        this.listen().await?;

        Ok(this)
    }

    async fn listen(&mut self) -> Result<()> {
        let net = Arc::clone(&self.net);
        let db = self.db.clone(); // We can clone it because it's an Arc to a Sqlx pool internally.

        let mut buf = [0; 1024];
        let task = tokio::spawn(async move {
            loop {
                let count = match net.recv(&mut buf).await {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("Error while reading from message socket! {:?}", e);
                        continue;
                    }
                };

                let data = &buf[..count];
                match Self::process_data(data, &db).await {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("Error occurred while processing incoming data! {:?}", e);
                        continue;
                    }
                };
            }
        });
        self.listening_task = Some(task);

        Ok(())
    }

    async fn process_data(data: &[u8], db: &DatabaseConnection) -> Result<()> {
        let message: Message = serde_json::from_slice::<Message>(data)?;
        match message {
            Message::Text(message) => Self::handle_text_message(message, db).await?,
        }

        Ok(())
    }

    async fn handle_text_message(message: TextMessage, db: &DatabaseConnection) -> Result<()> {
        message::ActiveModel {
            from_uuid: Set(message.from_uuid),
            content_type: Set(entity::content_type::ContentType::Text),
            content: Set(message.text.into_bytes()),
            ..Default::default()
        }
        .save(db)
        .await?;

        //TODO: notify frontend

        Ok(())
    }
}
