# SeaORM Migrations

This directory houses the SeaORM migrations, they can be run as follows:

> **IMPORTANT:** a .env file in the the **src-tauri** directory with a ``DATABASE_URL`` variable must be defined for this to work.

## Via sea-orm-cli

*Requires install via ``cargo install sea-orm-cli``*

- Generate a new migration file

    ```sh
    sea-orm-cli generate MIGRATION_NAME
    ```

- Apply all pending migrations

    ```sh
    sea-orm-cli migrate
    ```

    ```sh
    sea-orm-cli migrate up
    ```

  - Apply first 10 pending migrations

    ```sh
    sea-orm-cli migrate up -n 10
    ```

- Rollback last applied migrations

    ```sh
    sea-orm-cli migrate down
    ```

- Rollback last 10 applied migrations

    ```sh
    sea-orm-cli migrate down -n 10
    ```

- Drop all tables from the database, then reapply all migrations

    ```sh
    sea-orm-cli migrate fresh
    ```

- Rollback all applied migrations, then reapply all migrations

    ```sh
    sea-orm-cli migrate refresh
    ```

- Rollback all applied migrations

    ```sh
    sea-orm-cli migrate reset
    ```

- Check the status of all migrations

    ```sh
    sea-orm-cli migrate status
    ```

## Via Migrator CLI

- Generate a new migration file

    ```sh
    cargo run -- generate MIGRATION_NAME
    ```

- Apply all pending migrations

    ```sh
    cargo run
    ```

    ```sh
    cargo run -- up
    ```

- Apply first 10 pending migrations

    ```sh
    cargo run -- up -n 10
    ```

- Rollback last applied migrations

    ```sh
    cargo run -- down
    ```

- Rollback last 10 applied migrations

    ```sh
    cargo run -- down -n 10
    ```

- Drop all tables from the database, then reapply all migrations

    ```sh
    cargo run -- fresh
    ```

- Rollback all applied migrations, then reapply all migrations

    ```sh
    cargo run -- refresh
    ```

- Rollback all applied migrations

    ```sh
    cargo run -- reset
    ```

- Check the status of all migrations

    ```sh
    cargo run -- status
    ```
