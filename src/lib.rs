extern crate alloc;

pub mod ethereum_mainnet;
pub mod rinkeby_testnet;
pub mod traits;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use futures::{future, StreamExt};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::env;
// use std::env;
use chrono::format::format;
use secp256k1::rand::rngs::StdRng;
use secp256k1::rand::Rng;
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use std::str::FromStr;
use std::time::{SystemTime, SystemTimeError};
use web3::api::SubscriptionStream;
use web3::contract::tokens::{Detokenize, Tokenize};
use web3::contract::{Contract, Options};
use web3::ethabi::ethereum_types::H256;
use web3::ethabi::{Int, Uint};
use web3::signing::keccak256;
use web3::transports::{Http, WebSocket};
use web3::types::{
    Address, Bytes, FilterBuilder, Log, SignedTransaction, Transaction, TransactionId,
    TransactionParameters, H160, U256, U64,
};
use web3::Web3;

// use hex_literal::hex;

/// Emulates a `switch` statement.
///
/// The syntax is similar to `match` except that every left-side expression is
/// interpreted as an expression rather than a pattern. The expression to
/// compare against must be at the beginning with a semicolon. A default case
/// is required at the end with a `_`, similar to `match`.
///
/// Example:
///
/// ```
/// use switch_statement::switch;
/// use web3_rust_wrapper::switch;
///
/// const A: u32 = 1 << 0;
/// const B: u32 = 1 << 1;
///
/// let n = 3;
/// let val = switch! { n;
///     A => false,
///     // this is a bitwise OR
///     A | B => true,
///     _ => false,
/// };
/// assert!(val);
/// ```
#[macro_export]
macro_rules! switch {
    ($v:expr; $($a:expr => $b:expr,)* _ => $e:expr $(,)?) => {
        match $v {
            $(v if v == $a => $b,)*
            _ => $e,
        }
    };
}

#[cfg(test)]
mod tests {
    const A: u32 = 1 << 0;
    const B: u32 = 1 << 1;
    const C: u32 = 1 << 2;
    const D: u32 = 1 << 3;

    #[test]
    fn it_works() {
        assert!(switch! { 1; _ => true });

        let v = switch! { A | B;
            A => false,
            B | C => false,
            A | B => true,
            C | D => {
                unreachable!();
            },
            _ => false,
        };
        assert!(v);
    }

    #[test]
    fn no_trailing_comma() {
        let v = switch! { 1;
            1 => true,
            _ => false
        };
        assert!(v);
    }
}

// use chainlink_interface::EthereumFeeds;
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

