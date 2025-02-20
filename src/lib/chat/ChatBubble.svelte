<script lang="ts">
  import type { Snippet } from "svelte";

  let { outgoing, children }: { outgoing: boolean; children: Snippet } =
    $props();
</script>

<div class="chat-bubble" class:outgoing>
  {@render children()}
</div>

<style>
  .chat-bubble.outgoing {
    float: right;
    align-self: flex-end;
  }

  .chat-bubble {
    background-color: var(--tertiary-color);
    color: var(--primary-text-color);
    float: left;
    align-self: flex-start;

    border-radius: 15px;
    padding: 10px;
    font-size: 1em;
  }
</style>
