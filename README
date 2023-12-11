# Budget Beacon Server

## Starting the database

Make sure you are running docker, then run the following command to start the database

```bash
docker-compose up -d
```

## Run migrations

Create a `.env` file with your Database URL, example:

```
# Postgres
DATABASE_URL=postgres://<username>:<password>@localhost/budget_beacon
```

Install `sqlx-cli` if you haven't:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Finally run the migrations

```bash
sqlx migrate run
```

## Starting the server

For development, install `cargo-watch`

```bash
cargo install cargo-watch
```

To run the server, execute the following command:

```bash
cargo watch -x run
```