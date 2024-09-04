# Svelte Rust Todo App Backend

## Preparation
### Install Rust and Cargo
If you haven't already, you need to install Rust and Cargo. You can do this by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

### Copy the `.env.example` file
Copy the `.env.example` file to `.env` by running the following command:

```bash
cp .env.example .env
```

## Run database migrations
First, you need to run the database on docker, we use postgres as database, to run the database execute the following command:
```bash
docker-compose up -d
```

Before running the database migrations, you need to install the `sqlx-cli` tool, to do this, execute the following command:
```bash
cargo install sqlx-cli
```

Then, you need to run the database migrations, to do this, execute the following command:
```bash
sqlx migrate run
```

## Run the application
To run the application, execute the following command:
```bash
cargo run
```