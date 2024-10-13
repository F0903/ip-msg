<script lang="ts">
  import Fa from "svelte-fa";
  import {
    faArrowRight,
    faCheck,
    faXmark,
  } from "@fortawesome/free-solid-svg-icons";

  export let submit_state: "success" | "failure" | "unsubmitted" =
    "unsubmitted";
</script>

<button
  class="contact-input-submit"
  class:success={submit_state === "success"}
  class:failure={submit_state === "failure"}
  type="submit"
>
  {#if submit_state === "success"}
    <Fa
      icon={faCheck}
      class="input-submit-icon"
      style="width: 50%; height: 50%;"
    />
  {:else if submit_state === "failure"}
    <Fa
      icon={faXmark}
      class="input-submit-icon"
      style="width: 50%; height: 50%;"
    />
  {:else}
    <Fa
      icon={faArrowRight}
      class="input-submit-icon"
      style="width: 50%; height: 50%;"
    />
  {/if}
</button>

<style>
  .contact-input-submit.failure {
    color: rgb(245, 66, 66);
    pointer-events: none;
  }

  .contact-input-submit.success {
    color: rgb(64, 206, 64);
    pointer-events: none;
  }

  .contact-input-submit {
    background-color: var(--secondary-color);
    color: var(--tertiary-color);

    border: none;
    border: 5px;
    border-radius: 50%;

    width: 65px;
    aspect-ratio: 1;

    cursor: pointer;
  }
</style>
