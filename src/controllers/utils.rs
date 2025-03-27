use crate::error::{ AppError, AppJson };

/// Healthcheck endpoint
///
/// Endpoint for k8s healthcheck functionality
#[utoipa::path(get, path = "/healthcheck", responses((status = OK, body = String)), tag = "Utils")]
pub async fn healthcheck() -> Result<AppJson<String>, AppError> {
    Ok(AppJson("ok".into()))
}
