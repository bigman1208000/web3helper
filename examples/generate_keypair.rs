use chrono::{NaiveDate, NaiveTime};
use secp256k1::{PublicKey, SecretKey};
use serde_json::Value;
use web3_rust_wrapper::Web3Manager;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyPair {
    pub secret_key: String,
    pub public_key: String,
}

#[tokio::main]
async fn main() -> web3::Result<()> {


    // generate new private, public and wallet address
    println!("Generate new private, public and wallet address.\n");
    let (secret_key, pub_key) = Web3Manager::generate_keypair();
    let kp: KeyPair = KeyPair {
        secret_key: secret_key.display_secret().to_string(),
        public_key: pub_key.to_string(),
    };
    println!("Keypair: {:?}\n", kp);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S").unwrap();
    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap();

    // Save the JSON structure into the other file.
    let now = chrono::offset::Utc::now();
    let path = format!("./wallets/{}{}", "wss", ".json");
    println!("{}", path);
    std::fs::write(
        path,
        serde_json::to_string_pretty(&kp).unwrap(),
    )
        .unwrap();

    /*
    // generate list
    println!("Generate list of keypairs\n");
    let signers: Vec<(SecretKey, PublicKey)> = Web3Manager::generate_keypairs(10);
    println!("signers: {:?}", signers);

    for signer in signers.iter() {
        let (signer_secret_key, signer_pub_key) = signer;
        println!("secret key: {}", signer_secret_key.display_secret().to_string());
        println!("public key: {}", &signer_pub_key.to_string());
        println!("wallet address: {:?}\n", Web3Manager::public_key_address(&signer_pub_key));
    }
    */

    Ok(())
}
