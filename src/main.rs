use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use std::env;
use std::process;
use std::str::FromStr;
use std::{thread, time::Duration};
use web3::contract::tokens::Tokenize;
use web3::contract::{Contract, Options};
use web3::ethabi::Uint;
use web3::types::{
    Address, BlockNumber, Bytes, SignedTransaction, TransactionParameters, H160, U256, U64,
};

// NOTE(elsuizo:2022-02-28): ya no hace falta hacer extern de crates
// extern crate serde;

use std::fs::File;
use std::io::Read;
use std::time::Instant;
use web3::ethabi::ethereum_types::H256;
use web3::transports::Http;
use web3::Web3;
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
        .load_accounts(
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
        .await;

    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
    let router_abi = include_bytes!("../abi/RouterAbi.json");
    let router_instance: Contract<Http> = web3m.instance_contract(router_address, router_abi).await;

    // call example
    let account = web3m.get_first_loaded_account();
    let balance_of: Uint = web3m
        .query_contract(contract_instance.clone(), "balanceOf", account)
        .await;

    println!("balance_of tokens: {}", balance_of);
    // -------------------------

    let value = "10000000000000000";

    let tokenA = "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd";
    let tokenB = "0x7ef95a0FEE0Dd31b22626fA2e10Ee6A223F8a684";
    let path_address: Vec<&str> = vec![tokenA, tokenB];

    let now = Instant::now();

    let tx_id: H256 = web3m.swap_eth_for_exact_tokens( router_instance, value, path_address).await;

    let elapsed = now.elapsed();
    println!("elapsed: {:?}", elapsed);
    println!(
        "Transaction successful with hash: {}{:?}",
        &env::var("EXPLORER").unwrap(),
        tx_id
    );
    //     println!("Elapsed: {:.2?}", elapsed);
    //web3m.sent_erc20_token( contract_instance,contract_address, value).await;


    //println!("query_result: {:?}", query_result);

    // let mut i = 0;

    // while i < 2 {
    //     let now = Instant::now();
    //
    //     // example of write contract
    //
    //     // to usuario2
    //     let to_address = "0x64F5F982AFD264d640A52cF0EC72A58103d18FEc";
    //     // una moneda(esto es porque no soporta floats)
    //     let value = "1000000000000000000";
    //     // web3m
    //     //     .approve_erc20_token(contract_instance.clone(), to_address, value)
    //     //     .await;
    //     web3m
    //         .sent_erc20_token(contract_instance.clone(), to_address, value)
    //         .await;
    //     // web3m
    //     //     .swap_erc20_token(router_instance.clone(), "1000000000000000000")
    //     //     .await;
    //
    //     let elapsed = now.elapsed();
    //     println!("Elapsed: {:.2?}", elapsed);
    //     i = i + 1;
    // }

    // -------------------------

    Ok(())
}
