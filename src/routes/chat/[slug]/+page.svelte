<script lang="ts">
  import { invalidateAll } from "$app/navigation";
  import { Message } from "$lib/api/models/Message.js";
  import ChatBubble from "$lib/chat/ChatBubble.svelte";
  import ChatPrompt from "$lib/chat/ChatPrompt.svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let { data } = $props();

  let toContact = $derived(data.contacts.find((x) => x.id === data.toId))!;
  let messages: Message[] = $derived(data.messages);

  const textDecoder = new TextDecoder();

  // We need a variable to keep track of unlisten functions, as we can't return them from onMount due to the callback being async.
  let unlistenEvents: UnlistenFn[] = [];

  onMount(async () => {
    const unlisten_received = await listen(
      "message-received",
      async (eventData: any) => {
        const msg = new Message().deserialize(eventData);
        if (msg.from_uuid === toContact.uuid) {
          messages.push(msg);
        }

        await invalidateAll();
      }
    );

    const unlisten_contact_changed = await listen(
      "contact-changed",
      async (eventData: any) => {
        //TODO
        //await invalidateAll();
      }
    );

    unlistenEvents.push(unlisten_received, unlisten_contact_changed);
  });

  onDestroy(() => {
    for (const unlisten of unlistenEvents) {
      unlisten();
    }
  });
</script>

<main class="chat">
  <div class="top">
    <div class="chat-messages-container">
      <div class="chat-messages">
        {#each messages as message}
          <ChatBubble
            outgoing={message.from_uuid === data.selfContact.uuid}
            received={message.received}
          >
            <span>{textDecoder.decode(message.content)}</span>
          </ChatBubble>
        {/each}
      </div>
    </div>
  </div>
  <div class="bottom">
    <ChatPrompt to_uuid={toContact.uuid} />
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
    margin: auto;
  }

  .top {
    flex-grow: 1;
    padding: 25px 50px;
    width: 100%;
    overflow-y: scroll;
    scrollbar-color: blue;
  }

  /* width */
  .top::-webkit-scrollbar {
    width: 10px;
  }

  /* Track */
  .top::-webkit-scrollbar-track {
    background: hsl(0, 0%, 15%);
    padding: 5px;
  }

  /* Handle */
  .top::-webkit-scrollbar-thumb {
    background: hsl(0, 0%, 20%);
  }

  /* Handle on hover */
  .top::-webkit-scrollbar-thumb:hover {
    background: var(--quaternary-color);
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
    max-height: 100%;

    background-color: var(--primary-color);
  }
</style>
