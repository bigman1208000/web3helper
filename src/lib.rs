use secp256k1::SecretKey;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::env;
use std::str::FromStr;
use std::time::{SystemTime, SystemTimeError};
use web3::contract::tokens::{Tokenizable, Tokenize};
use web3::contract::{Contract, Options};
use web3::ethabi::ethereum_types::H256;
use web3::ethabi::Uint;
use web3::transports::{Http, WebSocket};
use web3::types::{
    Address, Bytes, SignedTransaction, TransactionParameters,
    H160, U256, U64,
};
use web3::{Web3};
use hex_literal::hex;

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
    // balances of each accounts
    balances: HashMap<H160, U256>,
    // public addresses
    pub web3http: Web3<Http>,
    // web3 https instance (for use call or write contract functions)
    pub web3web_socket: Web3<WebSocket>,
    // web3 websocket instance (for listen contracts events)
    accounts_map: HashMap<H160, String>,
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
        account: H160,
        contract_instance: &Contract<Http>,
        token_amount: &str,
        pairs: &[&str],
    ) -> Result<H256, Box<dyn std::error::Error>> {
        let contract_function = "swapETHForExactTokens".to_string();
        let deadline = self.generate_deadline()?;

        let mut addresses = Vec::new();
        let mut addresses2 = Vec::new();
        for pair in pairs {
            addresses.push(Address::from_str(pair).unwrap());
            addresses2.push(Address::from_str(pair).unwrap());
        }

        // todo talk with suizo
        //let mut addresses: [Address; 2] = [Address::default(); 2];
        //addresses[0] = Address::from_str(pairs[0])?;
        //addresses[1] = Address::from_str(pairs[1])?;

        let amount_out: U256 = U256::from_dec_str(token_amount).unwrap();
        let parameter_out = (amount_out, addresses);
        let amount_out_min: Vec<Uint> = self
            .query_contract(contract_instance, "getAmountsOut", parameter_out)
            .await;

        let slippage = 2usize;

        let min_amount = U256::from(amount_out_min[1].as_u128());

        let min_amount_less_slippage = min_amount - ((min_amount * slippage) / 100usize);

        let parameters2 = (
            min_amount_less_slippage,
            addresses2,
            self.first_loaded_account(),
            deadline + 600usize,
        );

        Ok(self
            .sign_and_send_tx(
                account,
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
            println!("balance: {}", wei_to_eth(balance));
        }
    }

    pub async fn last_nonce(&mut self) -> U256 {
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
            .transaction_count(self.first_loaded_account(), None)
            .await
            .unwrap();

        return nonce;
    }

    pub async fn load_account(
        &mut self,
        plain_address: &str,
        plain_private_key: &str,
    ) -> &mut Web3Manager {
        // cast plain pk to sk type

        let wallet: H160 = H160::from_str(plain_address).unwrap();

        // push on account list
        self.accounts_map.insert(wallet, plain_private_key.to_string());
        self.accounts.push(wallet);

        // get last nonce from loaded account
        let nonce: U256 = self.last_nonce().await;
        self.current_nonce = nonce;

        let gas_price: U256 = self.web3http.eth().gas_price().await.unwrap();
        self.current_gas_price = gas_price;

        return self;
    }

    pub async fn new(http_url: &str, websocket_url: &str) -> Web3Manager {
        // init web3 http connection
        let web3http: Web3<Http> = web3::Web3::new(web3::transports::Http::new(http_url).unwrap());

        // init web3 ws connection
        let web3web_socket: Web3<WebSocket> = web3::Web3::new(
            web3::transports::WebSocket::new(websocket_url)
                .await
                .unwrap(),
        );

        // create empty vector for store accounts
        let accounts: Vec<Address> = vec![];
        let balances: HashMap<H160, U256> = HashMap::new();
        let accounts_map: HashMap<H160, String> = HashMap::new();

        let current_nonce: U256 = U256::from(0);
        let current_gas_price: U256 = U256::from(0);

        let chain_id: Option<u64> =
            Option::Some(u64::try_from(web3http.eth().chain_id().await.unwrap()).unwrap());

        return Web3Manager {
            accounts,
            balances,
            web3http,
            web3web_socket,
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

    pub async fn sign_transaction(&self, account: H160, transact_obj: TransactionParameters) -> SignedTransaction {
        let plain_pk = self.accounts_map.get(&account).unwrap();
        let private_key = SecretKey::from_str(plain_pk).unwrap();

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

    pub fn first_loaded_account(&mut self) -> H160 {
        return self.accounts[0];
    }

    pub async fn approve_erc20_token(
        &mut self,
        account: H160,
        contract_instance: Contract<Http>,
        spender: &str,
        value: &str,
    ) -> H256 {
        let spender_address: Address = Address::from_str(spender).unwrap();
        let contract_function = "approve";
        let contract_function_parameters = (spender_address, U256::from_dec_str(value).unwrap());

        let result: H256 = self
            .sign_and_send_tx(
                account,
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
        account: H160,
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
        let signed_transaction: SignedTransaction = self.sign_transaction(account, tx_parameters).await;

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
        account: H160,
        contract_instance: Contract<Http>,
        to: &str,
        token_amount: &str,
    ) -> H256 {
        let contract_function = "transfer";

        let recipient_address: Address = Address::from_str(to).unwrap();
        let contract_function_parameters =
            (recipient_address, U256::from_dec_str(token_amount).unwrap());

        let result: H256 = self
            .sign_and_send_tx(
                account,
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

fn split_vector_in_chunks(data: Vec<Uint>, chunk_size: usize) -> Vec<Vec<Uint>> {
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
