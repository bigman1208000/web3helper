use chrono;
use colored::Colorize;
use std::env;
use std::process::exit;
use std::str::FromStr;
use web3::contract::Contract;
use web3::helpers as w3h;
use web3::transports::Http;
use web3::types::{Address, H160, U256};
use web3_rust_wrapper::Web3Manager;
//use textplots::{Chart, Plot, Shape};
use chrono::{Timelike, Utc};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    print_welcome();

    //let mut trading_active: bool = false;
    let mut buy_tx_ok: bool = false;
    let mut sell_tx_ok: bool = false;

    let mut web3m: Web3Manager = init_web3_connection().await;
    let account: H160 = web3m.first_loaded_account();

    let token_address = env::var("TOKEN_ADDRESS").unwrap();
    let token_lp_address = env::var("TOKEN_LP_ADDRESS").unwrap();
    let value = U256::from_str(env::var("INVEST_AMOUNT").unwrap().as_str()).unwrap();
    let max_slippage = 10usize;
    let mut slippage = 1usize;

    let mut buy_price = U256::from_str("0").unwrap();
    let take_profit_pencent = 0.4;
    let stop_loss_percent = -0.4;
    let mut token_balance: U256;

    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";

    //TOKEN_ADDRESS="0xB9708A726a6C997d5A90515246E52A5fE4791CBE"
    //TOKEN_LP_ADDRESS="0x8f7d1722Ceb8eA49ED1D081B27ebf21b61c316F2"

    println!("");
    println!("token_address {}", token_address.yellow());
    println!("token_lp_address {}", token_lp_address.yellow());
    println!("value {}", value);
    println!("slippage {}", slippage);
    println!("max_slippage {}", max_slippage);
    println!("");

    let token_abi = include_bytes!("../abi/TokenAbi.json");
    let token_instance: Contract<Http> = web3m
        .instance_contract(token_address.as_str(), token_abi)
        .await
        .expect("error creating the router instance");

    let approve_tx = web3m
        .approve_erc20_token(
            account,
            token_instance.clone(),
            router_address,
            "1000000000000000000000000000000",
        )
        .await
        .unwrap();
    println!("approve_tx {:?}", approve_tx);

    /*
    println!("y = sin(x) / x");
    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x.sin() / x)))
        .display();
        */

    println!("{}", "Checking Liquidity".yellow());
    check_has_liquidity(&web3m, token_lp_address.as_str()).await;

    while !buy_tx_ok && slippage < max_slippage {
        print!("{}[2J", 27 as char);
        let token_price = get_token_price(&web3m, router_address, token_address.as_str()).await;

        if slippage == max_slippage {
            println!("{}", "Max Slippage exceded".red());
        }

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        println!(
            "[{:02}:{:02}:{:02}] : {} {} with {}% slippage",
            hour,
            now.minute(),
            now.second(),
            wei_to_eth(token_price),
            "Trying buy ".yellow(),
            slippage
        );

        let path_address: Vec<&str> = vec![
            "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // BNB
            token_address.as_str(),
        ];

        let tx_result = web3m
            .swap_eth_for_exact_tokens(account, router_address, value, &path_address, slippage)
            .await;

        if tx_result.is_ok() {
            println!("{}", "Buy Tx Completed Successfully".green());
            buy_price = token_price;

            //println!("tx_id: {:?}", tx_result.unwrap());

            let mut tx_url: String = "https://testnet.bscscan.com/tx/".to_owned();
            tx_url.push_str(
                w3h::to_string(&tx_result.unwrap())
                    .replace("\"", "")
                    .as_str(),
            );

            if webbrowser::open(tx_url.as_str()).is_ok() {
                // ...
            }

            buy_tx_ok = true;
        } else {
            println!("{}", tx_result.err().unwrap().to_string().red());
            println!("{}", "Trading Not Enabled Or Slippage Too Low".red());
            slippage += 1;
        }
        println!("buy_price: {:?}", buy_price);
    }

    token_balance = web3m.get_token_balance(&token_instance, account).await;
    //println!("Token Balance {}", token_balance);

    while !sell_tx_ok {
        print!("{}[2J", 27 as char);
        let token_price = get_token_price(&web3m, router_address, token_address.as_str()).await;
        let price_change_percent =
            calc_price_change_percent(wei_to_eth(buy_price), wei_to_eth(token_price)).await;

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        println!(
            "[{:02}:{:02}:{:02}] - Price: {} BNB | Change: {}",
            hour,
            now.minute(),
            now.second(),
            wei_to_eth(token_price),
            price_change_percent
        );

        // TAKE PROFIT LOSS
        if price_change_percent > take_profit_pencent {
            println!("{}", "TAKE PROFIT".red());

            let path_address: Vec<&str> = vec![
                token_address.as_str(),
                "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // BNB
            ];

            let tx_result = web3m
                .swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
                    account,
                    router_address,
                    token_balance,
                    &path_address,
                )
                .await;

            if tx_result.is_ok() {
                println!("{}", "SELL Tx Completed Successfully".green());

                println!("Token Balance {}", token_balance);
                //println!("tx_id: {:?}", tx_result.unwrap());

                let mut tx_url: String = "https://testnet.bscscan.com/tx/".to_owned();
                tx_url.push_str(
                    w3h::to_string(&tx_result.unwrap())
                        .replace("\"", "")
                        .as_str(),
                );

                if webbrowser::open(tx_url.as_str()).is_ok() {
                    // ...
                }

                sell_tx_ok = true;
            } else {
                println!("{}", tx_result.err().unwrap().to_string().red());
            }
            println!("sell_stop_loss_price: {:?}", token_price);
        }

        // STOP LOSS
        if price_change_percent < stop_loss_percent {
            println!("{}", "STOP LOSS".red());

            let path_address: Vec<&str> = vec![
                token_address.as_str(),
                "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // BNB
            ];

            let tx_result = web3m
                .swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
                    account,
                    router_address,
                    token_balance,
                    &path_address,
                )
                .await;

            if tx_result.is_ok() {
                println!("{}", "SELL Tx Completed Successfully".green());

                //println!("tx_id: {:?}", tx_result.unwrap());

                let mut tx_url: String = "https://testnet.bscscan.com/tx/".to_owned();
                tx_url.push_str(
                    w3h::to_string(&tx_result.unwrap())
                        .replace("\"", "")
                        .as_str(),
                );

                if webbrowser::open(tx_url.as_str()).is_ok() {
                    // ...
                }

                sell_tx_ok = true;
            } else {
                println!("{}", tx_result.err().unwrap().to_string().red());
            }
            println!("sell_stop_loss_price: {:?}", token_price);
        }
    }

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

