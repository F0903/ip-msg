import type { Deserializable } from "./utils/Deserializable";

export type ContentType = "Text" | "File";

export type ContactUuidChangedEvent = {
  old_uuid: string;
  new_uuid: string;
};

export type MessageForm = {
  to_uuid: string;
  content_type: ContentType;
  content: Uint8Array;
};

export class Message implements Deserializable<Message> {
  id!: number;
  from_uuid!: string;
  to_uuid!: string;
  content_type!: ContentType;
  content!: Uint8Array;
  received!: boolean;
  sent_at!: Date;

  // The purpose of this is to take in a JSON object with the same fields as this, and initialize all fields from its values.
  deserialize(input: Message): Message {
    this.id = input.id;
    this.from_uuid = input.from_uuid;
    this.to_uuid = input.to_uuid;
    this.content_type = input.content_type;
    this.content = new Uint8Array(input.content);
    this.received = input.received;
    this.sent_at = new Date(input.sent_at);
    return this;
  }
}
