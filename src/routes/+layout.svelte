<script lang="ts">
  import "@fontsource-variable/roboto-flex";
  import ContactsPanel from "../lib/ContactPanel/ContactsPanel.svelte";
</script>

<div class="container">
  <ContactsPanel />
  <slot></slot>
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
  }

  :global(body) {
    margin: 0;
  }

  :global(:root) {
    font-family: "Roboto Flex Variable", sans-serif;
    font-size: 16px;
    line-height: 24px;

    color: #f6f6f6;
    background-color: #2f2f2f;
  }
</style>
