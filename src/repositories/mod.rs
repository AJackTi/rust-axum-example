use std::sync::Arc;

use axum::Extension;
use car::CarRepositoryImpl;
use part::PartRepositoryImpl;

use crate::{ config::Config, db::postgres };

pub mod car;
pub mod part;

pub type CarRepoExt = Extension<Arc<CarRepositoryImpl>>;
pub type PartRepoExt = Extension<Arc<PartRepositoryImpl>>;

pub async fn run_migrations(config: &Config) {
    let db_pool = Arc::new(postgres::db_connect(config).await);
    if let Err(e) = sqlx::migrate!().run(&*db_pool).await {
        panic!("Failed to run migrations: {}", e);
    }
}

pub async fn create_car_repository(config: &Config) -> CarRepositoryImpl {
    let db_pool = Arc::new(postgres::db_connect(config).await);
    CarRepositoryImpl::new(db_pool.clone())
}

pub async fn create_part_repository(config: &Config) -> PartRepositoryImpl {
    let db_pool = Arc::new(postgres::db_connect(config).await);
    PartRepositoryImpl::new(db_pool.clone())
}

#[cfg(test)]
pub async fn clear_database(config: &Config) {
    let db_pool = Arc::new(postgres::db_connect(config).await);
    sqlx::query("TRUNCATE TABLE parts, card CASCADE;")
        .execute(&*db_pool).await
        .expect("Failed to clear database tables");
}
