use alloy::rpc::types::Transaction;
use alloy::consensus::transaction::EthereumTxEnvelope;
// value type is usually a big integer; use the exact type from your imports
use alloy_primitives::U256; // or Uint<256,4> depending on your setup
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct DecodedTx {
    signer: String,
    block_number: u64,
    block_hash: String,
    transaction_index: u64,
    effective_gas_price: U256,
    chain_id: u64,
    tx_type: &'static str,
    value: U256,
}

impl DecodedTx {
    pub fn new(
            signer: String, 
            block_number: u64, 
            block_hash: String, 
            transaction_index: u64, 
            effective_gas_price: U256,
            chain_id: u64,
            tx_type: &'static str,
            value: U256
    ) -> Self {
        Self {
            signer,
            block_number,
            block_hash,
            transaction_index,
            effective_gas_price,
            chain_id,
            tx_type,
            value,
        }
    }
}

pub async fn tx_decoder<T>(tx: &Transaction<EthereumTxEnvelope<T>>) -> Result<DecodedTx, Box<dyn Error>> {

    // `recovered` is Recovered<EthereumTxEnvelope>
    let recovered = &tx.inner;
    // `envelope` is &EthereumTxEnvelope
    let envelope = recovered.inner();

    let (chain_id, tx_type, value): (u64, &'static str, U256) = match envelope {
        EthereumTxEnvelope::Legacy(signed) => {
            let t = signed.tx();
            (t.chain_id.unwrap_or(0), "legacy", t.value)
        }
        EthereumTxEnvelope::Eip1559(signed) => {
            let t = signed.tx();
            (t.chain_id, "eip1559", t.value)
        }
        // handle other variants if you care about them:
        // EthereumTxEnvelope::Eip2930(signed) => { ... }
        // EthereumTxEnvelope::Eip4844(signed) => { ... }
        _ => {
            // fallback if you don't want to handle all types now
            (0, "unknown", U256::from(0u64))
        }
    };

    
    let decoded_tx = DecodedTx::new(
        tx.inner.signer().to_string(),
        tx.block_number.unwrap(), 
        tx.block_hash.unwrap().to_string(), 
        tx.transaction_index.unwrap(), 
        U256::from(tx.effective_gas_price.unwrap()),
        chain_id,
        tx_type,
        value
    );

    

    Ok(decoded_tx)
}
