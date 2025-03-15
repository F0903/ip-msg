<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    outgoing,
    received,
    children,
  }: { outgoing: boolean; received: boolean; children: Snippet } = $props();
</script>

<div class="chat-bubble-container" class:outgoing>
  <div class="chat-bubble">
    {@render children()}
  </div>
  <div class="received-indicator" class:received></div>
</div>

<style>
  .received-indicator.received {
    background-color: var(--message-received-indicator-color);
  }

  .received-indicator {
    --message-received-indicator-color: hsl(118, 100%, 75%);
    --message-not-received-indicator-color: hsl(0, 100%, 70%);
    width: 0.3em;
    height: 0.3em;
    background-color: var(--message-not-received-indicator-color);
    border-radius: 50%;
  }

  .chat-bubble {
    background-color: var(--tertiary-color);
    color: var(--primary-text-color);
    float: left;

    border-radius: 15px;
    padding: 10px;
    font-size: 1em;
  }

  .chat-bubble-container {
    align-self: flex-start;

    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 10px;
  }

  .chat-bubble-container.outgoing {
    float: right;
    align-self: flex-end;
  }
</style>
