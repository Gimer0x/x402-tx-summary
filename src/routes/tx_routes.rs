use axum::{
    Router,
    routing::{get, post},
};

use crate::controllers::handlers::fetcher;

pub fn tx_routes() -> Router {
    Router::new()
        .route("/summary/{network}/{tx_hash}", post(fetcher))
        .route("/health", get(|| async { "ok" }))
}
