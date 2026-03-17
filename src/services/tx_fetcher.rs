//! Fetch transaction by hash using Alloy (no ethers).

use alloy::providers::{Provider, RootProvider};
use alloy::rpc::types::Transaction;
use alloy::transports::http::reqwest::Url;
use alloy_primitives::B256;
use std::str::FromStr;

/// Fetches a transaction by hash from the given RPC URL.
/// Returns `Ok(Some(tx))` if found, `Ok(None)` if not found, or an error on RPC/parse failure.
pub async fn tx_fetcher(rpc_url: &str, tx_hash: &str) -> Result<Option<Transaction>, FetchTxError> {
    let url: Url = rpc_url
        .parse()
        .map_err(|e| FetchTxError::InvalidUrl(format!("{:?}", e)))?;
    let hash = B256::from_str(tx_hash).map_err(FetchTxError::InvalidHash)?;

    let provider = RootProvider::<alloy::network::Ethereum>::new_http(url);
    let tx = provider
        .get_transaction_by_hash(hash)
        .await
        .map_err(|e| FetchTxError::Rpc(e.to_string()))?;

    println!("Transaction: {:?}", tx);
    Ok(tx)
}

#[derive(Debug, thiserror::Error)]
pub enum FetchTxError {
    #[error("invalid RPC URL: {0}")]
    InvalidUrl(String),
    #[error("invalid transaction hash: {0}")]
    InvalidHash(#[from] alloy_primitives::hex::FromHexError),
    #[error("RPC error: {0}")]
    Rpc(String),
}
