<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="container">
  <h1>Select a contact in the sidebar to start chatting.</h1>
</div>

<style>
  .container {
    padding: 40px 25px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
  }

  h1 {
    font-weight: 150;
    text-align: center;
    color: #9e9e9e;
  }
</style>
