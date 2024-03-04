# SQLx PostgreSQL Example

This project demonstrates how to use SQLx with PostgreSQL in Rust.
It covers setting up a PostgreSQL database using Docker, connecting to the database, and performing basic operations.

## Command to Run the Project

First, start a PostgreSQL instance using Docker:

```sh
docker run --name postgresql-container -d -p 5432:5432 --rm -e POSTGRES_PASSWORD=password postgres
```

Next, initialize the database with the necessary schema and data:

```sh
PGPASSWORD=password psql -h localhost -U postgres -d postgres < preparation.sql
```

Finally, run the Rust application:

```sh
$ cargo run
```

## Offline build

To build this project in offline mode, you first need to create the database metadata.

1. Install `sqlx-cli` if you haven't already:

```sh
cargo install sqlx-cli
```

2. Generate the database metadata for your workspace:

```sh
cargo sqlx prepare --workspace
```

This step creates metadata in `.sqlx` in your project directory, contains all the necessary schema information for `sqlx` to perform copmile-time verification of SQL queries.

3. Finally build this project in offline mode

```sh
SQLX_OFFLINE=true cargo build --release
```

By setting the SQLX_OFFLINE environment variable to true, you instruct sqlx to use the previously generated schema information for query validation, enabling the build process to proceed without needing access to the actual database.

## What I learned

- Setting up a PostgreSQL database in Docker and configure it for use with Rust.
- Connecting to a PostgreSQL database from Rust using SQLx.
- Mapping database records to Rust structs using SQLx's `query_as` function and `query)as!` macro.
- Handling UUIDs and timestamps in Rust with the `uuid` and `chrono` crates, respectively.
- The importance of enabling specific SQLx features in `Cargo.toml`to work with certain SQL types like UUID and TIMESTAMP.
- How to build SQLx project in offline mode.
