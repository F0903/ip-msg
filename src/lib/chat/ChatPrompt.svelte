<script lang="ts">
  import IconButton from "$lib/IconButton.svelte";
  import { faArrowUp } from "@fortawesome/free-solid-svg-icons";
  import { invoke } from "@tauri-apps/api/core";

  let inputElement: HTMLInputElement | undefined;

  function onsubmit(event: SubmitEvent) {
    event.preventDefault();

    const chatInput = inputElement!.value;
    console.log("chat input submit value: ", chatInput);
  }
</script>

<form class="chatprompt-container" {onsubmit}>
  <input
    name="input"
    type="text"
    class="chatprompt-input"
    bind:this={inputElement}
  />
  <div class="button-container">
    <IconButton
      icon={faArrowUp}
      --button-color="var(--quaternary-color)"
      --button-padding="4px"
      --button-height="100%"
      --button-width="100%"
    />
  </div>
</form>

<style>
  .button-container {
    flex-grow: 1;
    padding-right: 5px;
    padding-left: 5px;
  }

  input {
    background-color: var(--chatprompt-color, var(--secondary-color));
    color: var(--primary-text-color);
    border: none;
    border-radius: 15px;
    border-top-right-radius: 0px;
    border-bottom-right-radius: 0px;
  }

  .chatprompt-container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: stretch;
    gap: 5px;

    background-color: var(--chatprompt-color, var(--secondary-color));

    border-radius: 15px;
    padding: 7px;

    min-height: var(--chatprompt-min-height, 40px);
  }
</style>
