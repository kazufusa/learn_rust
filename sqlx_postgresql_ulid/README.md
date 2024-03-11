# Rust with PostgreSQL: Custom User ID Domain

This project is part of `learn_rust` repository, showcasing the use of Rust with PostgreSQL to handle ULID based user IDs. It leverages SQLx for database interaction, emphasizing type safety with custm domain type.

## Technologies

- Rust
- SQLx
- PostgreSQL
- ULID for unique identifiers

## Highlights

- Defining and using PostgreSQL custom domain types for usre IDs.
- Integrating ULID with Rust and PostgreSQL for sortable, unique identitfiers
- Implementing type conversions betweeb Rust and PostgreSQL, ensuring seemless integration.

## Running the Project

First, start a PostgreSQL instance using Docker:

```sh
cargo make DB
```

Next, configure `.env` with your database URL


```.env
DATABASE_URL=postgres://postgres:password@localhost:5432/postgres
```

Finally, run the Rust application:

```sh
$ cargo run
```

## Conclusion

This project deepens understanding of Rust's database capabilities, particularly for projects requiring custom types and advanced identifier schemes like ULID with PostgreSQL.
