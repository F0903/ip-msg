use super::{models::ContactForm, CommandResult};
use crate::AppState;
use entity::contact;
use sea_orm::{prelude::*, IntoActiveModel};
use tauri::State;

#[tauri::command]
pub async fn add_contact(state: State<'_, AppState>, contact: ContactForm) -> CommandResult<()> {
    let mut contact = contact.into_active_model();
    contact.set(
        contact::Column::Uuid,
        Value::Uuid(Some(Box::new(Uuid::new_v4()))), // Generate the UUID
    );
    let contact = contact
        .save(&state.db)
        .await
        .map_err(|err| err.to_string())?;

    log::info!("Contact created: {:?}", contact);

    Ok(())
}

#[tauri::command]
pub async fn get_all_contacts(state: State<'_, AppState>) -> CommandResult<Vec<contact::Model>> {
    let contacts = contact::Entity::find()
        .all(&state.db)
        .await
        .map_err(|err| err.to_string())?;
    log::debug!("Getting all contacts: {:?}", contacts);

    Ok(contacts)
}

#[tauri::command]
pub async fn get_self(state: State<'_, AppState>) -> CommandResult<contact::Model> {
    let contact = contact::Entity::find()
        .filter(contact::Column::Name.contains("Self"))
        .one(&state.db)
        .await
        .map_err(|err| err.to_string())?
        .ok_or("Self contact not found (this is not supposed to happen)")?;

    Ok(contact)
}
