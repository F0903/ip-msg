use super::CommandResult;
use crate::{
    AppState,
    services::contacts::{AddContactForm, ContactsService},
    utils::MapErrToString,
};
use entity::contact;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn add_contact(
    state: State<'_, AppState>,
    contact_form: AddContactForm,
) -> CommandResult<()> {
    let contacts = state.services.get_service::<ContactsService>();

    let added_contact = contacts
        .insert_contact(contact_form)
        .await
        .map_err_to_string()?;

    log::info!("Contact created: {:?}", added_contact);

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_contacts(state: State<'_, AppState>) -> CommandResult<Vec<contact::Model>> {
    let contacts = state.services.get_service::<ContactsService>();

    let all_contacts = contacts.get_all().await.map_err_to_string()?;
    log::debug!("Got all contacts: {:?}", all_contacts);

    Ok(all_contacts)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_contact_with_id(
    state: State<'_, AppState>,
    id: i32,
) -> CommandResult<Option<contact::Model>> {
    let contacts = state.services.get_service::<ContactsService>();

    let contact = contacts.get_with_id(id).await.map_err_to_string()?;
    log::debug!("Got contact with id: {:?}", contact);

    Ok(contact)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_self(state: State<'_, AppState>) -> CommandResult<contact::Model> {
    let contacts = state.services.get_service::<ContactsService>();

    let contact = contacts.get_self().await.map_err_to_string()?;

    Ok(contact.clone())
}
