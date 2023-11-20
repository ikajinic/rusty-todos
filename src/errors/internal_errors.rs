use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct ServerError;

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong, please try again later"),
        )
            .into_response()
    }
}
