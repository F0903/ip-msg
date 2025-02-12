<script lang="ts">
  import { faClose } from "@fortawesome/free-solid-svg-icons";
  import Header from "./Header.svelte";
  import IconButton from "./IconButton.svelte";

  let { title, children } = $props();

  let self: HTMLDialogElement | undefined = $state(undefined);

  export function show() {
    self!.show();
  }

  export function close() {
    self!.close();
  }
</script>

<dialog class="overlay" bind:this={self}>
  <Header>
    {#snippet center()}
      <h2 class="overlay-title">{title}</h2>
    {/snippet}
    {#snippet right()}
      <IconButton icon={faClose} onclick={close} />
    {/snippet}
  </Header>
  {@render children()}
</dialog>

<style>
  .overlay-title {
    color: var(--secondary-text-color);

    font-weight: 400;
  }

  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;

    background-color: var(--primary-color);

    box-shadow: none;
    border: none;
  }
</style>
