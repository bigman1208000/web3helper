use colored::Colorize;
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::env;
use std::future::Future;
use std::ops::Div;
use std::process;
use std::ptr::null;
use std::str::FromStr;
use std::time::Instant;
use std::time::SystemTime;
use std::{thread, time::Duration};
use web3::api::Eth;
use web3::contract::tokens::{Detokenize, Tokenizable, Tokenize};
use web3::contract::{Contract, Options};
use web3::ethabi::ethereum_types::H256;
use web3::ethabi::Uint;
use web3::futures::future::ok;
use web3::transports::{Http, WebSocket};
use web3::types::{
    Address, BlockNumber, Bytes, SignedTransaction, TransactionParameters, TransactionRequest,
    H160, U256, U64,
};
use web3::{Error, Web3};

trait InstanceOf
where
    Self: Any,
{
    fn instance_of<U: ?Sized + Any>(&self) -> bool {
        TypeId::of::<Self>() == TypeId::of::<U>()
    }
}

// implement this trait for every type that implements `Any` (which is most types)
impl<T: ?Sized + Any> InstanceOf for T {}

#[derive(Clone)]
pub struct Web3Manager {
    accounts: Vec<H160>,
    // public addressess
    pub web3http: Web3<Http>,
    // web3 https instance (for use call or write contract functions)
    pub web3WebSocket: Web3<WebSocket>,
    // web3 websocket instance (for listen contracts events)
    accounts_map: HashMap<H160, SecretKey>,
    // hashmap (like mapping on solidity) for store public and private keys
    current_nonce: U256,
    current_gas_price: U256,
    chain_id: Option<u64>,
}

impl Web3Manager {
    /*
    pub async fn instance_contract(&mut self, plain_contract_address: &str, abi_path: &str) -> Contract<Http> {
        let contract_instance: Contract<Http>
            = Contract::from_json(self.web3s.eth(),
                                  Address::from_str(plain_contract_address).unwrap(),
                                  include_bytes!("{}", abi_path)).unwrap();

        return contract_instance;
    }
    */

    pub fn generate_deadline(&self) -> Uint {
        U256::from(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }

    pub async fn swap_erc20_token(
        &mut self,
        contract_instance: Contract<Http>,
        value: &str,
        pairA: &str,
        pairB: &str,
        to: &str,
    ) -> H256 {
        let contract_function = "swapTokensForTokens".to_string();
        let deadline = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let tokenA: Address = Address::from_str(pairA).unwrap();
        let tokenB: Address = Address::from_str(pairB).unwrap();

        let recipient_address: Address = Address::from_str(to).unwrap();
        let valueA = Uint::from_dec_str(value).unwrap();
        let valueB = Uint::from_dec_str(value).unwrap();
        let parameters = (
            valueA,
            valueB,
            Uint::from_dec_str(value).unwrap(),
            vec![tokenA.to_string(), tokenB.to_string()],
            recipient_address,
            deadline + 300,
        );

        // let contract_function_parameters = (
        //     U256::from_dec_str(a).unwrap(),
        //     U256::from_dec_str(valueB).unwrap(),
        //     vec![pairA.to_string(), pairB.to_string()],
        //     recipient_address,
        //     deadline + 300,
        // );
        // self.sign_and_send_tx(contract_instance, contract_function, parameters)
        // .await
        // return result;
        return H256::from_str("1").unwrap();
    }

    pub async fn get_out_estimated_tokens_for_tokens(
        &mut self,
        contract_instance: Contract<Http>,
        pairA: &str,
        pairB: &str,
        amount: &str,
    ) -> U256 {
        let estimimated_out_amount: Uint = self
            .query_contract(
                contract_instance.clone(),
                "getAmountsOut",
                (
                    amount.to_string(),
                    vec![pairA.to_string(), pairB.to_string()],
                ),
            )
            .await;

        return estimimated_out_amount;
    }

    pub async fn get_token_balances(&mut self) -> U256 {
        return U256::from(0);
    }

    pub async fn get_nonce(&mut self) -> U256 {
        /*
        let block: Option<BlockNumber> = BlockNumber::Pending.into();

        let nonce: U256 = self.web3http
            .eth()
            .transaction_count(self.accounts[0], block)
            .await
            .unwrap();
        */

        let nonce: U256 = self
            .web3http
            .eth()
            .transaction_count(self.get_account(), None)
            .await
            .unwrap();

        return nonce;
    }

    /*
    pub async fn query_contract<R, P, T>(&mut self, func: &str, contract: Contract<Http>, params: P) -> web3::contract::Result<R>
        where
            R: Detokenize,
            P: Tokenize {
        /*
            let balance_of: Uint = contract_instance.query("balanceOf", web3m.get_account(), None, Options::default(), None).await.unwrap();
    println!("balance_of: {}", balance_of);
         */

        //println!("msg is ComplexMessage: {}", params.instance_of::<bool>());

        let res = contract
            .query(func, params, self.accounts[0], Options::default(), None)
            .await;

        return res;
    }
    */

