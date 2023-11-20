use askama::Template;
use askama_axum::Response;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    response::{Html, IntoResponse},
    Form,
};
use chrono::{Duration, Utc};
use password_auth::{generate_hash, verify_password};
use rand_core::RngCore;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::auth::models::{
    AuthUser, Login, LoginPage, RegisterPage, RegisterUser, User, UserErrors,
};
use crate::errors::internal_errors::ServerError;
use crate::state::AppState;

pub async fn handle_get_register() -> Response {
    Html(RegisterPage {}.render().unwrap()).into_response()
}

pub async fn handle_register(
    State(state): State<AppState>,
    Form(register_user): Form<RegisterUser>,
) -> Response {
    if register_user.password != register_user.confirm_password {
        return UserErrors::PasswordsDontMatch.into_response();
    }

    let existing_user = get_user(&state.pool, &register_user.email).await;
    if existing_user.is_ok() {
        return UserErrors::UserAlreadyExists.into_response();
    }
    if let Err(Error::ServerErr) = existing_user {
        return ServerError.into_response();
    }

    let id = Uuid::new_v4();
    match sqlx::query("INSERT INTO users (id, name, email, password) VALUES (?, ?, ?, ?)")
        .bind(id)
        .bind(&register_user.name)
        .bind(&register_user.email.to_lowercase())
        .bind(generate_hash(&register_user.password))
        .execute(&state.pool)
        .await {
            Ok(_) => {
                let session_token = new_session(state, id).await;
                set_cookie(&session_token)
            }
            Err(_) => return ServerError.into_response()
    }
}

pub async fn handle_get_login() -> Response {
    Html(LoginPage {}.render().unwrap()).into_response()
}

pub async fn handle_login(State(state): State<AppState>, Form(login): Form<Login>) -> Response {
    match get_user(&state.pool, &login.email).await {
        Ok(user) if verify_password(&login.password, &user.password).is_ok() => {
            let session_token = new_session(state, user.id).await;
            set_cookie(&session_token)
        }
        Err(Error::ServerErr) => ServerError.into_response(),
        _ => UserErrors::InvalidCredentials.into_response(),
    }
}

pub async fn handle_logout(State(state): State<AppState>, auth_user: AuthUser) -> Response {
    sqlx::query("DELETE FROM user_sessions WHERE user_id = ?")
        .bind(auth_user.id)
        .execute(&state.pool)
        .await
        .unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("Set-Cookie", "session_token=_; Max-Age=0")
        .header("HX-Redirect", "/login")
        .body(Html("").into_response().into_body())
        .unwrap()
}

fn set_cookie(session_token: &str) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(
            "Set-Cookie",
            format!("session_token={}; Max-Age=999999", session_token),
        )
        .header("HX-Redirect", "/")
        .body(Html("").into_response().into_body())
        .unwrap()
}

async fn get_user(pool: &SqlitePool, email: &str) -> Result<User, Error> {
    return if let Ok(user) =
        sqlx::query_as::<_, User>("SELECT id, name, email, password FROM users WHERE email = ?")
            .bind(&email.to_lowercase())
            .fetch_optional(pool)
            .await
    {
        match user {
            None => Err(Error::UserErr(UserErrors::UserNotFound)),
            Some(u) => Ok(u),
        }
    } else {
        Err(Error::ServerErr)
    };
}

async fn new_session(state: AppState, user_id: Uuid) -> String {
    let mut u128_pool = [0u8; 16];
    state.random.lock().unwrap().fill_bytes(&mut u128_pool);
    let session_token = u128::from_le_bytes(u128_pool);

    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO user_sessions (id, user_id, token, expires_at) VALUES (?, ?, ?, ?)")
        .bind(id)
        .bind(user_id)
        .bind(&session_token.to_le_bytes().to_vec())
        .bind(Utc::now() + Duration::days(1))
        .execute(&state.pool)
        .await
        .unwrap();
    session_token.to_string()
}

enum Error {
    UserErr(UserErrors),
    ServerErr,
}
