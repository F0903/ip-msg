use sea_orm::{prelude::*, ActiveValue, DeriveActiveEnum, EnumIter, IntoActiveValue};
use serde::{Deserialize, Serialize};

#[derive(
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize, Serialize,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case",
    enum_name = "content_type"
)]
pub enum ContentType {
    Text,
    File,
}

impl IntoActiveValue<ContentType> for ContentType {
    fn into_active_value(self) -> ActiveValue<ContentType> {
        ActiveValue::Unchanged(self)
    }
}
