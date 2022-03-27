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

    let account: H160 = web3m.first_loaded_account();
    let token_balance: Uint = web3m
        .query_contract(&contract_instance, "balanceOf", account)
        .await;
    println!("token_balance: {}", token_balance);

    let busd_balance: Uint = web3m
        .query_contract(&busd_instance, "balanceOf", account)
        .await;
    println!("busd_balance: {}", busd_balance);

    println!("listen....");
    web3m.listen_contract_events("0x78867BbEeF44f2326bF8DDd1941a4439382EF2A7").await;
    println!("end");

    Ok(())
}
