use std::env;
use web3::ethabi::Int;
use web3::types::Address;
use web3_rust_wrapper::rinkeby_testnet::RinkeByTestNet;
use web3_rust_wrapper::traits::GetAddress;
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let web3_http_url = "https://rinkeby.infura.io/v3/d8200853cc4c4001956d0c1a2d0de540";
    let web3_websocket_url = "wss://rinkeby.infura.io/ws/v3/d8200853cc4c4001956d0c1a2d0de540";
    let chain_id = 4;
    let web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, chain_id).await;

    let price: Int = web3m.get_latest_price(RinkeByTestNet, "ATOM / USD").await;
    println!("price: {:?}", price);

    Ok(())
}
