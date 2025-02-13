<script lang="ts">
  import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
  import Form from "./Form.svelte";
  import IconButton from "./IconButton.svelte";
  import Input from "./Input.svelte";
  import Overlay from "./Overlay.svelte";

  let addContactPopup: Overlay | undefined;
  let contactForm: Form | undefined;

  export function show() {
    addContactPopup!.show();
  }

  function on_submit(data: any) {}
</script>

<Overlay bind:this={addContactPopup} title="Add Contact">
  <Form
    bind:this={contactForm}
    {on_submit}
    --form-margin="50px auto"
    --form-max-width="400px"
  >
    <Input name="Name" placeholder="Enter the contacts name" required />
    <Input
      name="IP Address"
      placeholder="Enter the contacts IP address"
      required
    />
    <IconButton
      icon={faArrowRight}
      onclick={() => contactForm!.submit()}
      --button-height="50px"
    />
  </Form>
</Overlay>
