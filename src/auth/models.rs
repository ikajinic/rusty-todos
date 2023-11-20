use askama::Template;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;
use serde_json::json;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub name: String,
}

#[derive(Template)]
#[template(path = "auth/register.html")]
pub struct RegisterPage {}

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginPage {}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub enum UserErrors {
    UserAlreadyExists,
    UserNotFound,
    PasswordsDontMatch,
    InvalidCredentials,
}

impl IntoResponse for UserErrors {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            UserErrors::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            UserErrors::UserAlreadyExists => (StatusCode::BAD_REQUEST, "User already exists"),
            UserErrors::PasswordsDontMatch => (StatusCode::BAD_REQUEST, "Check your password"),
            UserErrors::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
