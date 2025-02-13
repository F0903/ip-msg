<script lang="ts">
  import { type IconDefinition } from "@fortawesome/free-solid-svg-icons";
  import { Fa } from "svelte-fa";
  import type { MouseEventHandler } from "svelte/elements";

  let {
    icon,
    text,
    onclick,
  }: {
    icon: IconDefinition;
    text?: string;
    onclick?: MouseEventHandler<HTMLButtonElement>;
  } = $props();
</script>

<button class="icon-button" {onclick}>
  <Fa {icon} style="height: 100%" />
  {#if text}
    <span class="button-text">{text}</span>
  {/if}
</button>

<style>
  .icon-button:hover {
    cursor: pointer;
    filter: brightness(1.5);
  }

  .icon-button {
    aspect-ratio: 1/1;

    height: var(--button-height, 100%);
    background-color: var(--button-background-color, transparent);
    border: none;
    border-radius: var(--button-border-radius, 10px);
    padding: var(--button-padding, 10px);

    color: var(--button-color, var(--tertiary-color));

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
</style>
