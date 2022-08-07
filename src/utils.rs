use chrono;
use chrono::{Timelike, Utc};
use colored::Colorize;
use core::time;
use std::env;
use std::process::exit;
use std::str::FromStr;
use std::thread;
use web3::contract::Contract;
use web3::helpers as w3h;
use web3::transports::Http;
use web3::types::{Address, H160, H256, U256};
use web3_rust_wrapper::Web3Manager;

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

pub fn open_tx_on_browser(tx_result: Result<H256, web3::Error>) {
    let mut tx_url: String = "https://testnet.bscscan.com/tx/".to_owned();
    tx_url.push_str(
        w3h::to_string(&tx_result.unwrap())
            .replace("\"", "")
            .as_str(),
    );

    if webbrowser::open(tx_url.as_str()).is_ok() {
        // ...
    }
}

pub async fn check_has_liquidity(web3m: &mut Web3Manager, token_lp_address: &str) -> bool {
    let mut has_liquidity: bool = false;
    while !has_liquidity {
        let lp_pair_instance = web3m.init_pair(token_lp_address).await;
        has_liquidity = web3m.token_has_liquidity(lp_pair_instance).await;

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        println!(
            "{}{:02}:{:02}:{:02}{}{}{}{}{}",
            "[".yellow(),
            hour.to_string().cyan(),
            now.minute().to_string().cyan(),
            now.second().to_string().cyan(),
            "]".yellow(),
            "[".yellow(),
            "HAS LIQUIDITY".cyan(),
            "]".yellow(),
            has_liquidity,
        );
    }
    has_liquidity
}

pub async fn get_token_price_info(
    web3m: &mut Web3Manager,
    router_address: &str,
    token_address: &str,
    buy_price: U256,
) -> (U256, f64) {
    let token_price = get_token_price(web3m, router_address, token_address).await;
    let price_change_percent =
        calc_price_change_percent(web3m.wei_to_eth(buy_price), web3m.wei_to_eth(token_price));

    (token_price, price_change_percent)
}

pub async fn hit_take_profit_or_stop_loss(
    price_change_percent: f64,
    take_profit_pencent: f64,
    stop_loss_percent: f64,
) -> (bool, bool) {
    (
        price_change_percent > take_profit_pencent,
        price_change_percent < stop_loss_percent,
    )
}

