use crate::utils::{tools, blockchain::{get_chain_info, get_token_info}};
use crate::models::tx_structs::{InputTxData, TokenInfo, Participants, FunctionInfo};
use std::error::Error;
use alloy_primitives::{Bytes, U256, Address};
use alloy::hex;
// use dotenvy::var;
// use crate::utils::etherscan;
// use std::fmt;
use std::convert::TryInto;

//#[derive(Debug)]
//pub struct StrError(pub String);
//impl fmt::Display for StrError {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "{}", self.0)
//    }
//}
//impl Error for StrError {}

pub fn get_erc20_transfer_tx(input: &Bytes, chain_id: u64, signer: &str, token_address: &str) -> Result<InputTxData, Box<dyn Error>> {
    let (receiver, amount) = get_amount_and_receiver(input);

    let (token_name, token_symbol, token_decimals) = get_token_info(chain_id, &token_address.to_string());
    let amount_in_string = tools::from_wei_to_string(amount, token_decimals.try_into().unwrap());
    let summary = format!("Sent {amount_in_string} {token_symbol} from {signer} to {receiver}");

    let selector = tools::get_selector(input).unwrap();

    let token_info = TokenInfo {
        name: token_name,
        symbol: token_symbol,
        decimals: token_decimals,
        raw_amount: amount.to_string(),
        amount: amount_in_string,
        token_address: Some(token_address.to_string()),
    };

    let direction = if signer.to_string() != receiver.to_string() {
        "outgoing".to_string()
    } else {
        "incoming".to_string()
    };

    let native_tx = InputTxData {
        r#type: "transfer".to_string(),
        subtype: "erc20".to_string(),
        intent: "send_money".to_string(),
        summary: summary,
        participants: Participants {
            sender: signer.to_string(),
            receiver: receiver.to_string(),
        },
        assets_in: vec![token_info],
        assets_out: vec![],
        protocol: None,
        function: Some(FunctionInfo {
            selector: "0x".to_string() + &hex::encode(selector),
            name: "transfer".to_string(),
        }),
        direction: direction,
    };

    Ok(native_tx)
}

/// Decodes ERC20 transfer(address,uint256) calldata.
/// Layout: 4-byte selector, then 32-byte word (address right-padded), then 32-byte word (amount).
pub fn get_amount_and_receiver(input: &Bytes) -> (Address, U256) {
    let receiver = Address::from_slice(&input[16..36]); // last 20 bytes of first 32-byte word
    let bytes: [u8; 32] = input[36..68].try_into().unwrap(); // second 32-byte word
    let amount = U256::from_be_bytes(bytes);
    (receiver, amount)
}

pub fn get_native_tx(signer: &str, receiver: &str, value: U256, chain_id: u64) ->  Result<InputTxData, Box<dyn Error>>{

    let zero_address: Address = Address::from_slice(&[0u8; 20]);
    
    let (token_name, token_symbol, token_decimals) = get_token_info(chain_id, &zero_address.to_string());
    
    let value_in_string = tools::from_wei_to_string(U256::from(value), token_decimals.try_into().unwrap());

    let (chain_name, native_asset, _) = get_chain_info(chain_id);
    let summary = format!("Sent {value_in_string} {native_asset} from {signer} to {receiver} on {chain_name}");
    let token_info = TokenInfo {
        name: token_name,
        symbol: token_symbol,
        decimals: token_decimals,
        raw_amount: value.to_string(),
        amount: value_in_string,
        token_address: None,
    };

    let direction = if signer.to_string() != receiver.to_string() {
        "outgoing".to_string()
    } else {
        "incoming".to_string()
    };

    let native_tx = InputTxData {
        r#type: "transfer".to_string(),
        subtype: "native".to_string(),
        intent: "send_money".to_string(),
        summary: summary,
        participants: Participants {
            sender: signer.to_string(),
            receiver: receiver.to_string(),
        },
        assets_in: vec![token_info],
        assets_out: vec![],
        protocol: None,
        function: None,
        direction: direction,
    };
    
    Ok(native_tx)
}


/*pub async fn decode_input_data(
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
}*/