async fn calc_price_change_percent(old_price: f64, new_price: f64) -> f64 {
    return -1.0 * ((old_price - new_price) / new_price * 100.0);
}

async fn get_token_price(web3m: &Web3Manager, router_address: &str, token_address: &str) -> U256 {
    let path_address: Vec<&str> = vec![
        token_address,
        "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // BNB
    ];
    let mut addresses = Vec::new();
    for pair in path_address {
        addresses.push(Address::from_str(pair).unwrap());
    }

    let token_price: U256 = web3m
        .clone()
        .get_token_price(router_address, addresses)
        .await;

    token_price
}

async fn check_has_liquidity(web3m: &Web3Manager, token_lp_address: &str) -> bool {
    let mut has_liquidity: bool = false;
    while !has_liquidity {
        println!("{}", chrono::offset::Local::now());

        // CHECK LIQUIDITY
        let lp_pair_instance = web3m.init_pair(token_lp_address).await;
        has_liquidity = web3m.token_has_liquidity(lp_pair_instance).await;
        println!("has_liquidity: {:?}", has_liquidity);
        println!("");
    }
    println!("{}", "Has Liquidity".green());
    has_liquidity
}

fn print_welcome() {
    println!("{}", "Welcome".green());
}

async fn init_web3_connection() -> Web3Manager {
    let web3_http_url = "http://127.0.0.1:8545";
    let web3_websocket_url =
        "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 31337).await;

    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;
    web3m
}
