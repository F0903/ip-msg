<script lang="ts">
  import IconButton from "$lib/IconButton.svelte";
  import { faArrowUp } from "@fortawesome/free-solid-svg-icons";
</script>

<div class="chatprompt-container">
  <input type="text" class="chatprompt-input" />
  <IconButton
    icon={faArrowUp}
    --button-height="25px"
    --button-color="var(--quaternary-color)"
  />
</div>

<style>
  .chatprompt-container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;

    background-color: var(--tertiary-color);
  }
</style>
