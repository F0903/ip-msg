use serde::Serialize;

#[derive(Serialize)]
pub struct BackendError {
    msg: String,
}

impl BackendError {
    pub fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
