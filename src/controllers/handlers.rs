use crate::errors::errors::ApiError;
use crate::services::tx_data::get_tx_data;
use crate::services::tx_fetcher::tx_fetcher;
use crate::utils::{blockchain::get_chain_info, tools::get_rpc_url};
use axum::Json;
use axum::body::Body;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use http::{StatusCode, header};
use serde::Deserialize;
use serde_json::{Value, json};

pub async fn fetcher(
    Path((network, tx_hash)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    fetch_summary(network, tx_hash).await
}

#[derive(Deserialize)]
pub struct SummaryRequest {
    pub network: String,
    pub tx_hash: String,
}

pub async fn fetcher_body(Json(req): Json<SummaryRequest>) -> Result<impl IntoResponse, ApiError> {
    fetch_summary(req.network, req.tx_hash).await
}

async fn fetch_summary(network: String, tx_hash: String) -> Result<impl IntoResponse, ApiError> {
    validate(&network, &tx_hash)?;

    let rpc_url = get_rpc_url(&network).unwrap();

    let result = tx_fetcher(&rpc_url.as_str(), &tx_hash).await;
    match result {
        Ok(Some(tx)) => match get_tx_data(&tx, &tx_hash).await {
            Ok(tx_data) => {
                let body: Value = serde_json::to_value(&tx_data)
                    .unwrap_or_else(|_| json!({ "error": "failed to serialize decoded tx" }));

                Ok((StatusCode::OK, Json(body)))
            }
            Err(e) => Err(ApiError::InternalError(e.to_string())),
        },
        Ok(None) => Err(ApiError::TXNotFound),
        Err(e) => Err(ApiError::InternalError(e.to_string())),
    }
}

fn validate(network: &str, tx_hash: &str) -> Result<(), ApiError> {
    let chain_info = get_chain_info(network.parse::<u64>().unwrap());

    if chain_info.0 == "Unknown" {
        return Err(ApiError::InvalidNetwork);
    }

    if !tx_hash.starts_with("0x") || tx_hash.len() != 66 {
        return Err(ApiError::InvalidTxHash);
    }

    Ok(())
}

pub async fn openapi_json() -> impl IntoResponse {
    // Path is resolved at compile-time relative to this source file.
    // Adjust ../ as needed from src/app.rs -> project root/docs/openapi.json
    let body = include_str!("./docs/openapi.json");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        body,
    )
}

pub async fn openapi_yaml() -> impl IntoResponse {
    // Path is resolved at compile-time relative to this source file.
    // Adjust ../ as needed from src/app.rs -> project root/docs/openapi.json
    let body = include_str!("./docs/openapi.yaml");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        body,
    )
}

pub async fn x402_well_known() -> impl IntoResponse {
    Json(json!({
        "version": 1,
        "resources": ["POST /summary"]
    }))
}

const FAVICON_PNG: &[u8] = include_bytes!("../../assets/favicon.png");

pub async fn favicon_get() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "image/png")], FAVICON_PNG)
}

/// Discovery tools issue `HEAD /favicon.ico` and require `Content-Type: image/*`.
pub async fn favicon_head() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .body(Body::empty())
        .expect("valid response")
}
