use axum::{Router, routing::{get, post}};

use crate::controllers::handlers::{
    favicon_get, favicon_head, fetcher, fetcher_body, openapi_json, openapi_yaml, x402_well_known,
};

pub fn tx_routes() -> Router {
    Router::new()
        .route("/summary", post(fetcher_body))
        .route("/summary/{network}/{tx_hash}", post(fetcher))
}

pub fn openapi_routes() -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/openapi.yaml", get(openapi_yaml))
        .route("/.well-known/x402", get(x402_well_known))
}

pub fn ok_route() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/favicon.ico", get(favicon_get).head(favicon_head))
}