    pub async fn load_accounts(
        &mut self,
        plain_address: &str,
        plain_private_key: &str,
    ) -> &mut Web3Manager {
        let private_key: SecretKey = SecretKey::from_str(plain_private_key).unwrap();
        let wallet: H160 = H160::from_str(plain_address).unwrap();

        self.accounts_map.insert(wallet, private_key);
        self.accounts.push(wallet);

        let nonce: U256 = self.get_nonce().await;
        self.current_nonce = nonce;

        let gas_price: U256 = self.web3http.eth().gas_price().await.unwrap();
        self.current_gas_price = gas_price;

        println!("wallet: {:?}", wallet);
        return self;
    }

    pub fn get_accounts(&mut self) -> &mut Web3Manager {
        //let keys = self.accountss.into_keys();

        //println!("keysd: {:?}", keysd);
        return self;
    }

    pub fn load_account(
        &mut self,
        plain_address: &str,
        plain_private_key: &str,
    ) -> &mut Web3Manager {
        //let account: Address = Address::from_str(plain_address).unwrap();

        self.accounts.push(H160::from_str(plain_address).unwrap());

        //let account: Address = Address::from_str("0xB06a4327FF7dB3D82b51bbD692063E9a180b79D9").unwrap(); // test

        //self.accounts.push(account);

        println!("self.accounts: {:?}", self.accounts);
        return self;
    }

    pub async fn new(httpUrl: &str, websocketUrl: &str) -> Web3Manager {
        // init web3 http connection
        let web3http: Web3<Http> = web3::Web3::new(web3::transports::Http::new(httpUrl).unwrap());

        // init web3 ws connection
        let web3WebSocket: Web3<WebSocket> = web3::Web3::new(
            web3::transports::WebSocket::new(websocketUrl)
                .await
                .unwrap(),
        );

        // create empty vector for store accounts
        let accounts: Vec<Address> = vec![];
        let accounts_map: HashMap<H160, SecretKey> = HashMap::new();

        let current_nonce: U256 = U256::from(0);
        let current_gas_price: U256 = U256::from(0);

        let chain_id: Option<u64> =
            Option::Some(u64::try_from(web3http.eth().chain_id().await.unwrap()).unwrap());

        return Web3Manager {
            accounts,
            web3http,
            web3WebSocket,
            accounts_map,
            current_nonce,
            current_gas_price,
            chain_id,
        };
    }

    pub async fn gas_price(&mut self) -> U256 {
        return self.web3http.eth().gas_price().await.unwrap();
    }

    pub async fn get_block(&mut self) -> U64 {
        let result: U64 = self.web3http.eth().block_number().await.unwrap();
        return result;
    }

    pub async fn query_contract<P, T>(
        &mut self,
        contract_instance: Contract<Http>,
        func: &str,
        params: P,
    ) -> T
    where
        P: Tokenize,
        T: Tokenizable,
    {
        // query contract
        let query_result: T = contract_instance
            .query(func, params, None, Options::default(), None)
            .await
            .unwrap();
        return query_result;
    }

    /*
    pub fn instance_contract(&mut self, contract_address: &str) -> Contract<Http> {
        let contract_instance: Contract<Http>
            = Contract::from_json(self.web3http.eth(),
                                  Address::from_str(contract_address).unwrap(),
                                  include_bytes!("../abi/TokenAbi.json")).unwrap();

        return contract_instance;
    }
    */

    pub async fn send_raw_transaction(&mut self, raw_transaction: Bytes) -> H256 {
        let result: H256 = self
            .web3http
            .eth()
            .send_raw_transaction(raw_transaction)
            .await
            .unwrap();
        return result;
    }

