use askama_axum::Response;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum_extra::extract::cookie::Cookie;
use chrono::{DateTime, Utc};
use shuttle_runtime::async_trait;
use uuid::Uuid;

use crate::auth::models::AuthUser;
use crate::state::AppState;

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session_token = parts
            .headers
            .get_all("Cookie")
            .iter()
            .filter_map(|cookie| {
                cookie
                    .to_str()
                    .ok()
                    .and_then(|cookie| cookie.parse::<Cookie>().ok())
            })
            .find_map(|cookie| {
                (cookie.name() == "session_token").then(move || cookie.value().to_owned())
            })
            .and_then(|cookie_value| cookie_value.parse::<u128>().ok());

        if session_token.is_none() {
            return redirect_to_login();
        }

        let user: Option<(Uuid, String, DateTime<Utc>)> = sqlx::query_as("SELECT u.id, u.name, us.expires_at FROM users u JOIN user_sessions us ON us.user_id = u.id WHERE token = ?")
            .bind(&session_token.unwrap().to_le_bytes().to_vec())
            .fetch_optional(&state.pool)
            .await
            .unwrap();

        if let Some((id, name, expires_at)) = user {
            if (expires_at - Utc::now()).num_seconds() < 0 {
                sqlx::query("DELETE FROM user_sessions WHERE token = ?")
                    .bind(&session_token.unwrap().to_le_bytes().to_vec())
                    .execute(&state.pool)
                    .await
                    .unwrap();
                return redirect_to_login();
            }
            return Ok(AuthUser { id, name });
        }

        return redirect_to_login();
    }
}

fn redirect_to_login() -> Result<AuthUser, Response> {
    Err(Redirect::to("/login").into_response())
}
