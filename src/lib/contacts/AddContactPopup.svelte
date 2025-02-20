<script lang="ts">
  import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
  import Form from "../Form.svelte";
  import IconButton from "../IconButton.svelte";
  import Input from "../Input.svelte";
  import Overlay from "../Overlay.svelte";
  import type { ContactForm } from "$lib/api/models/Contact";
  import { invalidateAll } from "$app/navigation";
  import { addContact } from "$lib/api/contacts";

  let addContactPopup: Overlay | undefined;
  let contactForm: Form | undefined;

  export function show() {
    addContactPopup!.show();
  }

  async function on_submit(contact: ContactForm) {
    await addContact(contact);
    addContactPopup!.close();
    await invalidateAll(); // We need to invalidate the cache to update the contacts list
  }
</script>

<Overlay bind:this={addContactPopup} title="Add Contact">
  <Form
    bind:this={contactForm}
    {on_submit}
    --form-margin="50px auto"
    --form-max-width="400px"
  >
    <Input
      label="Name"
      name="name"
      placeholder="Enter the contacts name"
      required
    />
    <Input
      label="IP Address"
      name="ip_address"
      placeholder="Enter the contacts IP address"
      required
    />
    <IconButton icon={faArrowRight} --button-height="50px" />
  </Form>
</Overlay>
