// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)

import { invoke } from "@tauri-apps/api/core";
import type { LayoutLoad } from "./$types";
import type { Contact } from "$lib/models/Contact";

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async () => {
  const contacts: Contact[] = await invoke("get_all_contacts");

  return {
    contacts,
  };
};
