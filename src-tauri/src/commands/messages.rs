use super::form_models::SendMessageForm;
use crate::{
    AppState, commands::CommandResult, services::messaging::Message, utils::MapErrToString,
};
use chrono::Utc;
use tauri::State;
use uuid::Uuid;

#[tauri::command(rename_all = "snake_case")]
pub async fn send_message(
    state: State<'_, AppState>,
    message_form: SendMessageForm,
) -> CommandResult<()> {
    let self_contact = state.contacts.get_self().await.map_err_to_string()?;
    state
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

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_correspondence(
    state: State<'_, AppState>,
    to_uuid: Uuid,
) -> CommandResult<Vec<Message>> {
    let messages = state
        .messages
        .get_correspondence(to_uuid)
        .await
        .map_err_to_string()?;

    Ok(messages)
}
