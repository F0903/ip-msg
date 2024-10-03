<input type="text" pattern="(\w|\d)+" />

<style>
  input:user-invalid + input::placeholder {
    content: "invalid input!";
    color: rgb(245, 66, 66);
  }

  input:user-invalid {
    border-bottom-color: rgb(245, 66, 66);
  }

  input:user-valid {
    border-bottom-color: var(--tertiary-color);
  }

  input:focus {
    border-bottom: 2px solid var(--tertiary-color);
    outline: none;
  }

  input {
    margin: auto;

    width: 100%;
    min-height: 30px;
    padding: 8px;
    box-sizing: border-box;

    background: var(--primary-color);

    border: none;
    border-radius: 5px;

    color: var(--tertiary-color);

    font-size: 18px;
    font-family: "Roboto Flex Variable", sans-serif;
    font-weight: 400;
  }
</style>
