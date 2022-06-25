use chrono;
use chrono::{Timelike, Utc};
use colored::Colorize;
use core::time;
use std::env;
use std::process::exit;
use std::str::FromStr;
use std::thread;
use textplots::{Chart, Plot, Shape};
use web3::contract::Contract;
use web3::helpers as w3h;
use web3::transports::Http;
use web3::types::{Address, H160, H256, U256};
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";

    // INITIALIZE VALUES
    let (
        web3m,
        account,
        slippage,
        max_slippage,
        take_profit_pencent,
        stop_loss_percent,
        is_trading_active,
        token_address,
        token_lp_address,
        value,
        account_puk,
        account_prk,
    ) = initialize_values().await;

    let mut buy_tx_ok: bool = false;
    let mut sell_tx_ok: bool = false;

    // CHECK IF TOKEN HAS LIQUIDITY
    // CHECK TRADING ENABLE
    // CHECK HONEYPOT
    check_before_buy(
        &web3m,
        account,
        token_address.as_str(),
        token_lp_address.as_str(),
    )
    .await;

    // CHECK TRADING ENABLE
    //try_buy(&web3m, account, token_lp_address.as_str()).await;

    //exit(0);

    let buy_price = get_token_price(&web3m, router_address, token_address.as_str()).await;

    let token_balance = web3m
        .get_token_balance(token_address.as_str(), account)
        .await;
    println!("Token Balance {}", web3m.wei_to_eth(token_balance));

    while !sell_tx_ok {
        clear_screen();
        let token_price = get_token_price(&web3m, router_address, token_address.as_str()).await;
        let price_change_percent =
            calc_price_change_percent(web3m.wei_to_eth(buy_price), web3m.wei_to_eth(token_price));

        let now = Utc::now();
        let (is_pm, hour) = now.hour12();

        //let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0), (3.0, -2.0), (4.0, 1.0)];
        //Chart::default().lineplot(&Shape::Lines(&data)).display();

        println!("");
        println!(
            "[{:02}:{:02}:{:02}] - Price: {} BNB | Change: {}",
            hour,
            now.minute(),
            now.second(),
            web3m.wei_to_eth(token_price),
            price_change_percent
        );

        // TAKE PROFIT LOSS
        if price_change_percent > take_profit_pencent {
            println!("{}", "TAKE PROFIT".red());

            sell_all(&web3m, account, token_address.as_str()).await;

            println!("sell_stop_loss_price: {:?}", token_price);
        }

        // STOP LOSS
        if price_change_percent < stop_loss_percent {
            println!("{}", "STOP LOSS".red());

            sell_all(&web3m, account, token_address.as_str()).await;

            println!("sell_stop_loss_price: {:?}", token_price);
        }
    }

    Ok(())
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn open_tx_on_browser(tx_result: Result<H256, web3::Error>) {
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

async fn check_has_liquidity(web3m: &Web3Manager, token_lp_address: &str) -> bool {
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

async fn check_before_buy(
    web3m: &Web3Manager,
    account: H160,
    token_address: &str,
    token_lp_address: &str,
) {
    // CHECK IF TOKEN HAS LIQUIDITY
    println!("{}", "Checking Liquidity".yellow());
    check_has_liquidity(&web3m, token_lp_address).await;

    // CHECK TRADING ENABLE
    check_trading_enable(&web3m, account, token_address).await;

    // CHECK HONEYPOT
    try_sell(&web3m, account, token_address).await;
}

async fn check_trading_enable(web3m: &Web3Manager, account: H160, token_address: &str) -> bool {
    let mut is_enabled: bool = false;
    while !is_enabled {
        let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";

        let tx_result = web3m
            .clone()
            .swap_eth_for_exact_tokens(
                account,
                router_address,
                token_address,
                U256::from_str("1000000000").unwrap(), // try buy 1GWei 1000000000 -> 100000000000000000
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

        let ten_millis = time::Duration::from_secs(1);

        thread::sleep(ten_millis);
    }
    is_enabled
}

async fn do_real_buy(web3m: &Web3Manager, account: H160, token_address: &str) -> bool {
    let mut is_enabled: bool = false;
    while !is_enabled {
        let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";

        let tx_result = web3m
            .clone()
            .swap_eth_for_exact_tokens(
                account,
                router_address,
                token_address,
                U256::from_str("1000000000").unwrap(), // try buy 1GWei 1000000000 -> 100000000000000000
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

        let ten_millis = time::Duration::from_secs(1);

        thread::sleep(ten_millis);
    }
    is_enabled
}

async fn try_sell(web3m: &Web3Manager, account: H160, token_address: &str) -> bool {
    let mut buy_tx_ok: bool = false;
    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
    do_approve(web3m.clone(), token_address, router_address, account).await;

    while !buy_tx_ok {
        //let token_balance = web3m.get_token_balance(token_address, account).await;
        //println!("Token Balance {}", web3m.wei_to_eth(token_balance));

        let path_address: Vec<&str> = vec![
            token_address,
            "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // BNB
        ];

        let tx_result = web3m
            .clone()
            .swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
                account,
                router_address,
                U256::from_dec_str("1000000000000000000").unwrap(), // 1 token,
                &path_address,
            )
            .await;

        if tx_result.is_ok() {
            println!("{}", "Buy Tx Completed Successfully".green());

            buy_tx_ok = true;
        } else {
            println!("{}", tx_result.err().unwrap().to_string().red());
        }
    }
    println!("{}", "Has Liquidity".green());
    buy_tx_ok
}

async fn sell_all(web3m: &Web3Manager, account: H160, token_address: &str) {
    let mut sell_ok: bool = false;
    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
    do_approve(web3m.clone(), token_address, router_address, account).await;

    let mut tx_result: Result<H256, web3::Error>;

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

async fn init_web3_connection() -> Web3Manager {
    let web3_http_url = "http://127.0.0.1:8545";
    let web3_websocket_url = "ws://127.0.0.1:8545/ws";
    let mut web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 31337).await;

    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;
    web3m
}

async fn get_env_variables() -> (String, U256, String, String) {
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

async fn initialize_values() -> (
    Web3Manager,
    H160,
    usize,
    usize,
    f64,
    f64,
    bool,
    String,
    String,
    U256,
    String,
    String,
) {
    let mut web3m: Web3Manager = init_web3_connection().await;
    let account: H160 = web3m.first_loaded_account();

    //let mut trading_active: bool = false;

    let mut slippage = 1usize;
    let max_slippage = 25usize;

    let take_profit_pencent = 0.4;
    let stop_loss_percent = -0.4;
    let mut is_trading_active: bool = false;

    let (token_address, value, account_puk, account_prk) = get_env_variables().await;

    let token_lp_address = web3m.find_lp_pair(token_address.as_str()).await;

    return (
        web3m,
        account,
        slippage,
        max_slippage,
        take_profit_pencent,
        stop_loss_percent,
        is_trading_active,
        token_address,
        token_lp_address,
        value,
        account_puk,
        account_prk,
    );
}

fn calc_price_change_percent(old_price: f64, new_price: f64) -> f64 {
    return -1.0 * ((old_price - new_price) / new_price * 100.0);
}

async fn do_approve(web3m: Web3Manager, token_address: &str, router_address: &str, account: H160) {
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
