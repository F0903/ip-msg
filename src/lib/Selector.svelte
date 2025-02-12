<script lang="ts">
  let {
    options,
    selected = $bindable(undefined),
  }: { options: string[]; selected?: string } = $props();
</script>

<div class="selector">
  <ul>
    {#each options as option}
      <li class:selected={option === selected}>{option}</li>
    {/each}
  </ul>
</div>

<style></style>
