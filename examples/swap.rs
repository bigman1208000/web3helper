use std::env;
use tokio::time::Instant;
use web3::contract::Contract;
use web3::ethabi::Uint;
use web3::transports::Http;
use web3::types::{H160, H256, U256};
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let web3_http_url = "https://api.avax-test.network/ext/bc/C/rpc";
    let web3_websocket_url = "wss://api.avax-test.network/ext/bc/C/ws";

    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 43113).await;

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
    let contract_address = "0x6d1dF17C0A44F0f35c00e5cA0Ea622701afDc8d1";
    let contract_instance: Contract<Http> = web3m
        .instance_contract(contract_address, contract_abi)
        .await
        .expect("error creating the contract instance");

    let router_address = "0x2D99ABD9008Dc933ff5c0CD271B88309593aB921";
    let router_abi = include_bytes!("../abi/RouterAbi.json");
    let router_instance: Contract<Http> = web3m
        .instance_contract(router_address, router_abi)
        .await
        .expect("error creating the router instance");

    // call example
    let account: H160 = web3m.first_loaded_account();
    let token_balance: Uint = web3m.query_contract(&contract_instance, "balanceOf", account).await.unwrap();
    println!("token_balance: {}", token_balance);    

  
    let value = "100000000000000000";
    //println!("value: {:?}", wei_to_eth(value));
 

    let path_address: Vec<&str> = vec![
        "0xd00ae08403B9bbb9124bB305C09058E32C39A48c", // WAVAX
        "0x4F8aD5346db86a0291E5359500B77be8A9057e43"  // TOKEN
        ];

    let now = Instant::now();
    let slippage = 40usize;

    let tx_id: H256 = web3m
        .swap_eth_for_exact_tokens(account, &router_instance, value, &path_address, slippage)
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

pub fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

pub fn eth_to_wei(eth_val: f64) -> U256 {
    let result = eth_val * 1_000_000_000_000_000_000.0;
    let result = result as u128;
    U256::from(result)
}

fn wei_to_eth2(val: &str) -> U256 {

    let v: f64 = val.parse().unwrap();
    let a:U256 = U256::from_dec_str(v.clone().to_string().as_str()).unwrap();
    //et k = wei_to_eth(a);
    return a;
}