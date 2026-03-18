use serde::Serialize;
use alloy_primitives::Address;

#[derive(Serialize, Debug)]
pub struct InputTxData {
    pub r#type: String,
    pub subtype: String,
    pub intent: String,
    pub summary: String,
    pub participants: Participants,
    pub asset_in: Vec<TokenInfo>,
    pub asset_out: Vec<TokenInfo>,
    pub amount: String,
    pub protocol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionInfo>,
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
    pub limit: String,
    pub price: String,
    pub effective_gas_price: String,
    pub max_fee_wei: String,
    pub max_fee_eth: String,
}

#[derive(Serialize, Debug)]
pub struct ChainInfo {
    pub chain_id: u64,
    pub name: String,
    pub native_asset: String,
}

#[derive(Serialize, Debug)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub raw_amount: String,
    pub amount: String,
    pub token_address: String,
}