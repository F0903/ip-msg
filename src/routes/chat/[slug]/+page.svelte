<script lang="ts">
  import type { Message } from "$lib/api/models/Message.js";
  import ChatBubble from "$lib/chat/ChatBubble.svelte";
  import ChatPrompt from "$lib/chat/ChatPrompt.svelte";

  let { data } = $props();

  let messages: Message[] = $state(data.messages);

  const textDecoder = new TextDecoder();
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
