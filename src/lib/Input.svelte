<script>
  let { name, label, type = "text", placeholder, required } = $props();
</script>

<div class="input-container">
  <label for={name}>{label}</label>
  <input {type} {name} {placeholder} {required} />
</div>

<style>
  input {
    min-height: 25px;
    background-color: var(--secondary-color);
    border: 0;
    border-radius: 10px;
    padding: 10px;

    color: var(--primary-text-color);
  }

  label {
    font-size: 1em;
    color: var(--quaternary-color);
    font-weight: 400;
    padding-left: 5px;
  }

  .input-container {
    display: flex;
    flex-direction: column;
    gap: 5px;
    justify-content: center;
    align-items: start;
  }
</style>
