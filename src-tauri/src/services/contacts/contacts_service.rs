use entity::{contact, ip_address::IpAddress};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    DatabaseConnection, IntoActiveModel, TryIntoModel,
    prelude::*,
};
use std::net::IpAddr;
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
                    .expect("Error occurred in 'get_self'")
                    .ok_or("Could not find 'Self' contact in database!")
                    .unwrap()
            })
            .await;

        Ok(self_contact)
    }

    pub async fn get_all(&self) -> crate::Result<Vec<contact::Model>> {
        let contacts = contact::Entity::find().all(&self.db).await?;

        Ok(contacts)
    }

    pub async fn get_with_name(
        &self,
        name: impl Into<String>,
    ) -> crate::Result<Option<contact::Model>> {
        let name = name.into();
        let contact = contact::Entity::find()
            .filter(contact::Column::Name.eq(&name))
            .one(&self.db)
            .await?;

        Ok(contact)
    }

    pub async fn get_with_ip(&self, ip: IpAddr) -> crate::Result<Option<contact::Model>> {
        let contact = contact::Entity::find()
            .filter(contact::Column::IpAddress.eq(Into::<IpAddress>::into(ip)))
            .one(&self.db)
            .await?;

        Ok(contact)
    }

    pub async fn get_or_create_with_ip(
        &self,
        ip: IpAddr,
        with_uuid: Option<Uuid>,
    ) -> crate::Result<contact::Model> {
        let contact = self.get_with_ip(ip).await?;

        match contact {
            Some(contact) => Ok(contact),
            None => {
                log::info!("Contact with ip '{}' not found, creating...", ip);
                let added_contact = self
                    .insert_contact(contact::ActiveModel {
                        id: NotSet,
                        uuid: Set(with_uuid.unwrap_or_else(Uuid::new_v4)),
                        name: Set(ip.to_string()),
                        ip_address: Set(ip.into()),
                    })
                    .await?;

                Ok(added_contact.try_into_model()?)
            }
        }
    }

    pub async fn get_with_uuid(&self, uuid: Uuid) -> crate::Result<Option<contact::Model>> {
        let contact = contact::Entity::find()
            .filter(contact::Column::Uuid.eq(uuid))
            .one(&self.db)
            .await?;

        Ok(contact)
    }

    pub async fn get_with_id(&self, id: i32) -> crate::Result<Option<contact::Model>> {
        let contact = contact::Entity::find_by_id(id).one(&self.db).await?;

        Ok(contact)
    }

    pub async fn update_contact(
        &self,
        contact: impl IntoActiveModel<contact::ActiveModel>,
    ) -> crate::Result<contact::ActiveModel> {
        let contact = contact.into_active_model();
        log::info!("Updating contact: {:?}", contact);
        let updated_contact = contact.save(&self.db).await?;

        Ok(updated_contact)
    }

    pub async fn insert_contact(
        &self,
        contact: impl IntoActiveModel<contact::ActiveModel>,
    ) -> crate::Result<contact::Model> {
        let contact = contact.into_active_model();
        log::info!("Inserting contact: {:?}", contact);
        let added_contact = contact.insert(&self.db).await?;

        Ok(added_contact)
    }
}
