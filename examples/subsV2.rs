use futures::StreamExt;
use web3::api::SubscriptionStream;
use web3::transports::{WebSocket};
use web3::types::{H160, TransactionId};
use web3::types::Log;
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let web3_http_url = "https://bsc-testnet.nodereal.io/v1/d4224d2458594df5830eb45cdef8b45b";
    let web3_websocket_url = "wss://bsc-testnet.nodereal.io/ws/v1/d4224d2458594df5830eb45cdef8b45b";

    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 97).await;

    // load acount from .env file
    web3m
        .load_account(
            "0xF49C6459D96Ca06c1C14698416089682aC6e8b88",
            "1f373d593b7ea77320b1d95cf6991058053f5421fad9db932160133b63f4f01e",
        )
        .await;

    //let contract_abi = include_bytes!("../abi/standartToken.json");
    let contract_address = "0x710E192Fbf911883E4D5D25645548fCE5011D14a";
    println!("listen....");
    let sub: SubscriptionStream<WebSocket, Log> =
        web3m.build_contract_events(contract_address).await;

    sub.for_each(|log| async {
        let l: Log = log.unwrap();
        println!("Address: {:?}", l.transaction_hash.unwrap());
        println!("Data: {:?}", l.data);
        println!("Data0: {:?}", l.data.0);
        println!("{}", std::str::from_utf8(&l.data.0).unwrap());
        println!("topics: {:?}", l.topics);
        println!("log_type: {:?}", l.log_type);

        let tx = web3m
            .web3http
            .eth()
            .transaction(TransactionId::Hash(l.transaction_hash.unwrap()))
            .await
            .unwrap()
            .unwrap();

        let from_addr = tx.from.unwrap_or(H160::zero());
        let to_addr = tx.to.unwrap_or(H160::zero());
        let value = tx.value;
        let input = tx.input;

        println!("from_addr: {:?}", from_addr);
        println!("to_addr: {:?}", to_addr);
        println!("value: {:?}", value);
        println!("input: {:?}", input);
    })
    .await;

    println!("end");

    Ok(())
}
