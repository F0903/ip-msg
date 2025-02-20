import { invoke } from "@tauri-apps/api/core";
import { Message, type MessageForm } from "./models/Message";

export async function sendMessage(message_form: MessageForm) {
  await invoke("send_message", {
    message_form: message_form,
  });
}

export async function getCorrespondence(to_uuid: string): Promise<Message[]> {
  const json = await invoke("get_correspondence", {
    to_uuid: to_uuid,
  });

  const messages = [];
  for (const message of json as any) {
    messages.push(new Message().deserialize(message as any));
  }
  return messages;
}
