use crate::db::types::IpV4;
use serde::Deserialize;

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct Contact {
    name: String,
    ip: IpV4,
}

impl Contact {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_ip(&self) -> IpV4 {
        self.ip
    }
}
