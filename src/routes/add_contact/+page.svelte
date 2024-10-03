<script lang="ts">
  import ContactInput from "$lib/ContactPanel/ContactInput.svelte";
  import Overlay from "$lib/Overlay.svelte";
</script>

<Overlay return_url="/">
  <form action="" class="add-contact-form">
    <ContactInput />
  </form>
</Overlay>

<style>
  .add-contact-form {
    min-width: 75px;
    min-height: 50px;
    margin: 75px;
    padding: 25px 50px;

    border-radius: 10px;

    background-color: var(--secondary-color);

    display: flex;
    flex-direction: column;
  }
</style>
