<script lang="ts">
  import Fa from "svelte-fa";
  import { faXmark } from "@fortawesome/free-solid-svg-icons";

  export let return_url: string;

  function onClose() {
    window.location.replace(return_url);
  }
</script>

<div class="overlay-container">
  <button class="close-button" on:click={onClose}>
    <Fa icon={faXmark} class="close-button-icon" />
  </button>
  <div class="overlay-content">
    <slot></slot>
  </div>
</div>

<style>
  .overlay-content {
    margin-right: 100px;
  }

  /* Specifying svg here is important so this overwrites the default FA style*/
  :global(svg.close-button-icon) {
    width: 50%;
    height: 50%;
  }

  .close-button {
    position: absolute;
    right: 0px;
    margin: 25px;
    width: 50px;
    height: 50px;

    background: var(--secondary-color);
    border: none;
    border-radius: 50%;

    color: var(--tertiary-color);
    text-align: center;

    cursor: pointer;
  }

  .overlay-container {
    position: fixed;
    width: 100%;
    height: 100%;

    background: var(--primary-color);
  }
</style>
