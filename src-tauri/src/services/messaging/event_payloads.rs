use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct ContactUuidChanged {
    pub old_uuid: Uuid,
    pub new_uuid: Uuid,
}
