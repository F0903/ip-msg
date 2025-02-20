import type { Deserializable } from "./utils/Deserializable";

export type ContentType = "Text" | "File";

export type MessageForm = {
  to_uuid: string;
  content_type: ContentType;
  content: Uint8Array;
};

export class Message implements Deserializable<Message> {
  remote_uuid!: string;
  content_type!: ContentType;
  content!: Uint8Array;
  sent_at!: Date;

  deserialize(input: {
    remote_uuid: string;
    content_type: ContentType;
    content: Array<number>;
    sent_at: string;
  }): Message {
    this.remote_uuid = input.remote_uuid;
    this.content_type = input.content_type;
    this.content = new Uint8Array(input.content);
    this.sent_at = new Date(input.sent_at);
    return this;
  }
}
