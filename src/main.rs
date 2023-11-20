use std::sync::{Arc, Mutex};

use axum::routing::{delete, post, put};
use axum::{routing::get, Router};
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use sqlx::SqlitePool;

use crate::auth::handlers as auth_handlers;
use crate::state::AppState;
use crate::todos::handlers as todo_handlers;

mod auth;
mod errors;
mod migrations;
mod state;
mod todos;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    migrations::create_db().await;

    let pool = SqlitePool::connect(migrations::DB_URL).await.unwrap();

    migrations::migrate(&pool).await;

    let state = AppState {
        random: Arc::new(Mutex::new(ChaCha8Rng::seed_from_u64(OsRng.next_u64()))),
        pool,
    };

    let app = Router::new()
        .route("/", get(todo_handlers::handle_index))
        .route("/register", get(auth_handlers::handle_get_register))
        .route("/register", post(auth_handlers::handle_register))
        .route("/login", get(auth_handlers::handle_get_login))
        .route("/login", post(auth_handlers::handle_login))
        .route("/logout", post(auth_handlers::handle_logout))
        .route("/todo", post(todo_handlers::handle_create))
        .route("/todo/:id", delete(todo_handlers::handle_delete))
        .route("/todo/:id", put(todo_handlers::handle_complete))
        .with_state(state);

    Ok(app.into())
}
