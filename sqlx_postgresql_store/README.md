# SQLx PostgreSQL Store

A concise Rust project demonstrating how to use SQLx with PostgreSQL for async database operations, including setting up, connecting, and executing transaction.

## Quick Overview

- **Dependencies**: `sqlx` for PostgreSQL, `dotenv` for environment management, `tokio` for async runtime.
- **Features**: Creates a `users` table and insert data asynchronously.

## Key Learning

- Integrated SQLx for async DB operations in Rust.
- Utilized transactions for data consistency.
- Practiced schema creation and data insrtion.

## Setup and Run


```sh
$ make db-init && sleep 4 && make db-setup run db-show
docker stop postgresql-container
postgresql-container
docker run --rm -d --name postgresql-container -p 5432:5432 -e POSTGRES_PASSWORD=password postgres
60525ab66c36230f09397044e760c6d53c3adff8281241c467149dd9706d9cdf
PGPASSWORD=password psql -h localhost -U postgres -d postgres < preparation.sql
BEGIN
CREATE TABLE
COMMIT
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/sqlx_postgresql_store`
Inserted user ID: 1
PGPASSWORD=password psql -h localhost -U postgres -d postgres -c "SELECT * FROM users;"
 id | name |      email
----+------+------------------
  1 | hoge | hoge@example.com
(1 row)
```


This project highlights the basics of async database operations with SQLx in Rust, offering a foundation for building robust applications.
