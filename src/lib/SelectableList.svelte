<script lang="ts">
  let {
    options,
    selected = $bindable(undefined),
  }: { options: string[]; selected?: string } = $props();
</script>

<ul>
  {#each options as option}
    <li class:selected={option === selected}>{option}</li>
  {/each}
</ul>

<style></style>
