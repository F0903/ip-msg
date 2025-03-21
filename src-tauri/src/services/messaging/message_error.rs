use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("cannot send message to non-existant contact")]
    SendToNonExistantContact,
    #[error("requested contact not found")]
    ContactNotFound,
    #[error("the message was sent with no recipient")]
    NoRecipient,
}
