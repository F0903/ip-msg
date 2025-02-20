use entity::content_type::ContentType;
use sea_orm_migration::{prelude::*, schema::*, sea_orm::ActiveEnum};

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
                    .col(uuid(Message::ToUUID))
                    .col(
                        ColumnDef::new_with_type(ContentType::name(), ContentType::column_type())
                            .not_null()
                            .comment("CHANGE TO ContentType"),
                    )
                    .col(blob(Message::Content))
                    .col(boolean(Message::Received))
                    .col(date_time(Message::SentAt))
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
    Content,
    Received,
    SentAt,
}
