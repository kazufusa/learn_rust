# SQLx Migration Project

## Project Overview

This project demonstrate how to use SQLx for database operations in Rust, focusing on running migrations. SQLx is a powerful, async, pure Rust database library. This example SQLx to manage database schemas through migrations, which is essential for maintaining and updating database structure in production environment.

## Objectives

- To learn how to configure and use SQLx in a Rust project.
- To implement database migration using SQLx.
- To understand the process of managing database versions in a Rust appllication.

## How to Run Migrations

Ensure you have Rust and SQLx CLI installed on your system. Then follow these steps to run the migrations:

1. Set up your database and ensure it's running. (Provide specific about the database setup if necessary)
2. Update the .env file with your database connection detail.
3. Run the following command in the project directory to apply migrations to your database.

```sh
sqlx migrate run
```
