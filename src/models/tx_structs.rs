use serde::Serialize;
use alloy_primitives::{Address};

#[derive(Serialize, Debug)]
pub struct InputTxData {
    pub r#type: String,
    pub subtype: String,
    pub intent: String,
    pub summary: String,
    pub participants: Participants,
    pub assets_in: Vec<TokenInfo>,
    pub assets_out: Vec<TokenInfo>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionInfo>,
    pub direction: String,
}

#[derive(Serialize, Debug)]
pub struct FunctionInfo {
    pub selector: String,
    pub name: String,
}


#[derive(Serialize, Debug)]
pub struct Participants {
    pub sender: String,
    pub receiver: String,
}

#[derive(Serialize, Debug)]
pub struct FetchedTxData {
    pub schema_version: String,
    pub block_number: u64,
    pub block_hash: String,
    pub tx_hash: String,
    pub transaction_index: u64,
    pub data: DecodedTxData,
    pub signer: String,
}

#[derive(Serialize, Debug)]
pub struct DecodedTxData {
    pub chain: ChainInfo,
    pub tx_type: &'static str,
    pub nonce: u64,
    pub gas: Gas,
    pub to: Address,
    pub actions: Vec<InputTxData>
}
#[derive(Serialize, Debug)]
pub struct Gas {
    pub limit: u64,
    pub price: u64,
    pub effective_gas_price: u64,
    pub max_fee_wei: u128,
    pub max_fee_eth: f64,
}

#[derive(Serialize, Debug)]
pub struct ChainInfo {
    pub chain_id: u64,
    pub name: String,
    pub native_asset: NativeAsset,
}

#[derive(Serialize, Debug)]
pub struct NativeAsset {
    pub symbol: String,
    pub decimals: u32,
}

#[derive(Serialize, Debug)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub raw_amount: u128,
    pub amount: f64,
    pub token_address: String,
}