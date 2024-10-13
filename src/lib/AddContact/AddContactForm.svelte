<script lang="ts">
  import ContactInput from "$lib/AddContact/ContactInput.svelte";
  import ContactInputSubmitButton from "$lib/AddContact/ContactInputSubmitButton.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let success = false;

  async function add_contact(event: SubmitEvent) {
    event.preventDefault();
    const formData = new FormData(event.target as HTMLFormElement);
    const form = Object.fromEntries(formData.entries());
    success = await invoke("add_contact", { contact: form });
  }
</script>

<div class="form-container">
  <h1 class="page-title">Add a new contact.</h1>
  <form class="add-contact-form" on:submit={add_contact}>
    <ContactInput
      name="name"
      placeholder="Name"
      pattern="(\w|\d)+"
      submit_success={success}
    />
    <ContactInput
      name="ip"
      placeholder="IP Address"
      pattern="^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){'{'}4{'}'}$"
      submit_success={success}
    />
    <ContactInputSubmitButton submit_success={success} />
  </form>
</div>

<style>
  .form-container {
    padding: 50px 0px 0px 10px;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .page-title {
    font-family: var(--primary-font);
    font-weight: 200;
    user-select: none;
  }

  .add-contact-form {
    box-sizing: border-box;
    min-width: 50%;
    height: 100%;
    padding: 25px 50px;

    background-color: var(--primary-color);

    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 25px;
  }
</style>
