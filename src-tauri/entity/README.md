# SeaORM Entities

This directory houses the SeaORM entities. These are generated via ``sea-orm-cli`` from the migrations.

## Generating entities

> **IMPORTANT:** a .env file in the the **src-tauri** directory with a ``DATABASE_URL`` variable must be defined for this to work.
> I recommend using a simple local SQLite database like so: ``DATABASE_URL=sqlite://db.sqlite?mode=rwc``

To generate entities, use the following command:

``sea-orm-cli generate entity --with-serde both -o entity/src/entities``

This tells SeaORM to generate entities from the migrations with Serde traits, and to output them in the *entity/src/entities* directory (important)
