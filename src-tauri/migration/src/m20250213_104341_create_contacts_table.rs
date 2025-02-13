use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    .col(pk_auto(Contact::Id))
                    .col(string_uniq(Contact::Name))
                    .col(string_uniq(Contact::IPAddress))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Contact {
    Table,
    Id,
    Name,
    IPAddress,
}
