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
use std::time::{SystemTime, SystemTimeError};
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
    // all the accounts
    accounts: Vec<H160>,
    // balnces of each accounts
    balances: HashMap<H160, U256>,
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
    pub async fn instance_contract(
        &self,
        plain_contract_address: &str,
        abi_path: &[u8],
    ) -> Result<Contract<Http>, Box<dyn std::error::Error>> {
        Ok(Contract::from_json(
            self.web3http.eth(),
            Address::from_str(plain_contract_address).unwrap(),
            abi_path,
        )?)
    }

    pub fn generate_deadline(&self) -> Result<U256, SystemTimeError> {
        Ok(U256::from(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs(),
        ))
    }

    // TODO(elsuizo:2022-03-03): documentation here
    pub async fn swap_eth_for_exact_tokens(
        &mut self,
        contract_instance: &Contract<Http>,
        token_amount: &str,
        pairs: &[&str],
    ) -> Result<H256, Box<dyn std::error::Error>> {
        let contract_function = "swapETHForExactTokens".to_string();
        let deadline = self.generate_deadline()?;

        let mut addresses: [H160; 2] = [H160::default(); 2];

        addresses[0] = Address::from_str(pairs[0])?;
        addresses[1] = Address::from_str(pairs[1])?;

        let amount_in = U256::from_dec_str(token_amount)?;
        let parameter_in = (amount_in, addresses);
        let amount_in_min: Vec<Uint> = self
            .query_contract(contract_instance, "getAmountsIn", parameter_in)
            .await;

        let amount_out: U256 = U256::from_dec_str(token_amount).unwrap();
        let parameter_out = (amount_out, addresses);
        let amount_out_min: Vec<Uint> = self
            .query_contract(contract_instance, "getAmountsOut", parameter_out)
            .await;

        let slipage = 2usize;

        let min_amount = U256::from(amount_out_min[1].as_u128());

        let min_amount_less_slipagge = min_amount - ((min_amount * slipage) / 100usize);

        let parameters2 = (
            min_amount_less_slipagge,
            addresses.clone(),
            self.get_first_loaded_account(),
            deadline + 600usize,
        );

        Ok(self
            .sign_and_send_tx(
                contract_instance.clone(),
                contract_function,
                &parameters2,
                &amount_out_min[0].to_string(),
            )
            .await)
    }

    pub async fn get_out_estimated_tokens_for_tokens(
        &self,
        contract_instance: &Contract<Http>,
        pair_a: &str,
        pair_b: &str,
        amount: &str,
    ) -> U256 {
        self.query_contract(
            contract_instance,
            "getAmountsOut",
            (
                amount.to_string(),
                vec![pair_a.to_string(), pair_b.to_string()],
            ),
        )
        .await
    }

    // TODO(elsuizo:2022-03-03): verify this method
    pub async fn set_token_balances(&mut self) {
        for account in &self.accounts {
            let balance = self.web3http.eth().balance(*account, None).await.unwrap();
            self.balances.insert(*account, balance);
        }
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
            .transaction_count(self.get_first_loaded_account(), None)
            .await
            .unwrap();

        return nonce;
    }

    pub async fn load_accounts(
        &mut self,
        plain_address: &str,
        plain_private_key: &str,
    ) -> &mut Web3Manager {
        // cast plain pk to sk type
        let private_key: SecretKey = SecretKey::from_str(plain_private_key).unwrap();
        let wallet: H160 = H160::from_str(plain_address).unwrap();

        // push on account list
        self.accounts_map.insert(wallet, private_key);
        self.accounts.push(wallet);

        // get last nonce from loaded account
        let nonce: U256 = self.get_nonce().await;
        self.current_nonce = nonce;

        let gas_price: U256 = self.web3http.eth().gas_price().await.unwrap();
        self.current_gas_price = gas_price;

        //println!("wallet: {:?}", wallet);
        return self;
    }

    pub fn get_accounts(&mut self) -> &mut Web3Manager {
        //let keys = self.accountss.into_keys();

        ////println!("keysd: {:?}", keysd);
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

        //println!("self.accounts: {:?}", self.accounts);
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
        let balances: HashMap<H160, U256> = HashMap::new();
        let accounts_map: HashMap<H160, SecretKey> = HashMap::new();

        let current_nonce: U256 = U256::from(0);
        let current_gas_price: U256 = U256::from(0);

        let chain_id: Option<u64> =
            Option::Some(u64::try_from(web3http.eth().chain_id().await.unwrap()).unwrap());

        return Web3Manager {
            accounts,
            balances,
            web3http,
            web3WebSocket,
            accounts_map,
            current_nonce,
            current_gas_price,
            chain_id,
        };
    }

    pub async fn gas_price(&self) -> U256 {
        return self.web3http.eth().gas_price().await.unwrap();
    }

    pub async fn get_block(&self) -> U64 {
        let result: U64 = self.web3http.eth().block_number().await.unwrap();
        return result;
    }

    pub async fn query_contract<P, T>(
        &self,
        contract_instance: &Contract<Http>,
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

    pub async fn send_raw_transaction(&mut self, raw_transaction: Bytes) -> H256 {
        let result: H256 = self
            .web3http
            .eth()
            .send_raw_transaction(raw_transaction)
            .await
            .unwrap();
        return result;
    }

    pub async fn sign_transaction(&self, transact_obj: TransactionParameters) -> SignedTransaction {
        let private_key = SecretKey::from_str(&env::var("PRIVATE_TEST_KEY").unwrap()).unwrap();

        self.web3http
            .accounts()
            .sign_transaction(transact_obj, &private_key)
            .await
            .unwrap()
    }

    pub fn encode_tx_parameters(
        &self,
        nonce: U256,
        to: Address,
        value: U256,
        gas: U256,
        gas_price: U256,
        data: Bytes,
    ) -> TransactionParameters {
        TransactionParameters {
            nonce: Some(nonce),
            to: Some(to),
            value,
            gas_price: Some(gas_price),
            gas,
            data,
            chain_id: self.chain_id,
            ..Default::default()
        }
    }

    // TODO(elsuizo:2022-03-03): add a `Result` here
    pub fn encode_tx_data<P>(&self, contract: &Contract<Http>, func: &str, params: P) -> Bytes
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
        contract: &Contract<Http>,
        func: &str,
        params: P,
        value: &str,
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
                    value: Some(U256::from_dec_str(value).unwrap()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        return out_gas_estimate;
    }

    pub fn get_first_loaded_account(&mut self) -> H160 {
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
                &contract_function_parameters,
                "0",
            )
            .await;
        return result;
    }

    pub async fn sign_and_send_tx<P: Clone>(
        &mut self,
        contract_instance: Contract<Http>,
        func: String,
        params: &P,
        value: &str,
    ) -> H256
    where
        P: Tokenize,
    {
        /*
        // estimate gas for call this function with this parameters
        // increase 200ms execution time, we use high gas available
        // gas not used goes back to contract
        let estimated_tx_gas: U256 = self
            .estimate_tx_gas(contract_instance.clone(), &func, params.clone(), value)
            .await;
        */

        let estimated_tx_gas: U256 = U256::from_dec_str("5000000").unwrap();

        // 2. encode_tx_data
        let tx_data: Bytes = self.encode_tx_data(&contract_instance, &func, params.clone());

        // 3. build tx parameters
        let tx_parameters: TransactionParameters = self.encode_tx_parameters(
            self.current_nonce,
            contract_instance.address(),
            U256::from_dec_str(value).unwrap(),
            estimated_tx_gas,
            self.current_gas_price,
            tx_data,
        );

        // 4. sign tx
        let signed_transaction: SignedTransaction = self.sign_transaction(tx_parameters).await;

        // send tx
        let result: H256 = self
            .web3http
            .eth()
            .send_raw_transaction(signed_transaction.raw_transaction)
            .await
            .unwrap();

        /*
        println!(
            "Transaction successful with hash: {}{:?}",
            &env::var("EXPLORER").unwrap(),
            result
        );
        */
        self.current_nonce = self.current_nonce + 1; // todo, check pending nonce dont works
        return result;
    }

    pub async fn sent_erc20_token(
        &mut self,
        contract_instance: Contract<Http>,
        to: &str,
        tokenAmount: &str,
    ) -> H256 {
        let contract_function = "transfer";

        let recipient_address: Address = Address::from_str(to).unwrap();
        let contract_function_parameters =
            (recipient_address, U256::from_dec_str(tokenAmount).unwrap());

        let result: H256 = self
            .sign_and_send_tx(
                contract_instance,
                contract_function.to_string(),
                &contract_function_parameters,
                "0",
            )
            .await;
        return result;
    }
}

fn wei_to_eth(wei_val: U256) -> f64 {
    let res: f64 = wei_val.as_u128() as f64;
    let res: f64 = res / 1_000_000_000_000_000_000.0;
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
