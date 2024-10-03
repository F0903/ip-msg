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
    --primary-color: #2b2b2b;
    --secondary-color: #252525;
    --tertiary-color: #ce6c47; /*#F46036*/

    --primary-text-color: #f6f6f6;

    font-family: "Roboto Flex Variable", sans-serif;
    font-size: 16px;
    line-height: 24px;
    color: var(--primary-text-color);

    background-color: var(--primary-color);
  }
</style>
