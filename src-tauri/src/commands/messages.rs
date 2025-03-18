use crate::{
    AppState,
    commands::CommandResult,
    services::messaging::{MessageService, SendMessageForm},
    utils::MapErrToString,
};
use entity::message;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn send_message(
    state: State<'_, AppState>,
    message_form: SendMessageForm,
) -> CommandResult<message::Model> {
    let messages = state.services.get_service::<MessageService>();

    let sent_message = messages
        .send_message(message_form)
        .await
        .map_err_to_string()?;

    Ok(sent_message)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_correspondence(
    state: State<'_, AppState>,
    to_id: i32,
) -> CommandResult<Vec<message::Model>> {
    let messages = state.services.get_service::<MessageService>();

    let correspondence = messages
        .get_correspondence(to_id)
        .await
        .map_err_to_string()?;

    Ok(correspondence)
}
