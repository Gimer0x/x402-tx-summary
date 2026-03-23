use crate::errors::errors::ApiError;
use crate::services::tx_data::get_tx_data;
use crate::services::tx_fetcher::tx_fetcher;
use crate::utils::{blockchain::get_chain_info, tools::get_rpc_url};
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use http::{StatusCode, header};
use serde_json::{Value, json};

pub async fn fetcher(
    Path((network, tx_hash)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    validate(&network, &tx_hash)?;

    let rpc_url = get_rpc_url(&network).unwrap();

    let result = tx_fetcher(&rpc_url.as_str(), tx_hash.as_str()).await;
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
