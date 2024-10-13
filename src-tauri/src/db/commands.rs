use crate::db::ContactsSurface;
use crate::models::Contact;
use crate::{AppState, BackendError};
use tauri::State;

type Result<T> = std::result::Result<T, BackendError>;

#[tauri::command]
pub async fn add_contact(state: State<'_, AppState>, contact: Contact) -> Result<bool> {
    println!("add_contact called! Contact is: {:?}", contact);

    let db = state.db.get().await.map_err(|e| BackendError::new(e))?;
    let contacts = ContactsSurface::on(db)
        .await
        .map_err(|e| BackendError::new(e))?;
    contacts
        .write(contact)
        .await
        .map_err(|e| BackendError::new(e))?;

    Ok(true)
}
