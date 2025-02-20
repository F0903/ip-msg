use entity::{content_type::ContentType, message::ActiveModel};
use sea_orm::DeriveIntoActiveModel;
use serde::Deserialize;
use uuid::Uuid;

#[derive(DeriveIntoActiveModel, Deserialize)]
pub struct SendMessageForm {
    pub to_uuid: Uuid,
    pub content_type: ContentType,
    pub content: Vec<u8>,
}
