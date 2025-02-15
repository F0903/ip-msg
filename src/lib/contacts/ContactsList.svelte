<script lang="ts">
  import IconButton from "$lib/IconButton.svelte";
  import PopoutShelf from "$lib/PopoutShelf.svelte";
  import { faAdd, faPerson } from "@fortawesome/free-solid-svg-icons";
  import AddContactPopup from "./AddContactPopup.svelte";
  import type { Contact } from "$lib/models/Contact";
  import { goto } from "$app/navigation";
  import Divider from "$lib/Divider.svelte";

  let { contacts }: { contacts: Contact[] } = $props();

  let addContactPopup: AddContactPopup | undefined;

  function onAddContactClicked() {
    addContactPopup!.show();
  }

  async function onContactClicked(contact: Contact) {
    await goto(`/chat/${contact.uuid}`);
  }
</script>

<AddContactPopup bind:this={addContactPopup} />
<PopoutShelf --shelf-padding="15px 0px 0px 0px">
  {#if contacts.length > 0}
    <div class="contacts-container">
      {#each contacts as contact}
        <IconButton
          icon={faPerson}
          text={contact.name}
          onclick={() => onContactClicked(contact)}
        />
      {/each}
    </div>
    <Divider --divider-width="50%" --divider-color="var(--primary-color)" />
  {/if}

  <IconButton
    icon={faAdd}
    text="Add Contact"
    onclick={onAddContactClicked}
    --button-color="var(--quaternary-color)"
  />
</PopoutShelf>

<style>
  .contacts-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
</style>
