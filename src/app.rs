use alloy_primitives::Address;
use axum::{BoxError, Router, error_handling::HandleErrorLayer, http::StatusCode};
use std::time::Duration;
use tower::{ServiceBuilder, limit::concurrency::ConcurrencyLimitLayer, load_shed::LoadShedLayer};
use tower_http::{cors::{CorsLayer, AllowOrigin}, timeout::TimeoutLayer, trace::TraceLayer};
use x402_axum::X402Middleware;
use x402_chain_eip155::{KnownNetworkEip155, V1Eip155Exact};
use x402_types::networks::USDC;
use axum::routing::get;
use tower_governor::{
    GovernorLayer,
    governor::GovernorConfigBuilder,
    key_extractor::SmartIpKeyExtractor,
};
use tower_http::limit::RequestBodyLimitLayer;

use crate::{config::Config,routes::tx_routes::tx_routes};

pub async fn build_app(config: Config) -> eyre::Result<Router> {
    let governor_config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .key_extractor(SmartIpKeyExtractor)
        .use_headers()
        .finish()
        .unwrap();

    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|error: BoxError| async move {
            if error.is::<tower::load_shed::error::Overloaded>() {
                StatusCode::SERVICE_UNAVAILABLE
            } else {
                StatusCode::REQUEST_TIMEOUT
            }
        }))
        .layer(TraceLayer::new_for_http())
        .layer(LoadShedLayer::new())
        .layer(RequestBodyLimitLayer::new(32 * 1024))
        .layer(GovernorLayer::new(governor_config))
        .layer(ConcurrencyLimitLayer::new(50))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ));

    let receiver_address: Address = config.receiver_address.parse().unwrap();
    let price: f64 = config.request_price;

    let x402 = X402Middleware::new(&config.facilitator_url).with_price_tag(
        V1Eip155Exact::price_tag(receiver_address, USDC::base_sepolia().parse(price).unwrap()),
    );

    let x402 = tx_routes().layer(x402);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(x402)
        .layer(cors(config.cors_origin))
        .layer(middleware_stack);
    
    Ok(app)
}

fn cors(domain: String) -> CorsLayer {
    use axum::http::{HeaderValue, Method};
    use tower_http::cors::Any;

    let origin = HeaderValue::from_str(&domain).expect("invalid CORS origin");
    let allow_origin = AllowOrigin::list(vec![origin]);

    CorsLayer::new()
        .allow_origin(allow_origin) // restrict in prod
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}
