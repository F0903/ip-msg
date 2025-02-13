use entity::contact::ActiveModel;
use sea_orm::DeriveIntoActiveModel;
use serde::Deserialize;

use crate::utils::IpAddress;

#[derive(DeriveIntoActiveModel, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub ip_address: IpAddress,
}
