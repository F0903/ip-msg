<script lang="ts">
  import { Message } from "$lib/api/models/Message.js";
  import ChatBubble from "$lib/chat/ChatBubble.svelte";
  import ChatPrompt from "$lib/chat/ChatPrompt.svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let { data } = $props();

  let messages: Message[] = $state(data.messages);

  const textDecoder = new TextDecoder();

  // We need a variable to keep track of unlisten functions, as we can't return them from onMount due to the callback being async.
  let unlistenEvents: UnlistenFn[] = [];

  onMount(async () => {
    const unlisten = await listen("message-received", (message: any) => {
      const msg = new Message().deserialize(message);
      if (msg.remote_uuid == data.to_uuid) {
        messages.push(msg);
      }
    });

    unlistenEvents.push(unlisten);
  });

  onDestroy(() => {
    for (const unlisten of unlistenEvents) {
      unlisten();
    }
  });
</script>

<main class="chat">
  <div class="top">
    <div class="chat-messages">
      {#each messages as message}
        <ChatBubble outgoing={message.remote_uuid == data.selfContact.uuid}>
          <span>{textDecoder.decode(message.content)}</span>
        </ChatBubble>
      {/each}
    </div>
  </div>
  <div class="bottom">
    <ChatPrompt to_uuid={data.to_uuid} />
  </div>
</main>

<style>
  .chat-messages {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    justify-content: flex-end;
    align-items: flex-start;
    gap: 20px;

    height: 100%;
    max-width: 1000px;
    margin: auto;
  }

  .top {
    flex-grow: 1;
    padding: 25px 50px;
    width: 100%;
  }

  .bottom {
    padding: 10px;
    padding-bottom: 15px;
  }

  .chat {
    display: flex;
    flex-direction: column;
    justify-self: space-between;
    align-items: center;

    width: 100%;

    background-color: var(--primary-color);
  }
</style>
