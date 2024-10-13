<script lang="ts">
  export let label: string;
</script>

<div class="container">
  <span class="label">{label}</span>
</div>

<style>
  .label {
    color: var(--primary-text-color);
    text-align: center;
    font-size: 150%;
    user-select: none;
  }

  .container {
    width: 100%;
    height: 100%;
    display: grid;
    place-items: center;
    cursor: pointer;
  }
</style>
