mod contacts;
mod form_models;
mod messages;

pub type CommandResult<T> = std::result::Result<T, String>;

pub fn get_handler<R: tauri::Runtime>()
-> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        contacts::add_contact,
        contacts::get_all_contacts,
        contacts::get_self,
        messages::send_message,
        messages::get_correspondence
    ]
}
