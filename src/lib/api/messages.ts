import { invoke } from "@tauri-apps/api/core";
import { Message, type MessageForm } from "./models/Message";

export async function sendMessage(messageForm: MessageForm) {
  await invoke("send_message", {
    message_form: messageForm,
  });
}

export async function getCorrespondence(toId: number): Promise<Message[]> {
  const json = await invoke("get_correspondence", {
    to_id: toId,
  });

  const messages = [];
  for (const message of json as any) {
    messages.push(new Message().deserialize(message as any));
  }
  return messages;
}
