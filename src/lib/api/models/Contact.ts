import type { Deserializable } from "./utils/Deserializable";

export type ContactForm = {
  name: string;
  ip_address: string;
};

export class Contact implements Deserializable<Contact> {
  uuid!: string;
  name!: string;
  ip_address!: string;

  // The purpose of this is to take in a JSON object with the same fields as this, and initialize all fields from its values.
  deserialize(input: Contact): Contact {
    this.uuid = input.uuid;
    this.name = input.name;
    this.ip_address = input.ip_address;
    return this;
  }
}
