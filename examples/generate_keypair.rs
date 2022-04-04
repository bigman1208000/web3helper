use secp256k1::{PublicKey, SecretKey};
use web3_rust_wrapper::Web3Manager;

#[tokio::main]
async fn main() -> web3::Result<()> {

    // generate new private, public and wallet address
    println!("Generate new private, public and wallet address.\n");
    let (secret_key, pub_key) = Web3Manager::generate_keypair();
    println!("secret key: {}", &secret_key.display_secret().to_string());
    println!("public key: {}", &pub_key.to_string());
    println!("wallet address: {:?}\n", Web3Manager::public_key_address(&pub_key));

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
