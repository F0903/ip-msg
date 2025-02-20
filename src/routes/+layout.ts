// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)

import type { LayoutLoad } from "./$types";
import type { Contact } from "$lib/api/models/Contact";
import { getAllContacts, getSelfContact } from "$lib/api/contacts";

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async () => {
  const selfContact = await getSelfContact();
  const contacts: Contact[] = await getAllContacts();

  return {
    selfContact,
    contacts,
  };
};
