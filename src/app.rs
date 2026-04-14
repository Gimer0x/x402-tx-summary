use alloy_primitives::Address;
use axum::{
    BoxError, Router,
    body::Body,
    error_handling::HandleErrorLayer,
    http::{StatusCode, header},
    middleware,
    response::Response,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
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
use crate::utils::bazaar::{enrich_payment_required_for_x402scan};

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
            Duration::from_secs(20),
        ));

    let receiver_address: Address = config.receiver_address.parse()
        .map_err(|e| eyre::eyre!("Invalid receiver address: {}", e))?;

    let price: f64 = config.request_price;

    let mut x402_mw = X402Middleware::new(&config.facilitator_url);

    if let Some(base) = &config.x402_public_base_url {
        x402_mw = x402_mw.with_base_url(Url::parse(base)?);
    }

    // Note: we are using Base Sepolia for the x402 price tag.
    let x402 = x402_mw
        .with_price_tag(V2Eip155Exact::price_tag(
            receiver_address,
            USDC::base()
                .parse(price)
                .map_err(|e| eyre::eyre!("Invalid price: {}", e))?,
        ))
        .with_description("Semantic transaction decode result".to_string())
        .with_mime_type("application/json".to_string());

    // `map_response` must sit *outside* `X402Middleware`: unpaid requests get 402 from x402 without
    // calling inner services, so an inner `map_response` would never run.
    let paid_api = tx_routes()
        .layer(middleware_stack)
        .layer(x402)
        .layer(middleware::map_response(mirror_payment_required_into_body))
        .layer(cors(config.cors_origin));

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

// This function turns a 402 “you need to pay” answer that might only live in a header 
// into the same information also shown in the JSON body, so it’s easier to read and work 
//with—especially for beginners and automated tools.
async fn mirror_payment_required_into_body(mut res: Response) -> Response {
    if res.status() != StatusCode::PAYMENT_REQUIRED {
        return res;
    }

    // x402 v2 may return a base64 JSON challenge in `Payment-Required` header with empty body.
    let Some(payment_required) = res
        .headers()
        .get("payment-required")
        .and_then(|h| h.to_str().ok())
        .map(str::to_owned)
    else {
        return res;
    };

    let Ok(decoded) = BASE64_STANDARD.decode(payment_required.as_bytes()) else {
        return res;
    };
    let Ok(mut payload) = serde_json::from_slice::<serde_json::Value>(&decoded) else {
        return res;
    };

    payload = enrich_payment_required_for_x402scan(payload);

    let Ok(encoded_vec) = serde_json::to_vec(&payload) else {
        return res;
    };
    let encoded = BASE64_STANDARD.encode(&encoded_vec);
    if let Ok(hv) = header::HeaderValue::from_str(&encoded) {
        res.headers_mut().insert("payment-required", hv);
    }

    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    res.headers_mut().remove(header::CONTENT_LENGTH);
    *res.body_mut() = Body::from(encoded_vec);
    res
}
