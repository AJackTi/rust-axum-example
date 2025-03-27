use std::sync::Arc;

use redis::AsyncCommands;
use tracing::info;

use crate::{
    cache::CacheImpl,
    models::car::{ Car, CarList, CarQuery, NewCar },
    repositories::car::CarRepository,
};

use anyhow::{ bail, Result };

const CAR_CACHE_TTL: u64 = 60;

pub async fn search<R: CarRepository>(repo: Arc<R>, conditions: &CarQuery) -> Result<CarList> {
    let cars = repo.find_all(conditions).await?;
    Ok(cars)
}

pub async fn view<R: CarRepository>(
    repo: Arc<R>,
    cache: Arc<CacheImpl>,
    car_id: i32
) -> Result<Car> {
    // Construct the cache key
    let cache_key = format!("car:{}", car_id);
    let mut redis_conn = cache.redis_pool.get().await?;
    // Attempt to retrieve the car data from cache
    let maybe_cached: Option<String> = redis_conn.get::<String, _>(cache_key.clone()).await?;

    if let Some(cached_json) = maybe_cached {
        // Deserialize the JSON string back to a Car object
        let cached_car: Car = serde_json::from_str(&cached_json)?;
        info!("Found cached car {}: {:?}", car_id, cached_car);
        return Ok(cached_car);
    }

    info!("Fetching car {} from DB...", car_id);
    // If not in cache, query the database
    let car = repo.find_by_id(car_id).await?;

    // Serialize the Car object to a JSON string
    let car_json = serde_json::to_string(&car)?;

    // Store the serialized Car in the cache
    redis_conn.set_ex::<_, _, ()>(&cache_key, car_json, CAR_CACHE_TTL).await?;

    Ok(car)
}

pub async fn create<R: CarRepository>(repo: Arc<R>, new_car: &NewCar) -> Result<Car> {
    let car = repo.create(new_car).await?;
    Ok(car)
}

pub async fn update<R: CarRepository>(repo: Arc<R>, car: &Car) -> Result<Car> {
    let car = repo.update(car).await?;
    Ok(car)
}

pub async fn delete<R: CarRepository>(repo: Arc<R>, car_id: i32) -> Result<u64> {
    let affected_rows = repo.delete(car_id).await?;
    if affected_rows == 0 {
        bail!("No rows affected, car with ID {} not found", car_id);
    } else if affected_rows > 1 {
        bail!("Unexpected number of rows affected: {}", affected_rows);
    }

    Ok(affected_rows)
}

#[cfg(test)]
mod tests {
    use crate::{ repositories::car::MockCarRepository, tests::fixture::car::cars_fixture };
    use super::*;

    #[tokio::test]
    async fn test_search() {
        let mut mock_repo_impl = MockCarRepository::new();
        mock_repo_impl.expect_find_all().returning(|_| Ok(cars_fixture(5)));
        let conditions = CarQuery {
            name: None,
        };
        let cars = search(Arc::new(mock_repo_impl), &conditions).await.unwrap();
        assert_eq!(cars.len(), 5);
    }
}
