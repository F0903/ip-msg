<script>
  let { visible = true, children } = $props();
</script>

{#if visible}
  <div class="shelf">
    {@render children()}
  </div>
{/if}

<style>
  .shelf {
    height: 100%;
    width: 100px;

    background-color: var(--secondary-color);

    display: flex;
    flex-direction: column;
    align-items: center;

    padding: var(--shelf-padding);
  }
</style>
