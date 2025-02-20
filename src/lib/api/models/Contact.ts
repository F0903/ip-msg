import type { Deserializable } from "./utils/Deserializable";

export type ContactForm = {
  name: string;
  ip_address: string;
};

export class Contact implements Deserializable<Contact> {
  uuid!: string;
  name!: string;
  ip_address!: string;

  deserialize(input: {
    uuid: string;
    name: string;
    ip_address: string;
  }): Contact {
    this.uuid = input.uuid;
    this.name = input.name;
    this.ip_address = input.ip_address;
    return this;
  }
}
