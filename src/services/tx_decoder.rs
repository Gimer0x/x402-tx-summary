use alloy::consensus::transaction::EthereumTxEnvelope;
use alloy::rpc::types::Transaction;
use alloy_primitives::{Address, Bytes, U128, U256};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::error::Error;


#[derive(Serialize, Debug)]
pub struct DecodedTx {
    signer: String,
    block_number: u64,
    block_hash: String,
    transaction_index: u64,
    effective_gas_price: U128,
    data: DecodedTxData
}

impl DecodedTx {
    pub fn new(
            signer: String, 
            block_number: u64, 
            block_hash: String, 
            transaction_index: u64, 
            effective_gas_price: U128,
            data: DecodedTxData
    ) -> Self {
        Self {
            signer, 
            block_number,
            block_hash,
            transaction_index,
            effective_gas_price,
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecodedTxData {
    chain_id: u64,
    tx_type: &'static str,
    value: U256,
    nonce: u64,
    gas_limit: u64,
    gas_price: U128,
    to: Address,
    input: Bytes,
}

pub async fn tx_decoder<T>(tx: &Transaction<EthereumTxEnvelope<T>>) -> Result<DecodedTx, Box<dyn Error>> {

    let recovered = &tx.inner;
    let envelope = recovered.inner();

    let zero_addr = Address::from_slice(&[0u8; 20]);

    let (chain_id, tx_type, value, nonce, gas_limit, gas_price, to, input): (
        u64,
        &'static str,
        U256,
        u64,
        u64,
        U128,
        Address,
        Bytes,
    ) = match envelope {
        EthereumTxEnvelope::Legacy(signed) => {
            let t = signed.tx();
            let to = *t.to.to().unwrap_or(&zero_addr);
            (
                t.chain_id.unwrap_or(0),
                "legacy",
                t.value,
                t.nonce,
                t.gas_limit,
                U128::from(t.gas_price),
                to,
                t.input.clone(),
            )
        }
        EthereumTxEnvelope::Eip1559(signed) => {
            let t = signed.tx();
            let to = *t.to.to().unwrap_or(&zero_addr);
            (
                t.chain_id,
                "eip1559",
                t.value,
                t.nonce,
                t.gas_limit,
                U128::from(t.max_fee_per_gas),
                to,
                t.input.clone(),
            )
        }
        _ => (
            0,
            "unknown",
            U256::from(0u64),
            0,
            0,
            U128::from(0u64),
            zero_addr,
            Bytes::from(vec![]),
        ),
    };

    
    let data = DecodedTxData { chain_id, tx_type, value, nonce, gas_limit, gas_price, to, input };

    let decoded_tx = DecodedTx::new(
        tx.inner.signer().to_string(),
        tx.block_number.unwrap(),
        tx.block_hash.unwrap().to_string(),
        tx.transaction_index.unwrap(),
        U128::from(tx.effective_gas_price.unwrap()),
        data,
    );

    

    Ok(decoded_tx)
}