pub async fn get_token_price(
    web3m: &mut Web3Manager,
    router_address: &str,
    token_address: &str,
) -> U256 {
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

pub async fn check_before_buy(
    web3m: &mut Web3Manager,
    account: H160,
    token_address: &str,
    token_lp_address: &str,
) {
    // 1. CHECK IF TOKEN HAS LIQUIDITY
    check_has_liquidity(web3m, token_lp_address).await;

    // 2. CHECK TRADING ENABLE
    check_trading_enable(web3m, account, token_address).await;

    // 3. CHECK HONEYPOT
    check_honeypot(web3m, account, token_address).await;
}

pub async fn check_trading_enable(
    web3m: &mut Web3Manager,
    account: H160,
    token_address: &str,
) -> bool {
    let mut is_enabled: bool = false;

    let mut slippage = 1usize;
    let max_slippage = 25usize;

    while !is_enabled {
        let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";

        if slippage <= max_slippage {
            let tx_result = web3m
                .swap_eth_for_exact_tokens(
                    account,
                    router_address,
                    token_address,
                    U256::from_str("1000000000").unwrap(), // try buy 10 GWei 10000000000 -> 0.00000001 BNB
                    slippage,
                )
                .await;

            if tx_result.is_ok() {
                is_enabled = true;
                println!("{}", "BUY OK".green());
                let token_balance = web3m.get_token_balance(token_address, account).await;
                println!("Token Balance {}", web3m.wei_to_eth(token_balance));
            } else {
                println!("{}", tx_result.err().unwrap().to_string().red());
                slippage += 1;

                if slippage == max_slippage {
                    println!("{}", "Max slipagge".red());
                    exit(0);
                }
            }

            let now = Utc::now();
            let (is_pm, hour) = now.hour12();

            println!(
                "{}{:02}:{:02}:{:02}{}{}{}{}{} slippage {}",
                "[".yellow(),
                hour.to_string().cyan(),
                now.minute().to_string().cyan(),
                now.second().to_string().cyan(),
                "]".yellow(),
                "[".yellow(),
                "TRADING ACTIVE".cyan(),
                "]".yellow(),
                is_enabled,
                slippage
            );

            //let ten_millis = time::Duration::from_secs(1);
            //thread::sleep(ten_millis);
        }
    }
    is_enabled
}

pub async fn do_real_sell(
    web3m: &mut Web3Manager,
    account: H160,
    token_address: &str,
    router_address: &str,
    take_profit_pencent: f64,
    stop_loss_percent: f64,
    buy_price: U256,
) -> bool {
    let mut sell_tx_ok: bool = false;

    let token_balance = web3m.get_token_balance(token_address, account).await;
    println!("Token Balance {}", web3m.wei_to_eth(token_balance));
    let (mut last_token_price, price_change_percent) =
        get_token_price_info(web3m, router_address, token_address, buy_price).await;
    let mut price_hit_take_profit_ath = false;

    while !sell_tx_ok {
        clear_screen();

        // GET TOKEN PRICE AND CHANGE PERCENTAGE
        let (token_price, price_change_percent) =
            get_token_price_info(web3m, router_address, token_address, buy_price).await;

        let ath_price_change_percent = calc_price_change_percent(
            web3m.wei_to_eth(last_token_price),
            web3m.wei_to_eth(token_price),
        );

        if token_price > last_token_price {
            last_token_price = token_price;
        }

        if ath_price_change_percent > 10.0 {
            price_hit_take_profit_ath = true
        }

        // CHECK IF TOKEN PERCENT HITS TAKE PROFIT OR STOP LOSS
        let (price_hit_take_profit, price_hit_stop_loss) = hit_take_profit_or_stop_loss(
            price_change_percent,
            take_profit_pencent,
            stop_loss_percent,
        )
        .await;

        // LOG
        let now = Utc::now();
        let (is_pm, hour) = now.hour12();
        println!("");
        println!(
            "[{:02}:{:02}:{:02}] - Price: {} BNB | Change: {}",
            hour,
            now.minute(),
            now.second(),
            web3m.wei_to_eth(token_price),
            price_change_percent
        );

        // STOP ATH
        if price_hit_take_profit_ath {
            println!("{}", "TAKE PROFIT".green());
            sell_all(web3m, account, token_address).await;
            println!("take_profit_price: {:?}", token_price);
            sell_tx_ok = true;
        }

        /*
        // TAKE PROFIT LOSS
        if price_hit_take_profit {
            println!("{}", "TAKE PROFIT".green());
            sell_all(web3m, account, token_address).await;
            println!("take_profit_price: {:?}", token_price);
            sell_tx_ok = true;
        }
        */

        // STOP LOSS
        if price_hit_stop_loss {
            println!("{}", "STOP LOSS".red());
            sell_all(web3m, account, token_address).await;
            println!("sell_stop_loss_price: {:?}", token_price);
            sell_tx_ok = true;
        }
    }
    true
}

pub async fn do_real_buy(web3m: &mut Web3Manager, account: H160, token_address: &str) -> U256 {
    let mut is_enabled: bool = false;
    let mut buy_price = U256::from_str("0").unwrap();
    while !is_enabled {
        let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
        buy_price = get_token_price(web3m, router_address, token_address).await;
        let tx_result = web3m
            .clone()
            .swap_eth_for_exact_tokens(
                account,
                router_address,
                token_address,
                U256::from_str(env::var("INVEST_AMOUNT").unwrap().as_str()).unwrap(), // try buy 1GWei 1000000000 -> 0.000000001 BNB
                1usize,
            )
            .await;

        if tx_result.is_ok() {
            is_enabled = true;
            let token_balance = web3m.get_token_balance(token_address, account).await;
            println!("Token Balance {}", web3m.wei_to_eth(token_balance));
        } else {
            println!("{}", tx_result.err().unwrap().to_string().red());
        }

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        println!(
            "{}{:02}:{:02}:{:02}{}{}{}{}{}",
            "[".yellow(),
            hour.to_string().cyan(),
            now.minute().to_string().cyan(),
            now.second().to_string().cyan(),
            "]".yellow(),
            "[".yellow(),
            "TRADING ACTIVE".cyan(),
            "]".yellow(),
            is_enabled,
        );
    }
    buy_price
}

pub async fn check_honeypot(web3m: &mut Web3Manager, account: H160, token_address: &str) -> bool {
    let mut is_honey_pot: bool = true;
    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
    do_approve(web3m.clone(), token_address, router_address, account).await;

    while is_honey_pot {
        let token_balance = web3m.get_token_balance(token_address, account).await;
        println!("Token Balance {}", web3m.wei_to_eth(token_balance));

        let path_address: Vec<&str> = vec![
            token_address,
            "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // BNB
        ];

        let tx_result = web3m
            .clone()
            .swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
                account,
                router_address,
                token_balance, // 1 token,
                &path_address,
            )
            .await;

        if tx_result.is_ok() {
            println!("{}", "Buy Tx Completed Successfully".green());

            is_honey_pot = false;
        } else {
            println!("{}", tx_result.err().unwrap().to_string().red());
        }

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        println!(
            "{}{:02}:{:02}:{:02}{}{}{}{}{}",
            "[".yellow(),
            hour.to_string().cyan(),
            now.minute().to_string().cyan(),
            now.second().to_string().cyan(),
            "]".yellow(),
            "[".yellow(),
            "IS_HONEYPOT".cyan(),
            "]".yellow(),
            is_honey_pot,
        );

        let ten_millis = time::Duration::from_secs(1);
        thread::sleep(ten_millis);
    }
    is_honey_pot
}

