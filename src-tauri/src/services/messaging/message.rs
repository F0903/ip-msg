use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum Message {
    Text(TextMessage),
}

#[derive(Deserialize, Serialize)]
pub struct TextMessage {
    pub from_uuid: Uuid,
    pub text: String,
}
