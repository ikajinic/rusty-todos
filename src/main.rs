use std::net::SocketAddr;

use axum::{Router, routing::get};
use axum::routing::{delete, post, put};
use sqlx::SqlitePool;

mod models;
mod handlers;
mod migrations;


#[tokio::main]
async fn main() {
    migrations::create_db().await;

    let pool = SqlitePool::connect(migrations::DB_URL).await.unwrap();

    migrations::migrate(&pool).await;

    let app = Router::new()
        .route("/", get(handlers::handle_index))
        .route("/todo", post(handlers::handle_create))
        .route("/todo/:id", delete(handlers::handle_delete))
        .route("/todo/:id", put(handlers::handle_complete))
        .with_state(pool);

    let listen_addr: SocketAddr = format!("{}:{}", "127.0.0.1", "3000")
        .parse()
        .unwrap();

    axum::Server::bind(&listen_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
