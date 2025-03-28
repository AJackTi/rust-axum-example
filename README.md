This is a template for a Rust application using Axum, SQLx and Redis.
# Rust Axum SQLx Redis WS Template


> Reference from [source](https://github.com/softwaremill/rust-axum-sqlx-redis-ws-template)

## Prerequisites
- Rust
- Docker
- Docker Compose
- PostgreSQL
- Redis

# Getting Started
Install the dependencies:

```
cargo clean
```

```
cargo install sqlx-cli --no-default-features --features "rustls,postgres"
```

```
docker-compose up -d
```

```
mkdir migrations
```

```
chmod +x migrate.sh
```

```
./migrate.sh create_cars_table
```

```
./migrate.sh create_parts_table
```

```
sqlx migrate run
```

```
cargo build
```

```
cargo run
```

# Directory Structure
```md
.
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml
├── docs
│   └── postman
│       └── Rust Axum.postman_collection.json
├── migrate.sh
├── migrations
│   ├── 001_create_cars_table.down.sql
│   ├── 001_create_cars_table.up.sql
│   ├── 002_create_parts_table.down.sql
│   └── 002_create_parts_table.up.sql
├── README.md
└── src
    ├── app
    │   └── mod.rs
    ├── build.rs
    ├── cache
    │   └── mod.rs
    ├── config.rs
    ├── controllers
    │   ├── cars.rs
    │   ├── mod.rs
    │   ├── parts.rs
    │   └── utils.rs
    ├── db
    │   ├── mod.rs
    │   ├── postgres
    │   │   └── mod.rs
    │   └── redis
    │       └── mod.rs
    ├── error
    │   └── mod.rs
    ├── main.rs
    ├── models
    │   ├── car.rs
    │   ├── mod.rs
    │   └── part.rs
    ├── repositories
    │   ├── car.rs
    │   ├── mod.rs
    │   └── part.rs
    ├── router
    │   └── mod.rs
    ├── services
    │   ├── cars.rs
    │   ├── mod.rs
    │   └── parts.rs
    └── tests
        ├── fixture
        │   ├── car.rs
        │   ├── mod.rs
        │   └── part.rs
        └── mod.rs
```
