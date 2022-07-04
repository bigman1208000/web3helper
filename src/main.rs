pub mod utils;
use std::time::{SystemTime, UNIX_EPOCH};
use textplots::{Chart, Plot, Shape};
pub use utils::*;
use web3::types::H160;
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let mut price_history: Vec<(f32, f32)> = Vec::new();

    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";

    // INITIALIZE VALUES
    let mut web3m: Web3Manager = init_web3_connection().await;
    let account: H160 = web3m.first_loaded_account();

    //let mut trading_active: bool = false;

    let mut slippage = 1usize;
    let max_slippage = 25usize;

    let take_profit_pencent = 90.0;
    let stop_loss_percent = -0.4;
    let mut is_trading_active: bool = false;

    let (token_address, value, account_puk, account_prk) = get_env_variables().await;

    let token_lp_address = web3m.find_lp_pair(token_address.as_str()).await;

    println!("token_address {}", token_address);
    println!("token_lp_address {}", token_lp_address);

    // 1. CHECK IF TOKEN HAS LIQUIDITY
    // 2. CHECK TRADING ENABLE
    // 3. CHECK HONEYPOT
    check_before_buy(
        &mut web3m,
        account,
        token_address.as_str(),
        token_lp_address.as_str(),
    )
    .await;

    // 4. DO REAL BUY
    let buy_price = do_real_buy(&mut web3m, account, token_address.as_str()).await;
    clear_screen();
    /*
    let sell_tx_ok: bool = false;

    while !sell_tx_ok {
        let token_price = get_token_price(&web3m, router_address, token_address.as_str()).await;
        let token_eth_price = web3m.wei_to_eth(token_price);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        price_history.push((token_eth_price as f32, timestamp as f32));

        Chart::default()
            .lineplot(&Shape::Lines(&price_history))
            .display();
        clear_screen();
    }
    */

    // 5. LOOP UNTIL TAKE PROFIT OR STOP LOSS
    do_real_sell(
        &mut web3m,
        account,
        token_address.as_str(),
        router_address,
        take_profit_pencent,
        stop_loss_percent,
        buy_price,
    )
    .await;

    Ok(())
}