pub async fn sell_all(web3m: &mut Web3Manager, account: H160, token_address: &str) {
    let mut sell_ok: bool = false;
    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
    do_approve(web3m.clone(), token_address, router_address, account).await;

    while !sell_ok {
        let token_balance = web3m.get_token_balance(token_address, account).await;
        println!("Token Balance {}", web3m.wei_to_eth(token_balance));

        let path_address: Vec<&str> = vec![
            token_address,
            "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // BNB
        ];

        let tx_result = web3m
            .clone()
            .swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
                account,
                router_address,
                token_balance,
                &path_address,
            )
            .await;

        if tx_result.is_ok() {
            println!("{}", "Sell Tx Completed Successfully".green());

            sell_ok = true;
        } else {
            println!("{}", tx_result.err().unwrap().to_string().red());
        }
    }
}

fn print_welcome() {
    println!("{}", "Welcome".green());
}

pub async fn init_web3_connection() -> Web3Manager {
    let web3_http_url = "http://127.0.0.1:8545";
    let web3_websocket_url = "ws://127.0.0.1:8545/ws";
    let chain_id = 31337;

    //let web3_http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
    //let web3_websocket_url =
    //    "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
    //let chain_id = 97;

    let mut web3m: Web3Manager =
        Web3Manager::new(web3_http_url, web3_websocket_url, chain_id).await;

    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;
    web3m
}

pub async fn get_env_variables() -> (String, U256, String, String) {
    let token_address = env::var("TOKEN_ADDRESS").unwrap();
    let value = U256::from_str(env::var("INVEST_AMOUNT").unwrap().as_str()).unwrap();
    let account_puk = env::var("ACCOUNT_ADDRESS").unwrap();
    let account_prk = env::var("PRIVATE_TEST_KEY").unwrap();

    println!("");
    println!("token_address {}", token_address.yellow());

    println!("value {}", value);
    println!("account_puk {}", account_puk);
    println!("account_prk {}", account_prk);
    println!("");

    return (token_address, value, account_puk, account_prk);
}

pub fn calc_price_change_percent(old_price: f64, new_price: f64) -> f64 {
    return -1.0 * ((old_price - new_price) / new_price * 100.0);
}

pub async fn do_approve(
    web3m: Web3Manager,
    token_address: &str,
    router_address: &str,
    account: H160,
) {
    let approve_tx = web3m
        .clone()
        .approve_erc20_token(
            account,
            token_address,
            router_address,
            "1000000000000000000000000000000",
        )
        .await
        .unwrap();
    println!("approve_tx {:?}", approve_tx);
}
