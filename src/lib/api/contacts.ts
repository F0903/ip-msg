import { invoke } from "@tauri-apps/api/core";
import { Contact, type ContactForm } from "./models/Contact";

export async function addContact(contact_form: ContactForm) {
  await invoke("add_contact", { contact_form: contact_form });
}

export async function getSelfContact(): Promise<Contact> {
  const json = await invoke("get_self");

  const contact = new Contact().deserialize(json as any);
  return contact;
}

export async function getAllContacts(): Promise<Contact[]> {
  const json = await invoke("get_all_contacts");

  const contacts = [];
  for (const contact of json as any) {
    contacts.push(new Contact().deserialize(contact as any));
  }
  return contacts;
}
