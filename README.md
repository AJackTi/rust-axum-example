```
cargo install sqlx-cli --no-default-features --features "rustls,postgres"
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
