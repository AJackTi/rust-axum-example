use axum::{ extract::{ Path, Query }, Extension, Json };

use crate::{
    cache::CacheExt,
    error::{ AppError, AppJson },
    models::car::{ Car, CarList, CarQuery, NewCar },
    repositories::CarRepoExt,
    router::CARS_TAG,
    services,
};

/// List all available Cars
///
/// Tries to all Cars from the database
#[utoipa::path(get, path = "", responses((status = OK, body = [Car])), tag = CARS_TAG)]
pub async fn list(
    Query(conditions): Query<CarQuery>,
    Extension(repo): CarRepoExt
) -> Result<AppJson<CarList>, AppError> {
    let cars = services::cars::search(repo.clone(), &conditions).await?;
    Ok(AppJson(cars))
}

/// Search all cars
///
/// Tries to get list of cars by query from the database
#[utoipa::path(
    get,
    path = "/search",
    params(("name" = String, Query, description = "Car Name")),
    responses((status = OK, body = [Car])),
    tag = CARS_TAG
)]
pub async fn search(
    Query(params): Query<CarQuery>,
    Extension(repo): CarRepoExt
) -> Result<AppJson<CarList>, AppError> {
    let cars = services::cars::search(repo.clone(), &params).await?;
    Ok(AppJson(cars))
}

/// Get single Car by id
///
/// Tries to get single car by id from the database
#[utoipa::path(
    get,
    path = "/{car_id}",
    params(("car_id" = i32, Path, description = "Car Id")),
    responses((status = OK, body = Car)),
    tag = CARS_TAG
)]
pub async fn view(
    Path(car_id): Path<i32>,
    Extension(repo): CarRepoExt,
    Extension(cache): CacheExt
) -> Result<AppJson<Car>, AppError> {
    let car = services::cars::view(repo.clone(), cache.clone(), car_id).await?;
    Ok(AppJson(car))
}

/// Create new Car
///
/// Tries to create a new Car in the database
#[utoipa::path(
    post,
    path = "",
    request_body(
        content = NewCar,
        content_type = "application/json",
        description = "New Car Information"
    ),
    responses((status = 201, description = "Car item created successfully", body = Car)),
    tag = CARS_TAG
)]
pub async fn create(
    Extension(repo): CarRepoExt,
    Json(new_car): Json<NewCar>
) -> Result<AppJson<Car>, AppError> {
    let car = services::cars::create(repo.clone(), &new_car).await?;
    Ok(AppJson(car))
}

/// Update existing Car
///
/// Tries to update a Car in the database
#[utoipa::path(
    put,
    path = "",
    request_body(content = Car, content_type = "application/json", description = "Car To Update"),
    responses((status = 200, description = "Car item updated successfully", body = Car)),
    tag = CARS_TAG
)]
pub async fn update(
    Extension(repo): CarRepoExt,
    Json(car): Json<Car>
) -> Result<AppJson<Car>, AppError> {
    let car = services::cars::update(repo.clone(), &car).await?;
    Ok(AppJson(car))
}

/// Delete existing Car
///
/// Tries to delete a Car from the database
#[utoipa::path(
    delete,
    path = "/{car_id}",
    params(("car_id" = i32, Path, description = "Car Id")),
    responses((status = 200, description = "Car item deleted successfully", body = String)),
    tag = CARS_TAG
)]
pub async fn delete(
    Path(car_id): Path<i32>,
    Extension(repo): CarRepoExt
) -> Result<AppJson<u64>, AppError> {
    let affected_rows = services::cars::delete(repo.clone(), car_id).await?;
    Ok(AppJson(affected_rows))
}

// Example of end-to-end with real database and repository
#[cfg(test)]
mod tests {}
