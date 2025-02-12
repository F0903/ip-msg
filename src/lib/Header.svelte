<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    left,
    center,
    right,
  }: { left?: Snippet; center?: Snippet; right?: Snippet } = $props();
</script>

<header>
  <div class="header-column left">
    {@render left?.()}
  </div>
  <div class="header-column center">
    {@render center?.()}
  </div>
  <div class="header-column right">
    {@render right?.()}
  </div>
</header>

<style>
  .left {
    justify-content: start;
  }

  .center {
    justify-content: center;
  }

  .right {
    justify-content: end;
  }

  .header-column {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  header {
    --header-height: 50px;

    display: grid;
    grid-template-columns: 1fr auto 1fr;
    grid-auto-rows: var(--header-height);

    background-color: var(--secondary-color);

    height: var(--header-height);

    box-sizing: border-box;
  }
</style>
