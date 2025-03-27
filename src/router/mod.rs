use axum::Router;
use utoipa::{ OpenApi };
use utoipa_axum::{ router::OpenApiRouter, routes };

use crate::controllers::{ cars::{ self }, parts, utils::{ self } };

pub const CARS_TAG: &str = "Cars";
pub const PARTS_TAG: &str = "Parts";

#[derive(OpenApi)]
#[openapi(
    paths(
        // Include all your controller endpoints here
        crate::controllers::cars::list,
        crate::controllers::cars::search,
        crate::controllers::cars::create,
        crate::controllers::cars::view,
        crate::controllers::cars::update,
        crate::controllers::cars::delete,
        crate::controllers::parts::index,
        crate::controllers::parts::search,
        crate::controllers::parts::create,
        crate::controllers::parts::view,
        crate::controllers::parts::update,
        crate::controllers::parts::delete,
        crate::controllers::utils::healthcheck
    ),
    components(
        schemas(
            // Include your model schemas here
            crate::models::car::Car,
            crate::models::car::NewCar,
            crate::models::car::CarList
            // Add other models as needed
        )
    ),
    tags(
        (name = CARS_TAG, description = "Cars management API"),
        (name = PARTS_TAG, description = "Parts management API")
    )
)]
struct ApiDoc;

pub fn router() -> Router {
    let app = OpenApiRouter::new()
        .routes(routes!(utils::healthcheck))
        .nest("/cars", car_routes())
        .nest("/parts", part_routes());

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", app)
        .split_for_parts();

    // Convert to axum Router
    let axum_router: Router = router.into();

    // Add a route to serve the OpenAPI document
    let axum_router = axum_router.route(
        "/api-docs/openapi.json",
        axum::routing::get(|| async move { axum::Json(api.clone()) })
    );

    // Let's try a simpler approach - just return the router we have so far
    // and add the documentation UIs later when we figure out the correct approach
    axum_router
}

fn car_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(cars::list))
        .routes(routes!(cars::search))
        .routes(routes!(cars::create))
        .routes(routes!(cars::view))
        .routes(routes!(cars::update))
        .routes(routes!(cars::delete))
}

fn part_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(parts::index))
        .routes(routes!(parts::search))
        .routes(routes!(parts::create))
        .routes(routes!(parts::view))
        .routes(routes!(parts::update))
        .routes(routes!(parts::delete))
}
