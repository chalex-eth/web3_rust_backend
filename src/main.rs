use axum::{routing::get, Extension, Router, Server};
use dapp_rust::{counter::Counter, routes};
use ethers::{
    prelude::{Http, Provider},
    types::Address,
};
use eyre::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Ethers-rs part for interacting with the contract
    let provider = Provider::<Http>::try_from("https://rpc.goerli.eth.gateway.fm").unwrap();
    let counter_address: Address =
        String::from("0x5719046dd09aF1718306Fb6f6d3AB106B95C0d31").parse()?;
    let counter = Counter::new(counter_address, Arc::new(provider));

    // Axum server
    let app = Router::new()
        .route("/api/number/", get(routes::handle_number))
        .route("/api/block_number/", get(routes::handle_block_number))
        .layer(Extension(counter));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("LISTENING on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
