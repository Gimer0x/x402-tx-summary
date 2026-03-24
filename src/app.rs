use alloy_primitives::Address;
use axum::{BoxError, Router, error_handling::HandleErrorLayer, http::StatusCode};
use std::time::Duration;
use tower::{ServiceBuilder, limit::concurrency::ConcurrencyLimitLayer, load_shed::LoadShedLayer};
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use url::Url;
use x402_axum::X402Middleware;
use x402_chain_eip155::{KnownNetworkEip155, V2Eip155Exact};
use x402_types::networks::USDC;

use crate::{
    config::Config,
    routes::api_routes::{openapi_routes, ok_route, tx_routes},
};

pub async fn build_app(config: Config) -> eyre::Result<Router> {
    let governor_config = GovernorConfigBuilder::default()
        // Slightly relaxed limits to avoid false 429s during automated discovery probes.
        .per_second(10)
        .burst_size(30)
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

    let mut x402_mw = X402Middleware::new(&config.facilitator_url);

    if let Some(base) = &config.x402_public_base_url {
        x402_mw = x402_mw.with_base_url(Url::parse(base)?);
    }

    let x402 = x402_mw
        .with_price_tag(V2Eip155Exact::price_tag(
            receiver_address,
            USDC::base().parse(price).unwrap(),
        ))
        .with_description("Semantic transaction decode result".to_string())
        .with_mime_type("application/json".to_string());

    // Keep `/health` outside rate limits, load shed, and body limits so Fly
    // `http_service.checks` (and k8s-style probes) always get 200 from a cheap handler.
    let paid_api = tx_routes()
        .layer(x402)
        .layer(cors(config.cors_origin))
        .layer(middleware_stack);

    let app = Router::new()
        .merge(ok_route())
        .merge(openapi_routes())
        .merge(paid_api);

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
