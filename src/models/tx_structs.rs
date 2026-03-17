use serde::Serialize;
use alloy_primitives::Address;

#[derive(Serialize, Debug)]
pub struct InputTxData {
    pub r#type: String,
    pub subtype: String,
    pub intent: String,
    pub summary: String,
    pub from: String,
    pub receipient: String,
    pub asset_in: String,
    pub asset_out: String,
    pub amount: String
}

#[derive(Serialize, Debug)]
pub struct FetchedTxData {
    pub block_number: u64,
    pub block_hash: String,
    pub transaction_index: u64,
    pub effective_gas_price: String,
    pub data: DecodedTxData,
}

#[derive(Serialize, Debug)]
pub struct DecodedTxData {
    pub chain: ChainInfo,
    pub tx_type: &'static str,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: String,
    pub to: Address,
    pub input_data: InputTxData
}

#[derive(Serialize, Debug)]
pub struct ChainInfo {
    pub chain_id: u64,
    pub name: String,
}