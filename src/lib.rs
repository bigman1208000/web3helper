#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
// mod bnb_main_net;
// mod rinkeby_testnet;
pub mod traits;
use secp256k1::SecretKey;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::convert::{From, TryFrom};
// use std::env;
use std::str::FromStr;
use std::time::{SystemTime, SystemTimeError};
use web3::contract::tokens::{Detokenize, Tokenize};
use web3::contract::{Contract, Options};
use web3::ethabi::ethereum_types::H256;
use web3::ethabi::Uint;
use web3::transports::{Http, WebSocket};
use web3::types::{Address, Bytes, SignedTransaction, TransactionParameters, H160, U256, U64};
use web3::Web3;
// use hex_literal::hex;

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

#[derive(Clone)]
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
        &self,
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

        // NOTE(elsuizo:2022-03-09): claaaro ya entendi aqui las addreeses pueden ser mas de dos
        // por eso es mejor usar un `Vec`
        // todo talk with suizo
        //let mut addresses: [Address; 2] = [Address::default(); 2];
        //addresses[0] = Address::from_str(pairs[0])?;
        //addresses[1] = Address::from_str(pairs[1])?;

        let amount_out: U256 = U256::from_dec_str(token_amount).unwrap();
        let parameter_out = (amount_out, addresses.clone());
        let amount_out_min: Vec<Uint> = self
            .query_contract(contract_instance, "getAmountsOut", parameter_out)
            .await;

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
        &self,
        account: H160,
        contract_instance: &Contract<Http>,
        token_amount: &str,
        pairs: &[&str],
        slippage: usize,
    ) -> Result<H256, Box<dyn std::error::Error>> {
        let contract_function = "swapETHForExactTokens";
        let deadline = self.generate_deadline()?;

        let mut addresses = Vec::new();
        for pair in pairs {
            addresses.push(Address::from_str(pair).unwrap());
        }

        // NOTE(elsuizo:2022-03-09): claaaro ya entendi aqui las addreeses pueden ser mas de dos
        // por eso es mejor usar un `Vec`
        // todo talk with suizo
        //let mut addresses: [Address; 2] = [Address::default(); 2];
        //addresses[0] = Address::from_str(pairs[0])?;
        //addresses[1] = Address::from_str(pairs[1])?;

        let amount_out: U256 = U256::from_dec_str(token_amount).unwrap();
        let parameter_out = (amount_out, addresses.clone());
        let amount_out_min: Vec<Uint> = self
            .query_contract(contract_instance, "getAmountsOut", parameter_out)
            .await;

        let min_amount = U256::from(amount_out_min[1].as_u128());
        let min_amount_less_slippage = min_amount - ((min_amount * slippage) / 100usize);

        let parameters2 = (
            min_amount_less_slippage,
            addresses,
            self.first_loaded_account(),
            deadline + 600usize,
        );

        Ok(self
            .sign_and_send_tx(
                account,
                contract_instance,
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

    // Counts the number of exececuted transactions by the loaded wallet to set the 'nonce' param for current transacction
    // Cuenta el número de transacciones se han ejecutado con la wallet cargada para establecer el parámetro 'nonce' en la transacción actual
    pub async fn last_nonce(&self) -> U256 {
        /*
        let block: Option<BlockNumber> = BlockNumber::Pending.into();

        let nonce: U256 = self.web3http
            .eth()
            .transaction_count(self.accounts[0], block)
            .await
            .unwrap();
        */

        self.web3http
            .eth()
            .transaction_count(self.first_loaded_account(), None)
            .await
            .unwrap()
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
        let nonce: U256 = self.last_nonce().await;
        self.current_nonce = nonce;

        let gas_price: U256 = self.web3http.eth().gas_price().await.unwrap();
        self.current_gas_price = gas_price;

        self
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
    pub async fn gas_price(&self) -> U256 {
        self.web3http.eth().gas_price().await.unwrap()
    }

    // Get the current block in the network
    // Obtiene el número del bloque actual en la red
    pub async fn get_block(&self) -> U64 {
        self.web3http.eth().block_number().await.unwrap()
    }

    pub async fn query_contract<P, T>(
        &self,
        contract_instance: &Contract<Http>,
        func: &str,
        params: P,
    ) -> T
    where
        P: Tokenize,
        T: Detokenize,
    {
        // query contract
        contract_instance
            .query(func, params, None, Default::default(), None)
            .await
            .unwrap()
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
        &self,
        account: H160,
        contract_instance: &Contract<Http>,
        spender: &str,
        value: &str,
    ) -> H256 {
        let spender_address: Address = Address::from_str(spender).unwrap();
        let contract_function = "approve";
        let contract_function_parameters = (spender_address, U256::from_dec_str(value).unwrap());

        self.sign_and_send_tx(
            account,
            contract_instance,
            contract_function,
            &contract_function_parameters,
            "0",
        )
        .await
    }

    pub async fn sign_and_send_tx<P: Clone>(
        &self,
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
        self.web3http
            .eth()
            .send_raw_transaction(signed_transaction.raw_transaction)
            .await
            .unwrap()

        /*
        println!(
            "Transaction successful with hash: {}{:?}",
            &env::var("EXPLORER").unwrap(),
            result
        );
        */
        // NOTE(elsuizo:2022-03-05): esta es la unica linea de codigo que hace que se necesite un
        // `&mut self` una de las reglas a seguir en Rust es no utilizar &mut cuando no es
        // necesario ya que con esa informacion el compilador puede hacer mas optimizaciones y
        // simplificaciones
        // self.current_nonce = self.current_nonce + 1; // todo, check pending nonce dont works
    }

    pub async fn sent_erc20_token(
        &self,
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

    async fn access_controller(&self, feed: impl crate::traits::GetAddress) -> Address {
        let proxy_abi = include_bytes!("../abi/EACAggregatorProxy.json");
        let proxy_instance: Contract<Http> = self
            .instance_contract(&feed.get_address(), proxy_abi)
            .await
            .expect("error creating the proxy instance");
        self.query_contract(&proxy_instance, "accessController", ())
            .await
    }
}

fn wei_to_eth(wei_val: U256) -> f64 {
    // ethereum does not have fractional numbers so every amount is expressed in wei, to show the
    // amount in ether this function is used ethereum no tiene numeros fraccionarios por lo que
    // toda cantidad se expresa en wei, para mostrar la cantidad en ether se utiliza esta función
    wei_val.as_u128() as f64 / 1_000_000_000_000_000_000.0f64
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

//-------------------------------------------------------------------------
//                        tests
//-------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{split_vector_in_chunks, split_vector_in_chunks2};
    use crate::U256;

    #[test]
    fn split_vector_tests() {
        let vector = vec![
            U256::from(3usize),
            U256::from(2usize),
            U256::from(4usize),
            U256::from(3usize),
            U256::from(4usize),
            U256::from(4usize),
            U256::from(0usize),
        ];
        let vector2 = vector.clone();
        let result = split_vector_in_chunks(vector, 2);
        let expected = split_vector_in_chunks2(&vector2, 2);
        assert_eq!(
            &result[..],
            &expected[..],
            "\nExpected\n{:?}\nfound\n{:?}",
            &result[..],
            &expected[..]
        );
    }
}
