use axum::{ extract::{ Path, Query }, Extension, Json };

use crate::{
    cache::CacheExt,
    error::{ AppError, AppJson },
    models::part::{ NewPart, Part, PartList, PartQuery },
    repositories::PartRepoExt,
    router::PARTS_TAG,
    services,
};

/// List all available Parts
///
/// Tries to all Parts from the database
#[utoipa::path(get, path = "", responses((status = OK, body = [Part])), tag = PARTS_TAG)]
pub async fn index(
    Query(conditions): Query<PartQuery>,
    Extension(repo): PartRepoExt
) -> Result<AppJson<PartList>, AppError> {
    let parts = services::parts::search(repo.clone(), &conditions).await?;
    Ok(AppJson(parts))
}

/// Create new Part
///
/// Tries to create a new Part in the database
#[utoipa::path(
    post,
    path = "",
    request_body(
        content = String,
        content_type = "application/json",
        description = "New Part Information"
    ),
    responses((status = 201, description = "Part item created successfully", body = Part)),
    tag = PARTS_TAG
)]
pub async fn create(
    Extension(repo): PartRepoExt,
    Json(new_part): Json<NewPart>
) -> Result<AppJson<Part>, AppError> {
    let part = services::parts::create(repo.clone(), &new_part).await?;
    Ok(AppJson(part))
}

/// Get single Part by id
///
/// Tries to get single part id from the database
#[utoipa::path(
    get,
    path = "/{part_id}",
    params(("part_id" = i32, Path, description = "Part Id")),
    responses((status = OK, body = [Part])),
    tag = PARTS_TAG
)]
pub async fn view(
    Path(part_id): Path<i32>,
    Extension(repo): PartRepoExt,
    Extension(cache): CacheExt
) -> Result<AppJson<Part>, AppError> {
    let part = services::parts::view(repo.clone(), cache.clone(), part_id).await?;
    Ok(AppJson(part))
}

/// Search all parts
///
/// Tries to get list of parts by query from the database
#[utoipa::path(
    get,
    path = "/search",
    params(("name" = String, Query, description = "Part Name")),
    responses((status = OK, body = [Part])),
    tag = PARTS_TAG
)]
pub async fn search(
    Query(params): Query<PartQuery>,
    Extension(repo): PartRepoExt
) -> Result<AppJson<PartList>, AppError> {
    let parts = services::parts::search(repo.clone(), &params).await?;
    Ok(AppJson(parts))
}

/// Update existing Part
///
/// Tries to update a Part in the database
#[utoipa::path(
    put,
    path = "",
    request_body(content = Part, content_type = "application/json", description = "Part To Update"),
    responses((status = 200, description = "Part item updated successfully", body = Part)),
    tag = PARTS_TAG
)]
pub async fn update(
    Extension(repo): PartRepoExt,
    Json(part): Json<Part>
) -> Result<AppJson<Part>, AppError> {
    let part = services::parts::update(repo.clone(), &part).await?;
    Ok(AppJson(part))
}

/// Delete existing Part
///
/// Tries to delete a Part from the database
#[utoipa::path(
    delete,
    path = "/{part_id}",
    params(("part_id" = i32, Path, description = "Part Id")),
    responses((status = 200, description = "Part item deleted successfully", body = String)),
    tag = PARTS_TAG
)]
pub async fn delete(
    Path(part_id): Path<i32>,
    Extension(repo): PartRepoExt
) -> Result<(), AppError> {
    services::parts::delete(repo.clone(), part_id).await?;
    Ok(())
}
