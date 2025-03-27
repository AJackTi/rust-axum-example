use axum::{ extract::FromRequest, response::IntoResponse, Json };
use hyper::StatusCode;
use serde::Serialize;

// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[derive(Debug)]
#[from_request(via(Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T> where axum::Json<T>: IntoResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

// The kinds of errors we can hit in our application.
#[derive(Debug)]
// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // How we want errors responses to be serialized.
        #[derive(Serialize)]
        struct ErrorMessage {
            message: String,
        }
        let err = self;
        let (status, message) = {
            // Because `TraceLayer` wraps each request in a span that contains the request
            // method, uri, etc we don't need to include those details here
            tracing::error!(%err);

            // Don't expose any details about the error to the client
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".to_owned())
        };

        (status, AppJson(ErrorMessage { message })).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError where E: Into<anyhow::Error> {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: {}", self.0)
    }
}
