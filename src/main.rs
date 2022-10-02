pub mod utils;
pub use utils::*;
use web3::types::H160;
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let (
        account_puk,
        account_prk,
        router_address,
        token_address,
        invest_amount,
        max_slipage,
        stop_loss,
        take_profit,
    ) = get_env_variables().await;

    let take_profit_pencent = 90.0;

    // INITIALIZE VALUES
    let mut web3m: Web3Manager =
        init_web3_connection(account_puk.as_str(), account_prk.as_str()).await;
    let account: H160 = web3m.first_loaded_account();

    let token_lp_address = web3m.find_lp_pair(token_address.as_str()).await;

    println!("token_address {}", token_address);
    println!("token_lp_address {}", token_lp_address);

    // 1. CHECK IF TOKEN HAS LIQUIDITY
    // 2. CHECK TRADING ENABLE
    // 3. CHECK HONEYPOT
    check_before_buy(
        &mut web3m,
        account,
        router_address.as_str(),
        token_address.as_str(),
        token_lp_address.as_str(),
    )
    .await;
    println!("invest_amount {}", invest_amount);
    // 4. DO REAL BUY
    let buy_price = do_real_buy(
        &mut web3m,
        account,
        router_address.as_str(),
        token_address.as_str(),
        invest_amount,
    )
    .await;
    clear_screen();

    let sell_tx_ok: bool = false;

    while !sell_tx_ok {
        let token_price =
            get_token_price(&mut web3m, router_address.as_str(), token_address.as_str()).await;
        println!("token_price {}", token_price);
        //let token_eth_price = wei_to_eth(token_price);

        /*
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        price_history.push((token_eth_price as f32, timestamp as f32));

        Chart::default()
            .lineplot(&Shape::Lines(&price_history))
            .display();
        clear_screen();
        */
    }

    // 5. LOOP UNTIL TAKE PROFIT OR STOP LOSS
    do_real_sell(
        &mut web3m,
        account,
        token_address.as_str(),
        router_address.as_str(),
        take_profit_pencent,
        stop_loss,
        buy_price,
    )
    .await;

    Ok(())
}
