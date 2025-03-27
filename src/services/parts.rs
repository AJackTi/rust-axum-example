use std::sync::Arc;

use bb8_redis::{ bb8::PooledConnection, RedisConnectionManager };
use redis::AsyncCommands;

use crate::{
    cache::CacheImpl,
    models::part::{ NewPart, Part, PartList, PartQuery },
    repositories::part::PartRepository,
};
use anyhow::{ bail, Result };
use tracing::info;

const PART_CACHE_TTL: u64 = 60;

pub async fn search<R: PartRepository>(repo: Arc<R>, conditions: &PartQuery) -> Result<PartList> {
    let parts = repo.find_all(conditions).await?;
    Ok(parts)
}

pub async fn view<R: PartRepository>(
    repo: Arc<R>,
    cache: Arc<CacheImpl>,
    part_id: i32
) -> Result<Part> {
    // Construct the cache key
    let cache_key = format!("part:{}", part_id);
    let mut redis_conn: PooledConnection<RedisConnectionManager> = cache.redis_pool.get().await?;
    // Attempt to retrieve the part data from cache
    let maybe_cached: Option<String> = redis_conn.get::<String, _>(cache_key.clone()).await?;

    if let Some(cached_json) = maybe_cached {
        // Deserialize the JSON string back to a Part object
        let cached_part: Part = serde_json::from_str(&cached_json)?;
        info!("Found cached part {}: {:?}", part_id, cached_part);
        return Ok(cached_part);
    }
    info!("Fetching part {} from DB...", part_id);
    // If not in cache, query the database
    let part = repo.find_by_id(part_id).await?;

    // Serialize the Part object to a JSON string
    let part_json = serde_json::to_string(&part)?;

    // Store the serialized Part in the cache
    redis_conn.set_ex::<_, _, ()>(&cache_key, part_json, PART_CACHE_TTL).await?;

    Ok(part)
}

pub async fn create<R: PartRepository>(repo: Arc<R>, new_part: &NewPart) -> Result<Part> {
    let part = repo.create(new_part).await?;
    Ok(part)
}

pub async fn update<R: PartRepository>(repo: Arc<R>, part: &Part) -> Result<Part> {
    let part = repo.update(part).await?;
    Ok(part)
}

pub async fn delete<R: PartRepository>(repo: Arc<R>, part_id: i32) -> Result<u64> {
    let affected_rows = repo.delete(part_id).await?;
    if affected_rows == 0 {
        bail!("No rows affected, part with ID {} not found", part_id);
    } else if affected_rows > 1 {
        bail!("Unexpected number of rows affected: {}", affected_rows);
    }
    Ok(affected_rows)
}

#[cfg(test)]
mod tests {
    use crate::repositories::part::MockPartRepository;
    use super::*;
    use crate::tests::fixture::part::parts_fixture;

    #[tokio::test]
    async fn test_search() {
        let mut mock_repo_impl = MockPartRepository::new();
        mock_repo_impl.expect_find_all().returning(|_| Ok(parts_fixture(5)));
        let conditions = PartQuery {
            name: None,
        };
        let parts = search(Arc::new(mock_repo_impl), &conditions).await.unwrap();
        assert_eq!(parts.len(), 5);
    }
}
