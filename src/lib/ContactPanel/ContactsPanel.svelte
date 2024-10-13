<script lang="ts">
  import { onMount } from "svelte";
  import AddNewContactElement from "./AddNewContactButton.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import ContactElement from "./ContactElement.svelte";
  import ContactElementLabel from "./ContactElementLabel.svelte";

  let contacts: Iterable<Contact> = [];

  onMount(async () => {
    // Find a way to only run this once and update as needed.
    contacts = await invoke("get_all_contacts");
  });
</script>

<aside class="sidepanel">
  <AddNewContactElement />
  {#each contacts as contact}
    <ContactElement>
      <ContactElementLabel label={contact.name[0].toUpperCase()} />
    </ContactElement>
  {/each}
</aside>

<style>
  :global(.sidepanel > *) {
    margin-bottom: 10px;
  }

  .sidepanel {
    box-sizing: border-box;
    padding: 10px;
    min-width: 75px;
    height: 100vh;
    background-color: var(--secondary-color);
    overflow: scroll;
    overflow-x: hidden;
    scrollbar-width: none;
  }
</style>
