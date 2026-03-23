use axum::{Router, routing::{get, post}};

use crate::controllers::handlers::{fetcher, openapi_json, openapi_yaml};

pub fn tx_routes() -> Router {
    Router::new().route("/summary/{network}/{tx_hash}", post(fetcher))
}

pub fn openapi_routes() -> Router {
    Router::new().route("/openapi.json", get(openapi_json)).route("/openapi.yaml", get(openapi_yaml))
}

pub fn ok_route() -> Router {
    Router::new().route("/health", get(|| async { "ok" }))
}