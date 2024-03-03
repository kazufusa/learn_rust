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

## What I learned

- Setting up a PostgreSQL database in Docker and configure it for use with Rust.
- Connecting to a PostgreSQL database from Rust using SQLx.
- Mapping database records to Rust structs using SQLx's `query_as` function.
- Handling UUIDs and timestamps in Rust with the `uuid` and `chrono` crates, respectively.
- The importance of enabling specific SQLx features in `Cargo.toml`to work with certain SQL types like UUID and TIMESTAMP.
