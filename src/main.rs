use core::time;
use ethereum_abi::Abi;
use ethereum_abi::Value;
use futures::StreamExt;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use web3::api::SubscriptionStream;
use web3::contract::Contract;
use web3::contract::Options;
use web3::ethabi::Address;
use web3::futures::future;
use web3::transports::Http;
use web3::transports::WebSocket;
use web3::types::FilterBuilder;
use web3::types::Log;
use web3::types::H160;
use web3::types::U256;
use web3_rust_wrapper::Web3Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventToken {
    pub from: String,
    pub to: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnResult {
    pub users: Vec<EventToken>,
    pub sent: bool,
    pub tx_hash: String,
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    let (
        pulse_doge_manager_pk,
        pulse_doge_manager_address,
        bsc_token_address,
        eth_token_address,
        bsc_rpc_url,
        bsc_ws_url,
        eth_rpc_url,
        eth_ws_url,
        explorer_url,
        airdrop_users_number,
        period_minutes,
    ) = get_env_variables().await;

    // INITIALIZE Web3Manager
    let mut web3m_bsc: Web3Manager = Web3Manager::new(web3_rust_wrapper::Network::BSCTestnet).await;
    let mut web3m_eth: Web3Manager =
        Web3Manager::new(web3_rust_wrapper::Network::EthereumGoerli).await;
    web3m_eth
        .load_account(
            pulse_doge_manager_address.as_str(),
            pulse_doge_manager_pk.as_str(),
        )
        .await;

    let account = web3m_eth.first_loaded_account();

    let dead_address = "0x000000000000000000000000000000000000dead";

    let contract_abi = include_bytes!("../Pulsedoge.json");
    let eth_contract_instance: Contract<Http> = web3m_eth
        .instance_contract(eth_token_address.as_str(), contract_abi)
        .await
        .unwrap();

    println!("listen....");
    let filter = FilterBuilder::default()
        .address(vec![Address::from_str(bsc_token_address.as_str()).unwrap()])
        .topics(None, None, None, None)
        .build();

    let mut used_file: File;
    let abi: Abi = load_abi_from_json("standardToken.json");

    let mut burn_result: BurnResult = BurnResult {
        users: Vec::new(),
        sent: false,
        tx_hash: String::new(),
    };

    // check if burn results folder is empty, if is empty, create a new burn result file
    let burn_results_files_number = get_burn_results_folder_files_number();
    if burn_results_files_number == 0 {
        println!("create new burn results file");
        used_file = create_burn_results_file();
        let mut contents = String::new();
        used_file.read_to_string(&mut contents).unwrap();
        burn_result = serde_json::from_str(&contents).unwrap();
    } else {
        // get the last burn result file
        println!("get last burn results file");
        let mut files = get_burn_results_files();
        let number_of_files = files.len();

        println!("files: {:?}", &mut files[number_of_files - 1]);

        // check if the last burn result file is sent, if is sent, create a new burn result file
        let is_sent = is_burn_results_file_sent(&mut files[number_of_files - 1]);
        if is_sent {
            println!("sent, create new burn results file");
            used_file = create_burn_results_file();
            let mut contents = String::new();
            used_file.read_to_string(&mut contents).unwrap();
            burn_result = serde_json::from_str(&contents).unwrap();
        }
    }

    let filter = web3m_bsc
        .web3web_socket
        .eth_filter()
        .create_logs_filter(filter)
        .await?;

    let logs_stream = filter.stream(time::Duration::from_micros(1));
    futures::pin_mut!(logs_stream);

    while true {
        let log = logs_stream.next().await.unwrap();

        let event_token: EventToken = decode_log(&abi, &log.unwrap());
        println!("event_token: {:?}", event_token);

        let is_burn_ok = event_token.to == dead_address && event_token.amount.parse::<u128>().unwrap() >= 5555000000000000000000;
        if is_burn_ok {
            println!("Burn greater or equal than 5555");
            burn_result.users.push(event_token);

            if burn_result.users.len() >= airdrop_users_number.try_into().unwrap() {
                println!("Add users to holders");

                let tx_result = call_add_holders_function(
                    &mut web3m_eth,
                    &eth_contract_instance,
                    &burn_result.users,
                )
                .await;

                println!("tx_result {:?}", tx_result);
            }
        }
    }

    return Ok(());
}

async fn call_add_holders_function(
    web3m: &mut Web3Manager,
    contract_instance: &Contract<Http>,
    users: &Vec<EventToken>,
) -> web3::types::H256 {
    let contract_function = "addTokenHolders";
    let contract_function_parameters = create_add_holders_parameters(users);

    let tx_result = web3m
        .sign_and_send_tx(
            web3m.first_loaded_account(),
            &contract_instance,
            &contract_function.to_string(),
            &contract_function_parameters,
            U256::from_dec_str("0").unwrap(),
        )
        .await
        .unwrap();
    tx_result
}

fn create_add_holders_parameters(users: &Vec<EventToken>) -> (Vec<H160>, Vec<U256>) {
    let base: u128 = 10;
    let mut addresses = Vec::new();
    let mut amounts = Vec::new();

    for user in users {
        let parsed_amount = user.amount.parse::<u128>().unwrap() / base.pow(18);
        addresses.push(Address::from_str(user.from.as_str()).unwrap());
        amounts.push(U256::from(parsed_amount));
    }
    (addresses, amounts)
}

// function for decode log and return the decoded data
fn decode_log(abi: &Abi, log: &Log) -> EventToken {
    let topics: &[H256] = &[
        H256::from_str(&format!("{:#x}", log.topics[0])).unwrap(),
        H256::from_str(&format!("{:#x}", log.topics[1])).unwrap(),
        H256::from_str(&format!("{:#x}", log.topics[2])).unwrap(),
    ];

    // Decode
    let (_evt, decoded_data) = abi.decode_log_from_slice(topics, &log.data.0).unwrap();

    let mut event_token: EventToken = EventToken {
        from: "".to_string(),
        to: "".to_string(),
        amount: "".to_string(),
    };

    if let (
        Value::Address(from_address),
        Value::Address(to_address),
        Value::Uint(token_amount, 256),
    ) = (
        &decoded_data[0].value,
        &decoded_data[1].value,
        &decoded_data[2].value,
    ) {
        event_token.from = format!("{:#x}", from_address);
        event_token.to = format!("{:#x}", to_address);
        event_token.amount = token_amount.to_string();
    }

    return event_token;
}

// function for create new burn results file, filename is current timestamp
fn create_burn_results_file() -> File {
    let filename = format!("burn_results/{}.json", chrono::Local::now().timestamp());

    // add BurnResult to file
    let mut file = File::create(filename).unwrap();
    let burn_result = BurnResult {
        users: vec![],
        sent: false,
        tx_hash: "".to_string(),
    };
    file.write_all(serde_json::to_string(&burn_result).unwrap().as_bytes())
        .unwrap();
    file
}

// function for get number of burn results files
fn get_burn_results_folder_files_number() -> u32 {
    let path = Path::new("burn_results");
    let mut count = 0;
    if path.exists() {
        for _entry in fs::read_dir(path).unwrap() {
            count += 1;
        }
    }
    return count;
}

// function for check if burn results file is sent
fn is_burn_results_file_sent(file: &mut File) -> bool {
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let burn_result: BurnResult = serde_json::from_str(&contents).unwrap();
    return burn_result.sent;
}

// function for get list of burn results files
fn get_burn_results_files() -> Vec<File> {
    let path = Path::new("burn_results");
    let mut files: Vec<File> = Vec::new();
    if path.exists() {
        for entry in fs::read_dir(path).unwrap() {
            let file = File::open(entry.unwrap().path()).unwrap();
            files.push(file);
        }
    }
    // sort files by name
    files.sort_by(|a, b| {
        a.metadata()
            .unwrap()
            .created()
            .unwrap()
            .cmp(&b.metadata().unwrap().created().unwrap())
    });
    return files;
}

// function for load abi from json file
fn load_abi_from_json(filename: &str) -> Abi {
    let abi: Abi = {
        let file = File::open(filename).expect("failed to open ABI file");
        serde_json::from_reader(file).expect("failed to parse ABI")
    };
    abi
}

pub async fn get_env_variables() -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    u64,
    u64,
) {
    let pulse_doge_manager_pk = env::var("PULSE_DOGE_MANAGER_PK").unwrap();
    let pulse_doge_manager_address = env::var("PULSE_DOGE_MANAGER_ADDRESS").unwrap();
    let bsc_token_address = env::var("BSC_TOKEN_ADDRESS").unwrap();
    let eth_token_address = env::var("ETH_TOKEN_ADDRESS").unwrap();
    let bsc_rpc_url = env::var("BSC_RPC_URL").unwrap();
    let bsc_ws_url = env::var("BSC_WS_URL").unwrap();
    let eth_rpc_url = env::var("ETH_RPC_URL").unwrap();
    let eth_ws_url = env::var("ETH_WS_URL").unwrap();
    let explorer_url = env::var("EXPLORER_URL").unwrap();
    let airdrop_users_number = env::var("AIRDROP_USERS_NUMBER")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let period_minutes = env::var("PERIOD_MINUTES").unwrap().parse::<u64>().unwrap();

    println!("--- ENVIRONMENT VARIABLES ---");
    println!("PULSE_DOGE_MANAGER_PK {:?}", pulse_doge_manager_pk);
    println!("BSC_TOKEN_ADDRESS {:?}", bsc_token_address);
    println!("ETH_TOKEN_ADDRESS {:?}", eth_token_address);
    println!("BSC_RPC_URL {:?}", bsc_rpc_url);
    println!("BSC_WS_URL {:?}", bsc_ws_url);
    println!("ETH_RPC_URL {:?}", eth_rpc_url);
    println!("ETH_WS_URL {:?}", eth_ws_url);
    println!("EXPLORER_URL {:?}", explorer_url);
    println!("AIRDROP_USERS_NUMBER {:?}", airdrop_users_number);
    println!("PERIOD_MINUTES {:?}", period_minutes);
    println!("--- ENVIRONMENT VARIABLES ---\n");

    return (
        pulse_doge_manager_pk,
        pulse_doge_manager_address,
        bsc_token_address,
        eth_token_address,
        bsc_rpc_url,
        bsc_ws_url,
        eth_rpc_url,
        eth_ws_url,
        explorer_url,
        airdrop_users_number,
        period_minutes,
    );
}
