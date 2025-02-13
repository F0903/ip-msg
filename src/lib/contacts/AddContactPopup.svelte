<script lang="ts">
  import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
  import Form from "../Form.svelte";
  import IconButton from "../IconButton.svelte";
  import Input from "../Input.svelte";
  import Overlay from "../Overlay.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { ContactForm } from "$lib/models/Contact";
  import { invalidateAll } from "$app/navigation";

  let addContactPopup: Overlay | undefined;
  let contactForm: Form | undefined;

  export function show() {
    addContactPopup!.show();
  }

  async function on_submit(data: ContactForm) {
    await invoke("add_contact", { contact: data });
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
