use sea_orm_migration::{prelude::*, schema::*};
use uuid::Uuid;

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
                    .col(pk_uuid(Contact::UUID))
                    .col(string_uniq(Contact::Name))
                    .col(string_uniq(Contact::IPAddress))
                    .to_owned(),
            )
            .await?;

        // Add self contact
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Contact::Table)
                    .columns([Contact::UUID, Contact::Name, Contact::IPAddress])
                    .values_panic([Uuid::new_v4().into(), "Self".into(), "127.0.0.1".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
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
    UUID,
    Name,
    IPAddress,
}
