use axum::{
    Router,
    routing::{ post},
};

use crate::controllers::handlers::fetcher;

pub fn tx_routes() -> Router {
    Router::new()
        .route("/summary/{network}/{tx_hash}", post(fetcher))
}
