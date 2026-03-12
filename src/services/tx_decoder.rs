use alloy::rpc::types::Transaction;
use eyre::Result;
use serde::{Serialize, Deserialize};
use axum::Json;
#[derive(Serialize, Deserialize, Debug)]
pub struct DecodedTx {
    signer: String,
    block_number: u64,

}

impl DecodedTx {
    pub fn new(signer: String, block_number: u64) -> Self {
        Self { signer, block_number }
    }
}

pub async fn decoder<T>(tx: &Transaction<T>) -> Result<Json<DecodedTx>> {
    let signer = tx.inner.signer();


    let decoded_tx = DecodedTx::new(signer.to_string(), tx.block_number.unwrap());

    Ok(Json(decoded_tx))
}

