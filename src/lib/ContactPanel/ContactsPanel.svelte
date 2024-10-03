<script lang="ts">
  import AddNewContactElement from "./AddNewContactButton.svelte";
</script>

<aside class="sidepanel">
  <AddNewContactElement />
</aside>

<style>
  :global(.sidepanel > *) {
    margin-bottom: 10px;
  }

  .sidepanel {
    box-sizing: border-box;
    padding: 10px;
    min-width: 75px;
    height: 100vh;
    background-color: var(--secondary-color);
    overflow: scroll;
    overflow-x: hidden;
    scrollbar-width: none;
  }
</style>
