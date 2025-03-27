use std::sync::Arc;

use axum::Extension;

use crate::{ config::Config, db::redis::{ redis_connect, Redis } };

pub type CacheExt = Extension<Arc<CacheImpl>>;

pub async fn create_cache(config: &Config) -> CacheImpl {
    let redis_pool = Arc::new(redis_connect(config).await);
    CacheImpl::new(redis_pool)
}

pub struct CacheImpl {
    pub redis_pool: Redis,
}

impl CacheImpl {
    pub fn new(pool: Redis) -> Self {
        Self { redis_pool: pool }
    }
}
