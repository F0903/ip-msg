<script lang="ts">
  export let name: string;
  export let placeholder: string;
  export let pattern: string;
  export let autocomplete: "on" | "off" | string = "off";

  export let submit_state: "success" | "failure" | "unsubmitted" =
    "unsubmitted";
</script>

<input
  {name}
  class="contact-input"
  class:success={submit_state === "success"}
  class:failure={submit_state === "failure"}
  type="text"
  maxlength="30"
  {autocomplete}
  {pattern}
  {placeholder}
/>

<style>
  .contact-input {
    min-width: 50px;
    width: max-content;
    min-height: 30px;
    padding: 8px;
    box-sizing: border-box;

    background: var(--secondary-color);

    --border-width: 2px;
    margin-bottom: var(--border-width);
    border: none;
    border-radius: 5px;

    color: var(--primary-text-color);

    font-size: 18px;
    font-family: "Roboto Flex Variable", sans-serif;
    font-weight: 400;
  }

  .contact-input:focus,
  .contact-input:not(:placeholder-shown) {
    outline: none;
    border-bottom: var(--border-width) solid var(--tertiary-color);
    margin-bottom: 0;
  }

  .contact-input:valid {
    border-bottom-color: var(--tertiary-color);
  }

  .contact-input.failure,
  .contact-input:invalid {
    color: rgb(245, 66, 66);
    border-bottom-color: rgb(245, 66, 66);
    border-bottom-style: dashed;
  }

  .contact-input.success {
    border-color: rgb(64, 206, 64);
    pointer-events: none;
    user-select: none;
  }
</style>
