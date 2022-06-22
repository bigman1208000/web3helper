use colored::Colorize;
use std::env;
use web3::types::H160;
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let mut has_liquidity: bool = false;
    let trading_active: bool = false;
    let buy_tx_ok: bool = false;

    let mut web3m: Web3Manager = init_web3_connection().await;

    web3m
        .load_account(
            &env::var("ACCOUNT_ADDRESS").unwrap(),
            &env::var("PRIVATE_TEST_KEY").unwrap(),
        )
        .await;

    let account: H160 = web3m.first_loaded_account();

    let token_address = "0x3bF5f072Cd559244fD0fb288E401230b129B57A0";
    let token_lp_address = "0x7B2B8f2C5dd4449D54a03CcF316462F15d56aA27";
    let router_address = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";

    let router_instance = web3m.init_router().await;
    let factory_instance = web3m.init_router_factory().await;
    //    let lp_pair_instance = web3m.init_pair(token_lp_address).await;
    let mut slippage = 0usize;

    let value = "10000000000000";
    //println!("value: {:?}", wei_to_eth(value));

    let path_address: Vec<&str> = vec![
        "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // BNB
        token_address,
    ];

    /*
    let path_address: Vec<&str> = vec![
        "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd", // WAVAX
        "0x7ef95a0FEE0Dd31b22626fA2e10Ee6A223F8a684", // TOKEN
    ];
    */

    while !has_liquidity {
        // CHECK LIQUIDITY
        let lp_pair_instance = web3m.init_pair(token_lp_address).await;
        has_liquidity = web3m.token_has_liquidity(lp_pair_instance).await;
        println!("has_liquidity: {:?}", has_liquidity);

        if has_liquidity {
            println!("{}", "has_liquidity".green());

            while !buy_tx_ok {
                //println!("trying buy");
                //println!("slippage: {:?}", slippage);
                println!("{}", "trying buy...".yellow());
                let tx_result = web3m
                    .swap_eth_for_exact_tokens(
                        account,
                        router_address,
                        value,
                        &path_address,
                        slippage,
                    )
                    .await;

                if tx_result.is_ok() {
                    println!("{}", "Buy Tx Completed Successfully".green());
                    println!("tx_id: {:?}", tx_result.unwrap());
                } else {
                    println!("TradingNotEnabled");
                    slippage += 1;
                }
            }
        }
    }
    Ok(())
}

async fn init_web3_connection() -> Web3Manager {
    let web3_http_url = "https://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet";
    let web3_websocket_url =
        "wss://speedy-nodes-nyc.moralis.io/84a2745d907034e6d388f8d6/bsc/testnet/ws";
    let web3m: Web3Manager = Web3Manager::new(web3_http_url, web3_websocket_url, 97).await;
    web3m
}
