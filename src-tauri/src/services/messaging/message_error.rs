use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("cannot send message to non-existant contact")]
    SendToNonExistantContact,
}
