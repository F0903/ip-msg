use chrono::{DateTime, Utc};
use entity::{content_type::ContentType, message};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// The object that represents messages that are both sent and received over the network
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub remote_uuid: Uuid,
    pub content_type: ContentType,
    pub content: Vec<u8>,
    pub sent_at: DateTime<Utc>,
}

impl From<message::Model> for Message {
    fn from(value: message::Model) -> Self {
        Self {
            remote_uuid: value.from_uuid,
            content_type: value.content_type,
            content: value.content,
            sent_at: value.sent_at.and_utc(),
        }
    }
}

impl From<&message::Model> for Message {
    fn from(value: &message::Model) -> Self {
        Self {
            remote_uuid: value.from_uuid,
            content_type: value.content_type.clone(),
            content: value.content.clone(),
            sent_at: value.sent_at.and_utc(),
        }
    }
}