#[derive(Clone, Debug)]
pub struct Web3Manager {
    // all the accounts
    pub accounts: Vec<H160>,
    // balances of each accounts
    pub balances: HashMap<H160, U256>,
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
            Address::from_str(plain_contract_address)?,
            abi_path,
        )?)
    }

    pub fn generate_keypair() -> (SecretKey, PublicKey) {
        let secp = secp256k1::Secp256k1::new();

        let n2: u64 = 1;
        println!("first random u64 is: {}", n2);
        let mut rng: StdRng = rngs::StdRng::seed_from_u64(n2);
        let random_number: u64 = rng.gen::<u64>();
        println!(
            "With seed {}, the first random u64 is: {}",
            n2, random_number
        );

        secp.generate_keypair(&mut rng)
    }

    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }

    pub fn generate_keypairs(n: u8) -> Vec<(SecretKey, PublicKey)> {
        let mut keypairs: Vec<(SecretKey, PublicKey)> = Vec::new();
        for _ in 0..n {
            keypairs.push(Web3Manager::generate_keypair());
        }
        keypairs
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

    fn wei_to_eth2(val: &str) -> U256 {
        let v: f64 = val.parse().unwrap();
        let a: U256 = U256::from_dec_str(v.clone().to_string().as_str()).unwrap();
        //et k = wei_to_eth(a);
        return a;
    }

    pub fn get_account_balance(&self, account: H160) -> U256 {
        self.balances[&account]
    }

    pub fn generate_deadline(&self) -> Result<U256, SystemTimeError> {
        Ok(U256::from(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs(),
        ))
    }

    // TODO(elsuizo:2022-03-03): documentation here
    pub async fn swap_tokens_for_exact_tokens(
        &mut self,
        account: H160,
        contract_instance: &Contract<Http>,
        token_amount: &str,
        pairs: &[&str],
        slippage: usize,
    ) -> Result<H256, Box<dyn std::error::Error>> {
        let contract_function = "swapTokensForExactTokens";
        let deadline = self.generate_deadline()?;

        let mut addresses = Vec::new();
        for pair in pairs {
            addresses.push(Address::from_str(pair).unwrap());
        }

        let amount_out: U256 = U256::from_dec_str(token_amount).unwrap();
        let parameter_out = (amount_out, addresses.clone());
        let amount_out_min: Vec<Uint> = self
            .query_contract(contract_instance, "getAmountsOut", parameter_out)
            .await?;

        let min_amount = U256::from(amount_out_min[1].as_u128());
        let min_amount_less_slippage = min_amount - ((min_amount * slippage) / 100usize);

        let parameters2 = (
            amount_out,
            min_amount_less_slippage,
            addresses,
            self.first_loaded_account(),
            deadline + 600usize,
        );

        println!("amount_out: {:?}", amount_out);
        println!("min_amount_less_slippage: {:?}", min_amount_less_slippage);

        Ok(self
            .sign_and_send_tx(
                account,
                contract_instance,
                contract_function,
                &parameters2,
                &U256::from("0").to_string(),
            )
            .await)
    }

    // TODO(elsuizo:2022-03-03): documentation here
    pub async fn swap_eth_for_exact_tokens(
        &mut self,
        account: H160,
        router_address: &str,
        token_amount: &str,
        pairs: &[&str],
        slippage: usize,
    ) -> Result<H256, Box<dyn std::error::Error>> {
        let mut router_abi_path = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";
        let mut contract_function: &str = "swapExactETHForTokens";

        switch! { router_address;
            "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3" => {
                println!("Input is equal to 0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3");
                router_abi_path = "../abi/PancakeRouterAbi.json";
                contract_function = "swapExactETHForTokens";
            },
                "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3" => {
                println!("Input is equal to 0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3");
                router_abi_path = "../abi/PancakeRouterAbi.json";
                contract_function = "swapExactETHForTokens";
            },
                "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3" => {
                println!("Input is equal to 0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3");
                router_abi_path = "../abi/PancakeRouterAbi.json";
                contract_function = "swapExactETHForTokens";
            },
            _ => {
                println!("Input is equal to 0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3");
                router_abi_path = "../abi/PancakeRouterAbi.json";
                contract_function = "swapExactETHForTokens";
            },
        }

        let router_abi = include_bytes!("../abi/PancakeRouterAbi.json");
        let router_instance: Contract<Http> = self
            .instance_contract(router_address, router_abi)
            .await
            .expect("error creating the router instance");

        let mut addresses = Vec::new();
        for pair in pairs {
            addresses.push(Address::from_str(pair).unwrap());
        }

        let amount_out: U256 = U256::from_dec_str(token_amount).unwrap();
        let parameter_out = (amount_out, addresses.clone());
        let amount_out_min: Vec<Uint> = self
            .query_contract(&router_instance, "getAmountsOut", parameter_out)
            .await?;

        let min_amount = U256::from(amount_out_min[1].as_u128());
        let min_amount_less_slippage = min_amount - ((min_amount * slippage) / 100usize);

        let deadline = self.generate_deadline()?;
        let parameters2 = (
            min_amount_less_slippage,
            addresses,
            self.first_loaded_account(),
            deadline + 600usize,
        );

        Ok(self
            .sign_and_send_tx(
                account,
                &router_instance,
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
    ) -> Result<U256, web3::contract::Error> {
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

    // Counts the number of exececuted transactions by the loaded wallet to set the 'nonce' param for current transacction
    // Cuenta el número de transacciones se han ejecutado con la wallet cargada para establecer el parámetro 'nonce' en la transacción actual
    pub async fn last_nonce(&self) -> Result<U256, web3::Error> {
        self.web3http
            .eth()
            .transaction_count(self.first_loaded_account(), None)
            .await
    }

    pub async fn load_account(
        &mut self,
        plain_address: &str,
        plain_private_key: &str,
    ) -> &mut Web3Manager {
        // cast plain pk to sk type

        let wallet: H160 = H160::from_str(plain_address).unwrap();

        // push on account list
        self.accounts_map
            .insert(wallet, plain_private_key.to_string());
        self.accounts.push(wallet);

        // load accounts balances
        self.set_token_balances().await;

        // get last nonce from loaded account
        let nonce: U256 = self
            .last_nonce()
            .await
            .expect("error getting the nonce parameter");
        self.current_nonce = nonce;

        let gas_price: U256 = self
            .web3http
            .eth()
            .gas_price()
            .await
            .expect("error getting the gas price parameter");
        self.current_gas_price = gas_price;

        self
    }

    pub async fn new(http_url: &str, websocket_url: &str, u64chain_id: u64) -> Web3Manager {
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

        //let chain_id: Option<u64> = Option::Some(u64::try_from(web3http.eth().chain_id().await.unwrap()).unwrap());
        let chain_id: Option<u64> = Option::Some(u64::try_from(u64chain_id).unwrap());

        Web3Manager {
            accounts,
            balances,
            web3http,
            web3web_socket,
            accounts_map,
            current_nonce,
            current_gas_price,
            chain_id,
        }
    }

    // Get a estimation on medium gas price in network
    // Obtiene un precio del gas  estimado en la red
    pub async fn gas_price(&self) -> Result<U256, web3::Error> {
        self.web3http.eth().gas_price().await
    }

    // Get the current block in the network
    // Obtiene el número del bloque actual en la red
    pub async fn get_block(&self) -> Result<U64, web3::Error> {
        self.web3http.eth().block_number().await
    }

    pub async fn query_contract<P, T>(
        &self,
        contract_instance: &Contract<Http>,
        func: &str,
        params: P,
    ) -> Result<T, web3::contract::Error>
    where
        P: Tokenize,
        T: Detokenize,
    {
        // query contract
        contract_instance
            .query(func, params, None, Default::default(), None)
            .await
    }

    // To execute a function in a contract it has to be sent as a raw transaction which is the basic transaction format
    // Para ejecutar cualquier transacción en un contrato ha de ser mandada como una transacción de tipo raw,
    // que es el formato básico de las transaaciones
    pub async fn send_raw_transaction(&self, raw_transaction: Bytes) -> H256 {
        self.web3http
            .eth()
            .send_raw_transaction(raw_transaction)
            .await
            .unwrap()
    }

    fn update_nonce(&mut self) {
        self.current_nonce = self.current_nonce + 1;
    }

    // The transactions must be signed with the private key of the wallet that executes it
    // Las transacciones han de ser firmadas con la clave privada de la cartera que la ejecuta
    pub async fn sign_transaction(
        &self,
        account: H160,
        transact_obj: TransactionParameters,
    ) -> SignedTransaction {
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
        contract
            .abi()
            .function(func)
            .unwrap()
            .encode_input(&params.into_tokens())
            .unwrap()
            .into()
    }

    pub async fn estimate_tx_gas<P>(
        &self,
        contract: &Contract<Http>,
        func: &str,
        params: P,
        value: &str,
    ) -> U256
    where
        P: Tokenize,
    {
        contract
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
            .unwrap()
    }

    pub fn first_loaded_account(&self) -> H160 {
        self.accounts[0]
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
                &contract_instance,
                &contract_function.to_string(),
                &contract_function_parameters,
                "0",
            )
            .await;
        return result;
    }

    pub async fn sign_and_send_tx<P: Clone>(
        &mut self,
        account: H160,
        contract_instance: &Contract<Http>,
        func: &str,
        params: &P,
        value: &str,
    ) -> H256
    where
        P: Tokenize,
    {
        // estimate gas for call this function with this parameters
        // increase 200ms execution time, we use high gas available
        // gas not used goes back to contract

        let estimated_tx_gas: U256 = self
            .estimate_tx_gas(&contract_instance.clone(), &func, params.clone(), value)
            .await;

        //let estimated_tx_gas: U256 = U256::from_dec_str("5000000").unwrap();

        // 2. encode_tx_data
        let tx_data: Bytes = self.encode_tx_data(contract_instance, func, params.clone());

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
        let signed_transaction: SignedTransaction =
            self.sign_transaction(account, tx_parameters).await;

        // send tx
        let tx_id: H256 = self
            .web3http
            .eth()
            .send_raw_transaction(signed_transaction.raw_transaction)
            .await
            .unwrap();

        println!(
            "Transaction successful with hash: {}{:?}",
            &env::var("EXPLORER").unwrap(),
            tx_id
        );

        self.update_nonce();

        return tx_id;

        // NOTE(elsuizo:2022-03-05): esta es la unica linea de codigo que hace que se necesite un
        // `&mut self` una de las reglas a seguir en Rust es no utilizar &mut cuando no es
        // necesario ya que con esa informacion el compilador puede hacer mas optimizaciones y
        // simplificaciones
        // self.current_nonce = self.current_nonce + 1; // todo, check pending nonce dont works
    }

    pub async fn sent_erc20_token(
        &mut self,
        account: H160,
        contract_instance: &Contract<Http>,
        to: &str,
        token_amount: &str,
    ) -> H256 {
        let contract_function = "transfer";

        let recipient_address: Address = Address::from_str(to).unwrap();
        let contract_function_parameters =
            (recipient_address, U256::from_dec_str(token_amount).unwrap());

        self.sign_and_send_tx(
            account,
            contract_instance,
            contract_function,
            &contract_function_parameters,
            "0",
        )
        .await
    }

    //-------------------------------------------------------------------------
    //                        chainlink inplementations
    //-------------------------------------------------------------------------

    pub async fn get_latest_price(
        &self,
        network: impl crate::traits::GetAddress,
        pair_address: &str,
    ) -> Int {
        let proxy_abi = include_bytes!("../abi/EACAggregatorProxy.json");
        let proxy_instance: Contract<Http> = self
            .instance_contract(&network.get_address(pair_address).unwrap(), proxy_abi)
            .await
            .expect("error creating the proxy instance");

        let res: (Uint, Int, Uint, Uint, Uint) = self
            .query_contract(&proxy_instance, "latestRoundData", ())
            .await
            .unwrap();
        res.1
    }

    /*
        pub async fn access_controller(
        &self,
        feed: impl crate::traits::GetAddress,
        pair: &str,
    ) -> Result<Address, web3::contract::Error> {
        let proxy_abi = include_bytes!("../abi/EACAggregatorProxy.json");
        let proxy_instance: Contract<Http> = self
            .instance_contract(&feed.get_address(pair).unwrap(), proxy_abi)
            .await
            .expect("error creating the proxy instance");
        self.query_contract(&proxy_instance, "accessController", ())
            .await
    }
     */

    pub async fn listen_contract_events(&self, contract_address: &str) {
        /*
        let filter = FilterBuilder::default()
        .address(vec![contract.address()])
        .topics(
            Some(vec![hex!(
                "d282f389399565f3671145f5916e51652b60eee8e5c759293a2f5771b8ddfd2e"
            )
            .into()]),
            None,
            None,
            None,
        )
        .build();
         */

        let filter = FilterBuilder::default()
            .address(vec![Address::from_str(contract_address).unwrap()])
            .topics(None, None, None, None)
            .build();

        let sub: SubscriptionStream<WebSocket, Log> = self
            .web3web_socket
            .eth_subscribe()
            .subscribe_logs(filter)
            .await
            .unwrap();
        sub.for_each(|log| async {
            let l: Log = log.unwrap();
            println!("Address: {:?}", l.transaction_hash.unwrap());
            println!("Data: {:?}", l.data);
            println!("Data0: {:?}", l.data.0);
            println!("{}", std::str::from_utf8(&l.data.0).unwrap());
            println!("topics: {:?}", l.topics);
            println!("log_type: {:?}", l.log_type);

            let tx = self
                .web3http
                .eth()
                .transaction(TransactionId::Hash(l.transaction_hash.unwrap()))
                .await
                .unwrap()
                .unwrap();

            let from_addr = tx.from.unwrap_or(H160::zero());
            let to_addr = tx.to.unwrap_or(H160::zero());
            let value = tx.value;
            let input = tx.input;

            println!("from_addr: {:?}", from_addr);
            println!("to_addr: {:?}", to_addr);
            println!("value: {:?}", value);
            println!("input: {:?}", input);
        })
        .await;
    }

    pub async fn build_contract_events(
        &self,
        contract_address: &str,
    ) -> SubscriptionStream<WebSocket, Log> {
        let filter = FilterBuilder::default()
            .address(vec![Address::from_str(contract_address).unwrap()])
            .topics(None, None, None, None)
            .build();

        let sub: SubscriptionStream<WebSocket, Log> = self
            .web3web_socket
            .eth_subscribe()
            .subscribe_logs(filter)
            .await
            .unwrap();
        return sub;
    }
}

pub fn split_vector_in_chunks(data: Vec<Uint>, chunk_size: usize) -> Vec<Vec<Uint>> {
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

pub fn split_vector_in_chunks2(data: &[Uint], chunk_size: usize) -> Vec<Vec<Uint>> {
    data.chunks(chunk_size)
        .map(|element| element.to_vec())
        .collect()
}
