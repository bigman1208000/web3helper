use std::env;
use web3::contract::Contract;
use web3::ethabi::Uint;
use web3::transports::Http;
use web3::types::H160;
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let web3_http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
    let web3_websocket_url =
        "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";

    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 97).await;

    // load acount from .env file
    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;

    let contract_abi = include_bytes!("../abi/TokenAbi.json");
    let contract_address = "0xB7926C0430Afb07AA7DEfDE6DA862aE0Bde767bc";
    println!("listen....");
    web3m.listen_contract_events(contract_address).await;
    println!("end");

    Ok(())
}
