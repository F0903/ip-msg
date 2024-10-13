use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Sqlite, Type};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct IpV4(pub Ipv4Addr);

impl Type<Sqlite> for IpV4 {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <String as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &<Sqlite as sqlx::Database>::TypeInfo) -> bool {
        <String as Type<Sqlite>>::compatible(ty)
    }
}

impl<'a> Encode<'a, Sqlite> for IpV4 {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'a>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let ip_str = self.0.to_string();
        <String as Encode<'a, Sqlite>>::encode(ip_str, buf)
    }
}

impl<'a> Decode<'a, Sqlite> for IpV4 {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'a>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let val = <String as Decode<Sqlite>>::decode(value)?;
        let ip = val.parse()?;
        Ok(Self(ip))
    }
}

impl From<Ipv4Addr> for IpV4 {
    fn from(value: Ipv4Addr) -> Self {
        Self(value)
    }
}
