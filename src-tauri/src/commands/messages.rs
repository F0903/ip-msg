use super::form_models::SendMessageForm;
use crate::{
    AppState, commands::CommandResult, services::messaging::Message, utils::MapErrToString,
};
use chrono::Utc;
use entity::message;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn send_message(
    state: State<'_, AppState>,
    message_form: SendMessageForm,
) -> CommandResult<message::Model> {
    let self_contact = state.contacts.get_self().await.map_err_to_string()?;
    let sent_message = state
        .messages
        .send_message(
            Message {
                remote_uuid: self_contact.uuid,
                content_type: message_form.content_type,
                content: message_form.content,
                sent_at: Utc::now(),
            },
            message_form.to_uuid,
        )
        .await
        .map_err_to_string()?;

    Ok(sent_message)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_correspondence(
    state: State<'_, AppState>,
    to_id: i32,
) -> CommandResult<Vec<message::Model>> {
    let messages = state
        .messages
        .get_correspondence(to_id)
        .await
        .map_err_to_string()?;

    Ok(messages)
}
