
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
        take_profit_percent,
    ) = get_env_variables().await;

    // INITIALIZE Web3Manager
    let mut web3m: Web3Manager = Web3Manager::new(web3_rust_wrapper::Network::BSCTestnet)
    .await;

    // INITIALIZE ACCOUNT
    web3m
        .load_account(account_puk.as_str(), account_prk.as_str())
        .await;
    let account: H160 = web3m.first_loaded_account();

    // 1. CHECK IF TOKEN HAS LIQUIDITY
    // 2. CHECK TRADING ENABLE
    // 3. CALC BUY SELL FEES
    // 4. CHECK HONEYPOT
    check_before_buy(
        &mut web3m,
        account,
        router_address.as_str(),
        token_address.as_str(),
    )
    .await;

    // 4. DO REAL BUY
    let buy_price = do_real_buy(
        &mut web3m,
        account,
        router_address.as_str(),
        token_address.as_str(),
        eth_to_wei(invest_amount, 18),
    )
    .await;
    clear_screen();

    // 5. LOOP UNTIL TAKE PROFIT OR STOP LOSS
    do_real_sell(
        &mut web3m,
        account,
        token_address.as_str(),
        router_address.as_str(),
        take_profit_percent,
        stop_loss,
        buy_price,
    )
    .await;

    Ok(())
}