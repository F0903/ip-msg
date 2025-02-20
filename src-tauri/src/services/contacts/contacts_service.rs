use std::net::IpAddr;

use entity::{contact, ip_address::IpAddress};
use sea_orm::{DatabaseConnection, prelude::*};
use tokio::sync::OnceCell;

pub struct ContactsService {
    db: DatabaseConnection,
    self_contact: OnceCell<contact::Model>,
}

impl ContactsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            self_contact: OnceCell::new(),
        }
    }

    /// Get the contact of the local user
    pub async fn get_self(&self) -> crate::Result<&contact::Model> {
        // Cache self contact so we don't have to query the db for every message
        let self_contact = self
            .self_contact
            .get_or_init(|| async {
                self.get_with_name("Self")
                    .await
                    .expect("Could not find 'Self' contact in database!")
            })
            .await;

        Ok(self_contact)
    }

    pub async fn get_all(&self) -> crate::Result<Vec<contact::Model>> {
        let contacts = contact::Entity::find().all(&self.db).await?;

        Ok(contacts)
    }

    pub async fn get_with_name(&self, name: impl Into<String>) -> crate::Result<contact::Model> {
        let name = name.into();
        let contact = contact::Entity::find()
            .filter(contact::Column::Name.eq(&name))
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "contact with name '{}' not found",
                name
            )))?;

        Ok(contact)
    }

    pub async fn get_with_ip(&self, ip: IpAddr) -> crate::Result<contact::Model> {
        let contact = contact::Entity::find()
            .filter(contact::Column::IpAddress.eq(Into::<IpAddress>::into(ip)))
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "contact with ip '{}' not found",
                ip
            )))?;

        Ok(contact)
    }

    pub async fn get_with_uuid(&self, uuid: Uuid) -> crate::Result<contact::Model> {
        let contact = contact::Entity::find()
            .filter(contact::Column::Uuid.eq(uuid))
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "contact with uuid '{}' not found",
                uuid
            )))?;

        Ok(contact)
    }

    pub async fn add(&self, contact: contact::ActiveModel) -> crate::Result<contact::ActiveModel> {
        let added_contact = contact.save(&self.db).await?;

        Ok(added_contact)
    }
}
