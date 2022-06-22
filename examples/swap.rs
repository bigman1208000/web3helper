use std::{env, thread, time};
use tokio::time::Instant;
use web3::contract::Contract;
use web3::ethabi::Uint;
use web3::transports::Http;
use web3::types::{H160, H256, U256};
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

    // init contract
    // usuario1
    let contract_abi = include_bytes!("../abi/TokenAbi.json");
    let contract_address = "0x4b55b91D7854CcE48B2794e60f53266B6B896125";
    let contract_instance: Contract<Http> = web3m
        .instance_contract(contract_address, contract_abi)
        .await
        .expect("error creating the contract instance");

    // call example
    let account: H160 = web3m.first_loaded_account();
    let token_balance: Uint = web3m
        .query_contract(&contract_instance, "balanceOf", account)
        .await
        .unwrap();
    println!("token_balance: {}", token_balance);

    let value = "100000000000000";
    //println!("value: {:?}", wei_to_eth(value));

    let path_address: Vec<&str> = vec![
        "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // WAVAX
        "0x7ef95a0FEE0Dd31b22626fA2e10Ee6A223F8a684", // TOKEN
    ];
    

    let now = Instant::now();
    let slippage = 40usize;

    for _ in 0..10 {
        let tx_id: H256 = web3m
            .swap_eth_for_exact_tokens(
                account,
                "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3",
                value,
                &path_address,
                slippage,
            )
            .await
            .unwrap();
        println!("tx_id: {:?}", tx_id);

        //let sleep_time = time::Duration::from_millis(100);
        //thread::sleep(sleep_time);
    }

    let elapsed = now.elapsed();
    println!("elapsed: {:?}", elapsed);

    Ok(())
}
