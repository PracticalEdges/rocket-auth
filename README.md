# Rocket-Auth

A Simple Authentication Server Written in Rust

## Run

Up the database

```shell
cd dev
docker compose up -d
```

build

```shell
cargo build
```

Run

```shell
cargo run
```

## Note

Some extra linux packages are needed for it

```shell
sudo apt-get install libmariadb-dev-compat libmariadb-dev
export MYSQLCLIENT_INCLUDE_DIR=/usr/include/mariadb
export MYSQLCLIENT_LIB_DIR=/usr/lib/x86_64-linux-gnu
export MYSQLCLIENT_VERSION=10.5.9  # Replace with your installed version
```

## Generate migrations

New Table

```shell
diesel migration generate new_table
```

Run

```shell
diesel migration run
```

Note: You have to write sql for migrations which is sequential in nature why bcoz diesel works like that i.e no joins table first then all tables that have kind of joins
