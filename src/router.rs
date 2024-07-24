use axum::{
    routing::{get}, Router,
};
use sqlx::PgPool;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn init_router(db: PgPool) -> Router {
    let state = AppState { db };

    Router::new()
        .route("/", get(routes::home))
        .route("/output.css", get(routes::styles))
        .with_state(state)
}
