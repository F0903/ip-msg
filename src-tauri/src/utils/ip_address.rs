use sea_orm::{ActiveValue, IntoActiveValue};
use serde::Deserialize;
use std::net::IpAddr;

// A wrapper type for IpAddr that can be serialized to a SeaORM ActiveValue
pub struct IpAddress {
    ip_address: IpAddr,
}

impl From<String> for IpAddress {
    fn from(value: String) -> Self {
        IpAddress {
            ip_address: value.parse().unwrap(),
        }
    }
}

impl<'a> Deserialize<'a> for IpAddress {
    fn deserialize<D>(deserializer: D) -> Result<IpAddress, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(IpAddress::from(s))
    }
}

impl ToString for IpAddress {
    fn to_string(&self) -> String {
        self.ip_address.to_string()
    }
}

impl Into<String> for IpAddress {
    fn into(self) -> String {
        self.to_string()
    }
}

impl IntoActiveValue<String> for IpAddress {
    fn into_active_value(self) -> ActiveValue<String> {
        ActiveValue::Unchanged(self.into())
    }
}
