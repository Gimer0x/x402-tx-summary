use axum::response::IntoResponse;
use http::StatusCode;
use axum::extract::Path;
use crate::services::tx_fetcher::tx_fetcher;
use dotenvy::var;
use crate::services::tx_decoder::decoder;


pub async fn fetcher(Path(tx_hash): Path<String>) -> impl IntoResponse {

    let alchemy_api_key = var("ALCHEMY_API_KEY").unwrap();

    let rpc_url = alchemy_api_key.to_string();
    let result = tx_fetcher(rpc_url.as_str(),tx_hash.as_str()).await;
    match result {
        Ok(Some(tx)) => {

            let decoded = decoder(&tx).await;
            match decoded {
                Ok(decoded_tx) => (
                    StatusCode::OK,
                    format!("{:?}", decoded_tx),
                ),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error decoding transaction: {}", e),
                ),
            }
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, "Transaction not found".to_string())
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching transaction: {}", e))
        }
    }
}