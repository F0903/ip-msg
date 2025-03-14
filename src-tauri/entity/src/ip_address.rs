use sea_orm::{
    ActiveValue, ColumnType, IntoActiveValue, QueryResult, Set, TryGetable, Value,
    prelude::StringLen, sea_query::ValueType,
};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

// A wrapper type for IpAddr that can be serialized to a SeaORM ActiveValue, and used as a custom type in SeaORM entities.
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize)]
pub struct IpAddress {
    ip_address: IpAddr,
}

impl ValueType for IpAddress {
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::N(46))
    }

    fn type_name() -> String {
        stringify!(IpAddress).to_owned()
    }

    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        let ip_addr = <String as ValueType>::try_from(v)?.into();
        Ok(ip_addr)
    }
}

impl TryGetable for IpAddress {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, sea_orm::TryGetError> {
        let ip_addr = <String as TryGetable>::try_get(res, pre, col)?.into();
        Ok(ip_addr)
    }

    fn try_get_by<I: sea_orm::ColIdx>(
        res: &QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let ip_addr = <String as TryGetable>::try_get_by(res, index)?.into();
        Ok(ip_addr)
    }

    fn try_get_by_index(res: &QueryResult, index: usize) -> Result<Self, sea_orm::TryGetError> {
        let ip_addr = <String as TryGetable>::try_get_by_index(res, index)?.into();
        Ok(ip_addr)
    }
}

impl From<IpAddr> for IpAddress {
    fn from(value: IpAddr) -> Self {
        Self { ip_address: value }
    }
}

impl Into<IpAddr> for IpAddress {
    fn into(self) -> IpAddr {
        self.ip_address
    }
}

impl From<SocketAddr> for IpAddress {
    fn from(value: SocketAddr) -> Self {
        IpAddress {
            ip_address: value.ip(),
        }
    }
}

impl Into<Value> for IpAddress {
    fn into(self) -> Value {
        Value::String(Some(Box::new(self.to_string())))
    }
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

impl IntoActiveValue<Self> for IpAddress {
    fn into_active_value(self) -> ActiveValue<Self> {
        Set(self)
    }
}
