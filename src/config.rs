use dotenvy::var;

#[derive(Clone)]
pub struct Config {
    pub server_addr: String,
    pub facilitator_url: String,
    pub receiver_address: String,
    pub request_price: f64,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> eyre::Result<Self> {
        Ok(Self {
            server_addr: var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            facilitator_url: var("FACILITATOR_URL")
                .map_err(|e| eyre::eyre!("FACILITATOR_URL missing: {e}"))?,
            receiver_address: var("RECEIVER_ADDRESS")
                .map_err(|e| eyre::eyre!("RECEIVER_ADDRESS missing: {e}"))?,
            request_price: var("REQUEST_PRICE")
                .map_err(|e| eyre::eyre!("REQUEST_PRICE missing: {e}"))?
                .parse()?,
            cors_origin: var("CORS_ORIGIN").unwrap_or_else(|_| "https://dappdojo.com".to_string()),
        })
    }
}
