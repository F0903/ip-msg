//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub from_uuid: Uuid,
    pub to_uuid: Option<Uuid>,
    pub content_type: String,
    #[sea_orm(column_type = "Blob")]
    pub content: Vec<u8>,
    pub received: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub signature: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
