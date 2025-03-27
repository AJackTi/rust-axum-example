use std::sync::Arc;

use dotenv::dotenv;
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };

use crate::config::Config;

pub type Db = Arc<Pool<Postgres>>;

pub async fn db_connect(config: &Config) -> Pool<Postgres> {
    dotenv().ok();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str()).await
        .expect("Error connecting to database")
}
