use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(pk_auto(Message::Id))
                    .col(uuid(Message::FromUUID))
                    .col(uuid_null(Message::ToUUID).null())
                    .col(string(Message::ContentType))
                    .col(blob(Message::Content))
                    .col(boolean(Message::Received))
                    .col(text_null(Message::Signature))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    FromUUID,
    ToUUID,
    ContentType,
    Content,
    Received,
    Signature,
}
