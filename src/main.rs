use std::env;
use web3::contract::Contract;
use web3::ethabi::Uint;
use web3::types::H160;

use std::time::Instant;
use web3::ethabi::ethereum_types::H256;
use web3::transports::Http;
use rust_web3_helper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let web3_http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
    let web3_websocket_url =
        "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";

    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url).await;

    // load acount from .env file
    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;

    // init contract
    // usuario1
    let contract_abi = include_bytes!("../abi/TokenAbi.json");
    let contract_address = "0x7ef95a0FEE0Dd31b22626fA2e10Ee6A223F8a684";
    let contract_instance: Contract<Http> = web3m
        .instance_contract(contract_address, contract_abi)
        .await
        .expect("error creating the contract instance");

    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
    let router_abi = include_bytes!("../abi/RouterAbi.json");
    let router_instance: Contract<Http> = web3m
        .instance_contract(router_address, router_abi)
        .await
        .expect("error creating the router instance");

    // call example
    let account: H160 = web3m.first_loaded_account();
    let balance_of: Uint = web3m
        .query_contract(&contract_instance, "balanceOf", account)
        .await;

    println!("balance_of tokens: {:?}", balance_of);
    // -------------------------

    let value = "10000000000000000";

    let token_a = "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd";
    let token_b = "0x7ef95a0FEE0Dd31b22626fA2e10Ee6A223F8a684";
    let path_address: Vec<&str> = vec![token_a, token_b];

    let now = Instant::now();

    let tx_id: H256 = web3m
        .swap_eth_for_exact_tokens(account, &router_instance, value, &path_address)
        .await
        .unwrap();

    let elapsed = now.elapsed();
    println!("elapsed: {:?}", elapsed);
    println!(
        "Transaction successful with hash: {}{:?}",
        &env::var("EXPLORER").unwrap(),
        tx_id
    );

    Ok(())
}
