<script>
  import ChatPrompt from "$lib/chat/ChatPrompt.svelte";
</script>

<div class="chat">
  <div class="top">
    <div class="chat-messages"></div>
  </div>
  <div class="bottom">
    <ChatPrompt />
  </div>
</div>

<style>
  .top {
    flex-grow: 1;
    padding: 10px;
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
