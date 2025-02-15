use entity::{content_type::ContentType, message::ActiveModel};
use sea_orm::DeriveIntoActiveModel;
use serde::Deserialize;
use uuid::Uuid;

#[repr(u8)]
pub enum MessageContentType {
    Text,
    Image,
    Video,
}

#[derive(DeriveIntoActiveModel, Deserialize)]
pub struct SendMessageForm {
    pub to_uuid: Uuid,
    pub content_type: ContentType,
    pub content: Vec<u8>,
}
