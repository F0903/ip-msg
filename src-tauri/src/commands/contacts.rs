use super::{CommandResult, form_models::ContactForm};
use crate::{AppState, utils::MapErrToString};
use entity::contact;
use sea_orm::{IntoActiveModel, prelude::*};
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn add_contact(
    state: State<'_, AppState>,
    contact_form: ContactForm,
) -> CommandResult<()> {
    let mut contact = contact_form.into_active_model();
    contact.set(
        contact::Column::Uuid,
        Value::Uuid(Some(Box::new(Uuid::new_v4()))), // Generate the UUID
    );
    let added_contact = state
        .contacts
        .insert_contact(contact)
        .await
        .map_err_to_string()?;

    log::info!("Contact created: {:?}", added_contact);

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_contacts(state: State<'_, AppState>) -> CommandResult<Vec<contact::Model>> {
    let contacts = state.contacts.get_all().await.map_err_to_string()?;
    log::debug!("Got all contacts: {:?}", contacts);

    Ok(contacts)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_contact_with_id(
    state: State<'_, AppState>,
    id: i32,
) -> CommandResult<Option<contact::Model>> {
    let contact = state.contacts.get_with_id(id).await.map_err_to_string()?;
    log::debug!("Got contact with id: {:?}", contact);

    Ok(contact)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_self(state: State<'_, AppState>) -> CommandResult<contact::Model> {
    let contact = state.contacts.get_self().await.map_err_to_string()?;

    Ok(contact.clone())
}
