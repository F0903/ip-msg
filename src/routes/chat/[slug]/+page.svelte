<script lang="ts">
  import { invalidateAll } from "$app/navigation";
  import {
    Message,
    type ContactUuidChangedEvent,
  } from "$lib/api/models/Message.js";
  import ChatBubble from "$lib/chat/ChatBubble.svelte";
  import ChatPrompt from "$lib/chat/ChatPrompt.svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let { data } = $props();

  let messages: Message[] = $derived(data.messages);

  const textDecoder = new TextDecoder();

  // We need a variable to keep track of unlisten functions, as we can't return them from onMount due to the callback being async.
  let unlistenEvents: UnlistenFn[] = [];

  onMount(async () => {
    const unlisten_received = await listen("message-received", (data: any) => {
      const msg = new Message().deserialize(data);
      if (msg.from_uuid == data.to_uuid) {
        messages.push(msg);
      }
    });

    const unlisten_contact_uuid_changed = await listen(
      "contact-uuid-changed",
      async (data: any) => {
        let contactChanges = data as ContactUuidChangedEvent;

        // Invalidate all for now, but later on it would be better to only invalidate the specific contact. (we also need to make sure the contact list buttons update)
        await invalidateAll();
      }
    );

    unlistenEvents.push(unlisten_received, unlisten_contact_uuid_changed);
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
        <ChatBubble outgoing={message.from_uuid === data.selfContact.uuid}>
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
