pub use sea_orm_migration::prelude::*;

mod m20250213_104341_create_contacts_table;
mod m20250213_104356_create_messages_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250213_104341_create_contacts_table::Migration),
            Box::new(m20250213_104356_create_messages_table::Migration),
        ]
    }
}
