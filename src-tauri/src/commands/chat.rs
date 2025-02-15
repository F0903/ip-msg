use super::models::SendMessageForm;
use crate::{commands::CommandResult, AppState};
use tauri::State;

#[tauri::command]
pub async fn send_message(
    state: State<'_, AppState>,
    message: SendMessageForm,
) -> CommandResult<()> {
    Ok(())
}
