<script lang="ts">
  import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";
  import IconButton from "./IconButton.svelte";

  let {
    off_icon,
    on_icon,
    toggled = $bindable(false),
  }: {
    off_icon: IconDefinition;
    on_icon: IconDefinition;
    toggled: boolean;
  } = $props();

  let icon = $derived(toggled ? on_icon : off_icon);

  async function onclick() {
    toggled = !toggled;
  }
</script>

<IconButton {icon} {onclick} />