    pub async fn sign_transaction(
        &mut self,
        transact_obj: TransactionParameters,
    ) -> SignedTransaction {
        let private_key: secp256k1::SecretKey =
            SecretKey::from_str(&env::var("PRIVATE_TEST_KEY").unwrap()).unwrap();

        let signed_transaction: SignedTransaction = self
            .web3http
            .accounts()
            .sign_transaction(transact_obj, &private_key)
            .await
            .unwrap();
        return signed_transaction;
    }

    pub fn encode_tx_parameters(
        &mut self,
        nonce: U256,
        to: Address,
        value: U256,
        gas: U256,
        gas_price: U256,
        data: Bytes,
    ) -> TransactionParameters {
        let chain_id: Option<u64> = self.chain_id;

        let transact_obj = TransactionParameters {
            nonce: Some(nonce),
            to: Some(to),
            value,
            gas_price: Some(gas_price),
            gas,
            data,
            chain_id,
            ..Default::default()
        };

        return transact_obj;
    }

    pub fn encode_tx_data<P>(&mut self, contract: Contract<Http>, func: &str, params: P) -> Bytes
    where
        P: Tokenize,
    {
        let data = contract
            .abi()
            .function(func)
            .unwrap()
            .encode_input(&params.into_tokens())
            .unwrap();
        return data.into();
    }

