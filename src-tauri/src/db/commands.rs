use crate::backend_error::Result;
use crate::db::ContactsSurface;
use crate::models::Contact;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn add_contact(state: State<'_, AppState>, contact: Contact) -> Result<()> {
    println!("add_contact called! Contact is: {:?}", contact);

    let contacts = ContactsSurface::on(state.db.get().await?).await?;
    let result = contacts.write(contact).await;
    match &result {
        Ok(_) => println!("ok"),
        Err(e) => println!("err: {}", e),
    }
    result?;

    Ok(())
}
