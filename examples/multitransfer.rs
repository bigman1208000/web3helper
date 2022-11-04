use futures::StreamExt;
use std::error::Error;
use std::io;
use web3::api::SubscriptionStream;
use web3::contract::Contract;
use web3::transports::{Http, WebSocket};
use web3::types::Log;
use web3::types::{TransactionId, H160};
use web3_rust_wrapper::Web3Manager;

use csv;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let web3_http_url = "https://bsc-testnet.nodereal.io/v1/d4224d2458594df5830eb45cdef8b45b";
    let web3_websocket_url = "wss://bsc-testnet.nodereal.io/ws/v1/d4224d2458594df5830eb45cdef8b45b";

    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 97).await;

    // load acount from .env file
    web3m
        .load_account(
            "0xe21Ce03236B84926c68f790a7d9D50E55dA772bC",
            "647e7f5b84e301ae4890cca81e6cb01a31f56574ea4ecde352f7a8c836486378",
        )
        .await;

    let contract_abi = include_bytes!("../abi/TokenAbi.json");
    let contract_address = "0xc43aF0698bd618097e5DD933a04F4e4a5A806834";
    let contract_instance: Contract<Http> = web3m
        .instance_contract(contract_address, contract_abi)
        .await
        .unwrap();

    // Creates a new csv `Reader` from `stdin`
    let mut reader = csv::Reader::from_reader(io::stdin());

    let headers = reader.headers().unwrap();

    println!("Headers: {:?}", headers);

    // `.records` return an iterator of the internal
    // record structure
    for result in reader.records() {
        let record = result.unwrap();

        println!("{:?}", record);
    }

    web3m
        .sent_erc20_token(
            web3m.first_loaded_account(),
            contract_instance,
            "0xc43aF0698bd618097e5DD933a04F4e4a5A806834",
            "1000000000000000000",
        )
        .await;

    Ok(())
}