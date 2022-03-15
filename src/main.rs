use std::env;
use web3::contract::Contract;
use web3::ethabi::Uint;
use web3::types::{H160, U256};

use std::time::Instant;
use web3::ethabi::ethereum_types::H256;
use web3::transports::Http;
use web3_rust_wrapper::Web3Manager;

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
    let contract_address = "0x883ccFF843a0bd58783903041ed7f02D97Ce4513";
    let contract_instance: Contract<Http> = web3m
        .instance_contract(contract_address, contract_abi)
        .await
        .expect("error creating the contract instance");

    let busd_abi = include_bytes!("../abi/TokenAbi.json");
    let busd_address = "0x78867BbEeF44f2326bF8DDd1941a4439382EF2A7";
    let busd_instance: Contract<Http> = web3m
        .instance_contract(busd_address, busd_abi)
        .await
        .expect("error creating the contract instance");

    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
    let router_abi = include_bytes!("../abi/RouterAbi.json");
    let router_instance: Contract<Http> = web3m
        .instance_contract(router_address, router_abi)
        .await
        .expect("error creating the router instance");

    let EAC_aggregator_proxy_address = "0x887f177CBED2cf555a64e7bF125E1825EB69dB82";
    let EAC_aggregator_proxy_abi = include_bytes!("../abi/EACAggregatorProxy.json");
    let EAC_aggregator_proxy_instance: Contract<Http> = web3m
        .instance_contract(EAC_aggregator_proxy_address, EAC_aggregator_proxy_abi)
        .await
        .expect("error creating the router instance");

    let latest_round_data: (Uint, i32, Uint, Uint, Uint) = web3m
        .query_contract(&EAC_aggregator_proxy_instance, "latestRoundData", ())
        .await;
    println!("latestRoundData: {:?}", latest_round_data);

    // call example
    let account: H160 = web3m.first_loaded_account();
    let token_balance: Uint = web3m
        .query_contract(&contract_instance, "balanceOf", account)
        .await;
    println!("token_balance: {}", token_balance);

    let busd_balance: Uint = web3m
        .query_contract(&busd_instance, "balanceOf", account)
        .await;
    println!("busd_balance: {}", busd_balance);
    // -------------------------

    let value = "10000000000000000";
    //println!("value: {:?}", wei_to_eth(value));

    let path_address: Vec<&str> = vec![
        "0x78867BbEeF44f2326bF8DDd1941a4439382EF2A7", // BUSD
        "0x883ccFF843a0bd58783903041ed7f02D97Ce4513", // TOKEN
    ];

    let now = Instant::now();
    let slippage = 3usize;

    let tx_id: H256 = web3m
        .swap_tokens_for_exact_tokens(account, &router_instance, value, &path_address, slippage)
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
    let a: U256 = U256::from_dec_str(v.clone().to_string().as_str()).unwrap();
    //et k = wei_to_eth(a);
    return a;
}
