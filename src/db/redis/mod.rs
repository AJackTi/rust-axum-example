use std::sync::Arc;

use bb8_redis::{ bb8::Pool, RedisConnectionManager };
use dotenv::dotenv;

use crate::config::Config;

pub type Redis = Arc<Pool<RedisConnectionManager>>;

pub async fn redis_connect(config: &Config) -> Pool<RedisConnectionManager> {
    dotenv().ok();
    let manager = RedisConnectionManager::new(config.cache_url.as_str()).unwrap();
    Pool::builder().build(manager).await.unwrap()
}
