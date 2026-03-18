use crate::services::tx_data::get_tx_data;
use crate::services::tx_fetcher::tx_fetcher;
use crate::utils::etherscan::EtherscanAbiError;
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
//use dotenvy::var;
use crate::utils::tools::get_rpc_url;
use http::StatusCode;
use serde_json::{Value, json};

pub async fn fetcher(Path((network, tx_hash)): Path<(String, String)>) -> impl IntoResponse {
    let rpc_url = get_rpc_url(&network).unwrap();

    let result = tx_fetcher(&rpc_url.as_str(), tx_hash.as_str()).await;
    match result {
        Ok(Some(tx)) => match get_tx_data(&tx, &tx_hash).await {
            Ok(tx_data) => {
                let body: Value = serde_json::to_value(&tx_data)
                    .unwrap_or_else(|_| json!({ "error": "failed to serialize decoded tx" }));

                (StatusCode::OK, Json(body))
            }
            Err(e) => {
                let body = json!({ "error": e.to_string() });
                let status = if e.is::<EtherscanAbiError>() {
                    StatusCode::BAD_REQUEST
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                };
                (status, Json(body))
            }
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Transaction not found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Error fetching transaction: {e}") })),
        ),
    }
}
