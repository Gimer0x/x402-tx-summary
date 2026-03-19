use alloy_primitives::Address;
use axum::{Router, http::StatusCode};
use std::time::Duration;
use tower::{ServiceBuilder, limit::concurrency::ConcurrencyLimitLayer};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use x402_axum::X402Middleware;
use x402_chain_eip155::{KnownNetworkEip155, V1Eip155Exact};
use x402_types::networks::USDC;
use axum::routing::get;

use crate::{config::Config, routes::tx_routes::tx_routes};

pub async fn build_app(config: Config) -> eyre::Result<Router> {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(ConcurrencyLimitLayer::new(100));

    let receiver_address: Address = config.receiver_address.parse().unwrap();
    let price: f64 = config.request_price;

    let x402 = X402Middleware::new(&config.facilitator_url).with_price_tag(
        V1Eip155Exact::price_tag(receiver_address, USDC::base_sepolia().parse(price).unwrap()),
    );

    let protected = tx_routes().layer(x402);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(protected)
        .layer(middleware_stack)
        .layer(cors());

    Ok(app)
}

fn cors() -> CorsLayer {
    use axum::http::Method;
    use tower_http::cors::Any;

    CorsLayer::new()
        .allow_origin(Any) // restrict in prod
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}
