<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    on_submit,
    children,
  }: {
    on_submit: (data: any) => void;
    children: Snippet;
  } = $props();

  let form: HTMLFormElement | undefined;

  function onsubmit(event: SubmitEvent) {
    event.preventDefault();

    const formData = new FormData(form);
    const data = Object.fromEntries(formData.entries());

    on_submit(data);
  }

  export function submit() {
    form!.submit();
  }
</script>

<form {onsubmit} bind:this={form}>
  {@render children()}
</form>

<style>
  form {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 20px;

    max-width: var(--form-max-width, 0);
    margin: var(--form-margin, 0);
  }
</style>
