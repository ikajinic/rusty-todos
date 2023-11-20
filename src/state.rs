use rand_chacha::ChaCha8Rng;
use sqlx::SqlitePool;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub random: Arc<Mutex<ChaCha8Rng>>,
    pub pool: SqlitePool,
}