    pub async fn estimate_tx_gas<P>(
        &mut self,
        contract: Contract<Http>,
        func: &str,
        params: P,
    ) -> U256
    where
        P: Tokenize,
    {
        let out_gas_estimate: U256 = contract
            .estimate_gas(
                func,
                params,
                self.accounts[0],
                Options {
                    value: Some(U256::from_dec_str("0").unwrap()),
                    gas: Some(U256::from_dec_str("8000000").unwrap()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        return out_gas_estimate;
    }

    pub fn get_account(&mut self) -> H160 {
        return self.accounts[0];
    }

    pub async fn approve_erc20_token(
        &mut self,
        contract_instance: Contract<Http>,
        spender: &str,
        value: &str,
    ) -> H256 {
        let spender_address: Address = Address::from_str(spender).unwrap();
        let contract_function = "approve";
        let contract_function_parameters = (spender_address, U256::from_dec_str(value).unwrap());

        let result: H256 = self
            .sign_and_send_tx(
                contract_instance,
                contract_function.to_string(),
                contract_function_parameters,
            )
            .await;
        return result;
    }

    // pub async fn swap_erc20_token(
    //     &mut self,
    //     contract_instance: Contract<Http>,
    //     valueA: &str,
    //     valueB: &str,
    //     pairA: &str,
    //     pairB: &str,
    //     to: &str,
    // ) -> H256 {
    //     let contract_function = "swapTokensForTokens";
    //     let deadline = SystemTime::now()
    //         .duration_since(SystemTime::UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs();
    //
    //     let recipient_address: Address = Address::from_str(to).unwrap();
    //     let contract_function_parameters = (
    //         U256::from_dec_str(valueA).unwrap(),
    //         U256::from_dec_str(valueB).unwrap(),
    //         vec![pairA, pairB],
    //         recipient_address,
    //         deadline + 300,
    //     );
    //
    //     let result: H256 = self
    //         .sign_and_send_tx(
    //             contract_instance,
    //             contract_function,
    //             contract_function_parameters,
    //         )
    //         .await;
    //     return result;
    //     //return H256::from_str("1").unwrap();
    // }

    // pub async fn get_out_estimated_tokens_for_tokens(
    //     &mut self,
    //     contract_instance: Contract<Http>,
    //     pairA: &str,
    //     pairB: &str,
    //     amount: &str,
    // ) {
    //     //let estimimated_out_amount: Uint = self.query_contract(contract_instance.clone(),"getAmountsOut",).await[0];
    //
    //     let tokenA: Address = Address::from_str(pairA).unwrap();
    //     let tokenB: Address = Address::from_str(pairB).unwrap();
    //
    //     let a: Uint = Uint::from_dec_str("100000000000000").unwrap();
    //
    //     let estimimated_out_amount: web3::contract::Result<Vec<String>> = self
    //         .query_contract(
    //             contract_instance.clone(),
    //             "getAmountsOut",
    //             (a, vec![tokenA, tokenB]),
    //         )
    //         .await;
    //
    //     println!("estimimated_out_amount: {:?}", estimimated_out_amount);
    //     //return H256::from_dec_str("10000000").unwrap();
    // }

    /*

    pub async fn approve(&mut self, contract_instance: Contract<Http>, from: &str, spender: &str, value: &str) -> H256 {

        let from_address: Address = Address::from_str(from).unwrap();
        let spender_address: Address = Address::from_str(spender).unwrap();


        let contract_function = "approve";
        let contract_function_parameters =
            (from_address, spender_address, U256::from_dec_str(value).unwrap());

    }

    */

    pub async fn sign_and_send_tx<P: Clone>(
        &mut self,
        contract_instance: Contract<Http>,
        func: String,
        params: P,
    ) -> H256
    where
        P: Tokenize + Copy,
    {
        // estimate gas for call this function with this parameters
        // increase 200ms execution time, we use high gas available
        // gas not used goes back to contract
        let estimated_tx_gas: U256 = self
            .estimate_tx_gas(contract_instance.clone(), &func, params)
            .await;

        /*
        let estimated_tx_gas: U256 = U256::from_dec_str("8000000").unwrap();
        */

        let tx_data: Bytes = self.encode_tx_data(contract_instance.clone(), &func, params.clone());

        // build tx parameters
        let tx_parameters: TransactionParameters = self.encode_tx_parameters(
            self.current_nonce,
            contract_instance.address(),
            U256::from_dec_str("0").unwrap(),
            estimated_tx_gas,
            self.current_gas_price,
            tx_data,
        );

        // sign tx
        let signed_transaction: SignedTransaction = self.sign_transaction(tx_parameters).await;

        // send tx
        let result: H256 = self
            .web3http
            .eth()
            .send_raw_transaction(signed_transaction.raw_transaction)
            .await
            .unwrap();

        println!(
            "Transaction successful with hash: {}{:?}",
            &env::var("EXPLORER").unwrap(),
            result
        );
        self.current_nonce = self.current_nonce + 1;
        return result;
    }

    pub async fn sent_erc20_token(
        &mut self,
        contract_instance: Contract<Http>,
        to: &str,
        value: &str,
    ) -> H256 {
        let contract_function = "transfer";

        let recipient_address: Address = Address::from_str(to).unwrap();
        let contract_function_parameters = (recipient_address, U256::from_dec_str(value).unwrap());

        let result: H256 = self
            .sign_and_send_tx(
                contract_instance,
                contract_function.to_string(),
                contract_function_parameters,
            )
            .await;
        return result;
    }

    /*
        pub async fn send_eth(&mut self, to: &str, value: U256) -> H256 {

            let recipient_address: Address = Address::from_str(to).unwrap();

            let nonce:U256 = self.get_nonce().await;

            // Build the tx object
            let tx_object = TransactionRequest {
                from,
                to: Some(recipient_address),
                value: Some(U256::from(value)),
                ..Default::default()
            };

            // Send the tx to localhost
            let result = self.web3http.eth().send_transaction(tx_object).await.unwrap();



            // sign tx
            let signed_transaction: SignedTransaction = self.sign_transaction(tx_object).await;

            // send tx
            let result: H256 = self.web3http
                .eth()
                .send_raw_transaction(signed_transaction.raw_transaction)
                .await
                .unwrap();





            let gas_price: U256 = self.web3http.eth().gas_price().await.unwrap();


            // Insert the 20-byte "from" address in hex format (prefix with 0x)
            //let from = Address::from_str("0xC48ad5fd060e1400a41bcf51db755251AD5A2475").unwrap();
            let from = self.get_account();

            // Insert the 20-byte "to" address in hex format (prefix with 0x)
            //let to = Address::from_str(to).unwrap();

            // Build the tx object
            let tx_object = TransactionRequest {
                from,
                to: Some(recipient_address),
                value: Some(U256::from(value)),
                ..Default::default()
            };

            // Send the tx to localhost
            let result = self.web3http.eth().send_transaction(tx_object).await.unwrap();

            println!("Tx succeeded with hash: {}", result);


            println!("Transaction successful with hash: {}{:?}", &env::var("EXPLORER").unwrap(), result);
            return result;
        }
    }
    */
}

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    let res = res / 1_000_000_000_000_000_000.0;
    return res;
}

fn chunks(data: Vec<Uint>, chunk_size: usize) -> Vec<Vec<Uint>> {
    let mut results = vec![];
    let mut current = vec![];
    for i in data {
        if current.len() >= chunk_size {
            results.push(current);
            current = vec![];
        }
        current.push(i);
    }
    results.push(current);

    return results;
}
