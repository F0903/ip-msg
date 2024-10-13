use crate::backend_error::Result;
use crate::db::ContactsSurface;
use crate::models::Contact;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn add_contact(state: State<'_, AppState>, contact: Contact) -> Result<()> {
    println!("add_contact called! Contact is: {:?}", contact);

    let contacts = ContactsSurface::on(state.db.get().await?).await?;
    contacts.write(contact).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_all_contacts(state: State<'_, AppState>) -> Result<Vec<Contact>> {
    println!("get_all_contacts called!");

    let contacts = ContactsSurface::on(state.db.get().await?).await?;
    let contacts_list = contacts.get_all().await?;

    Ok(contacts_list)
}
