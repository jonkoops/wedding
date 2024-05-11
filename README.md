# Jon & Gabriella's wedding website

We're getting married! This is the website for our wedding, built with Rust, as I wanted to learn how to build a web application with Rust.

## Technical choices

The application is built with the [`axum` web framework](https://github.com/tokio-rs/axum#readme) and uses [SQLx](https://github.com/launchbadge/sqlx#readme) in combination with SQLite for the database. The frontend is largely built with static HTML templates powered by [Askama](https://github.com/djc/askama#readme) (a template engine based on [Jinja](https://jinja.palletsprojects.com)). Some JavaScript is sprinkled on top for interactivity, using [Alpine](https://alpinejs.dev/) to keep things as declarative as possible.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/package-manager)

## Setup

### Create the database

First make sure to install the [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#readme) by running the following command:

```sh
cargo install sqlx-cli --no-default-features --features sqlite
```

Then run the following commands to create the database and apply the migrations to create the tables required, this will create the `wedding.db` file in the root of the project. The database is a SQLite database, and can be interacted with with any SQLite client.

```sh
sqlx db create
sqlx migrate run
```

### Install the frontend dependencies

The package manager used for the frontend is `pnpm`, which is version managed by [Corepack](https://nodejs.org/api/corepack.html) (built-in to Node.js). To install the frontend dependencies, first enable `corepack` and then install the dependencies:

```sh
corepack enable
pnpm install
```


## Building

To build the project run the following command to save the database query metadata:

```sh
cargo sqlx prepare
```

> Note: This command will need to be run every time the database schema changes.

Then use regular [Cargo commands](https://doc.rust-lang.org/cargo/commands/build-commands.html) to build and run the project:

```sh
cargo run
```
