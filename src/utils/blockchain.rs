


pub fn get_chain_info(chain_id: u64) -> (&'static str, &'static str) {
    match chain_id {
        1 => ("Ethereum", "ETH"),
        5 => ("Ethereum Goerli", "ETH"),
        137 => ("Polygon", "MATIC"),
        80001 => ("Polygon Mumbai", "MATIC"),
        42161 => ("Arbitrum", "ETH"),
        421611 => ("Arbitrum Sepolia", "ETH"),
        8453 => ("Base", "ETH"),
        84532 => ("Base Sepolia", "ETH"),
        _ => ("Unknown", "UNKNOWN"),
    }
}

pub fn get_token_info(chain_id: u64, token_address: &str) -> (String, String, u64) {
    match (chain_id, token_address) {
        (1, "0x0000000000000000000000000000000000000000") => ("Ethereum".to_string(), "ETH".to_string(), 18),
        (8453, "0x0000000000000000000000000000000000000000") => ("Ethereum".to_string(), "ETH".to_string(), 18),
        (8453, "0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2") => ("Tether USD".to_string(), "USDT".to_string(), 6),
        (84532, "0xD77811e522177b08a87921EF110e9b8040c12F13") => ("Tether USD".to_string(), "USDT".to_string(), 6),

        _ => ("Unknown".to_string(), "UNKNOWN".to_string(), 0),
    }
}