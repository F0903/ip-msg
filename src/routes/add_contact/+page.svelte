<script lang="ts">
  import AddContactForm from "$lib/AddContact/AddContactForm.svelte";
  import Overlay from "$lib/Overlay.svelte";
</script>

<Overlay return_url="/">
  <AddContactForm />
</Overlay>
