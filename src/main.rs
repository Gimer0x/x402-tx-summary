use alloy_primitives::{Address};
use axum::{Router, routing::{get, post}};
use x402_axum::X402Middleware;
use x402_chain_eip155::{KnownNetworkEip155, V1Eip155Exact};
use x402_types::networks::USDC;
use tokio::net::TcpListener;
use dotenvy::var;

mod controllers;
use controllers::handlers::{my_handler, decoder};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let facilitator_url = var("FACILITATOR_URL").unwrap();
    let price = var("PRICE").unwrap().parse::<f64>().unwrap();

    let x402 = X402Middleware::new(facilitator_url.as_str());

    
    let receiver_address:Address = var("RECEIVER_ADDRESS")
        .unwrap()
        .parse()
        .expect("RECEIVER_ADDRESS must be a valid Ethereum address");

    let app: Router = Router::new()
        .route("/explain",get(my_handler))
        .route("/summary/{tx_hash}",post(decoder))
        .layer(
            x402.with_price_tag(V1Eip155Exact::price_tag(
                receiver_address,
                USDC::base_sepolia().parse(price).unwrap(),
            )
        ),
    );
    let app = app.into_make_service();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}