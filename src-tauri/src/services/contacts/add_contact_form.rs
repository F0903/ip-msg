use entity::{contact::ActiveModel, ip_address::IpAddress};
use sea_orm::DeriveIntoActiveModel;
use serde::Deserialize;

#[derive(DeriveIntoActiveModel, Deserialize)]
pub struct AddContactForm {
    pub name: String,
    pub ip_address: IpAddress,
}
