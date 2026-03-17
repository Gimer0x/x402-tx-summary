use crate::utils::{etherscan, tools::{self, TxType}};
use alloy::consensus::transaction::EthereumTxEnvelope;
use alloy::rpc::types::Transaction;
use alloy_primitives::{Address, Bytes, U128, U256};
use dotenvy::var;
use eyre::Result;
use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct InputTxData {
    r#type: String,
    subtype: String,
    intent: String,
    summary: String,
    from: String,
    to: String,
    asset_in: String,
    asset_out: String,
    amount: String
}

#[derive(Debug)]
pub struct StrError(pub String);
impl fmt::Display for StrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for StrError {}

#[derive(Serialize, Debug)]
pub struct FetchedTxData {
    block_number: u64,
    block_hash: String,
    transaction_index: u64,
    effective_gas_price: String,
    data: DecodedTxData,
}

impl FetchedTxData {
    pub fn new(
        block_number: u64,
        block_hash: String,
        transaction_index: u64,
        effective_gas_price: String,
        data: DecodedTxData,
    ) -> Self {
        Self {
            block_number,
            block_hash,
            transaction_index,
            effective_gas_price,
            data,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DecodedTxData {
    chain_id: u64,
    tx_type: &'static str,
    nonce: u64,
    gas_limit: u64,
    gas_price: String,
    to: Address,
    input_data: InputTxData
}

pub async fn get_tx_data<T>(
    tx: &Transaction<EthereumTxEnvelope<T>>,
) -> Result<FetchedTxData, Box<dyn Error>> {
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

    let tx_match = tools::match_tx_type(&input, value)?;

    let input_data = match tx_match {
        TxType::ETHTransfer => {
            get_native_tx(
                &tx.inner.signer().to_string().as_str(), 
                &to.to_string().as_str(), 
                value
            )
        },
        TxType::ERC20Transfer => {
            //decode_input_data(&input, chain_id, to).await?
            get_native_tx(
                &tx.inner.signer().to_string().as_str(), 
                &to.to_string().as_str(), 
                value
            )
        },
        TxType::Unknown => get_native_tx(
            &tx.inner.signer().to_string().as_str(), 
            &to.to_string().as_str(), 
            value
        )
        
    }?;

    let value = value.to_string();
    let gas_price = gas_price.to_string();
    let data = DecodedTxData {
        chain_id,
        tx_type,
        nonce,
        gas_limit,
        gas_price,
        to,
        input_data
    };

    let fetched_tx = FetchedTxData::new(
        tx.block_number.unwrap(),
        tx.block_hash.unwrap().to_string(),
        tx.transaction_index.unwrap(),
        tx.effective_gas_price.unwrap().to_string(),
        data,
    );

    Ok(fetched_tx)
}

pub fn get_native_tx(signer: &str, to: &str, value: U256) ->  Result<InputTxData, Box<dyn Error>>{

    let value_in_eth = tools::wei_to_eth_string(value);

    let summary = format!("Transfer {} ETH from {} to {}", value_in_eth, signer, to);
    let native_tx = InputTxData {
        r#type: "transfer".to_string(),
        subtype: "native".to_string(),
        intent: "send_money".to_string(),
        summary: summary,
        from: signer.to_string(),
        to: to.to_string(),
        asset_in: "ETH".to_string(),
        asset_out: "".to_string(),
        amount: value.to_string(),
    };
    
    Ok(native_tx)
}

pub async fn decode_input_data(
    input: &Bytes,
    chain_id: u64,
    to: Address,
) -> Result<String, Box<dyn Error>> {
    let etherscan_api_key =
        var("ETHERSCAN_API_KEY").map_err(|e| -> Box<dyn Error> { Box::new(e) })?;

    if input.len() < 4 {
        return Ok(input.to_string());
    }
    let selector = tools::get_selector(input)
        .map_err(|e| -> Box<dyn Error> { Box::new(StrError(e.to_string())) })?;

    // Let's check if the selector is a well-known selector
    let _abi = etherscan::fetch_etherscan_abi(
        chain_id,
        to.to_string().as_str(),
        selector,
        etherscan_api_key.as_str(),
    )
    .await
    .map_err(|e| -> Box<dyn Error> { Box::new(e) })?;
    Ok(input.to_string())
}
