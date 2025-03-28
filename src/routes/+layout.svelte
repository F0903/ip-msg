<script lang="ts">
  import { faArrowLeft, faBars } from "@fortawesome/free-solid-svg-icons";
  import Header from "../lib/Header.svelte";
  import ToggleIconButton from "$lib/ToggleIconButton.svelte";
  import ContactsList from "$lib/contacts/ContactsList.svelte";

  let { data, children } = $props();

  let menuToggled = $state(true);
</script>

<div class="container">
  <Header>
    {#snippet left()}
      <ToggleIconButton
        off_icon={faBars}
        on_icon={faArrowLeft}
        bind:toggled={menuToggled}
        --button-height="100%"
        --button-aspect-ratio="1/1"
      />
    {/snippet}
    {#snippet center()}
      <h1 class="header-title">ip-msg</h1>
    {/snippet}
  </Header>

  <div class="content">
    {#if menuToggled}
      <ContactsList contacts={data.contacts} />
    {/if}

    {@render children()}
  </div>
</div>

<style>
  .content {
    display: flex;
    flex-direction: row;
    height: 100%;
  }

  .container {
    display: grid;
    grid-template-rows: auto calc(100vh - var(--header-height));

    height: 100vh;
  }

  .header-title {
    text-align: center;
    color: var(--quaternary-color);

    font-size: 150%;
    font-family: "Roboto Flex Variable", sans-serif;
    font-weight: 300;
  }

  :root {
    --primary-color: hsl(0, 0%, 13%);
    --secondary-color: hsl(0, 0%, 16%);
    --tertiary-color: hsl(0, 0%, 30%);
    --quaternary-color: hsl(30, 84%, 65%);

    --primary-text-color: hsl(0, 0%, 65%);
    --secondary-text-color: hsl(30, 40%, 81%);

    --header-height: 50px;

    background-color: var(--primary-color);
    color: var(--primary-text-color);

    font-family: "Roboto Flex Variable", sans-serif;
    font-weight: 200;
  }

  :global(*, *::before, *::after) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  /* roboto-flex-latin-wght-normal */
  @font-face {
    font-family: "Roboto Flex Variable";
    font-style: oblique 0deg 10deg;
    font-display: auto;
    font-weight: 100 1000;
    font-stretch: 25% 151%;
    src: url(@fontsource-variable/roboto-flex/files/roboto-flex-latin-full-normal.woff2)
      format("woff2-variations");
    unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA,
      U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122, U+2191,
      U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
  }

  /* roboto-flex-latin-ext-wght-normal */
  @font-face {
    font-family: "Roboto Flex Variable";
    font-style: oblique 0deg 10deg;
    font-display: auto;
    font-weight: 100 1000;
    font-stretch: 25% 151%;
    src: url(@fontsource-variable/roboto-flex/files/roboto-flex-latin-ext-full-normal.woff2)
      format("woff2-variations");
    unicode-range: U+0100-02BA, U+02BD-02C5, U+02C7-02CC, U+02CE-02D7,
      U+02DD-02FF, U+0304, U+0308, U+0329, U+1D00-1DBF, U+1E00-1E9F, U+1EF2-1EFF,
      U+2020, U+20A0-20AB, U+20AD-20C0, U+2113, U+2C60-2C7F, U+A720-A7FF;
  }
</style>
