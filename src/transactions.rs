use std::time::Duration;
use std::env;
use dotenv::dotenv;
use eyre::{ContextCompat, Result};
use hex::ToHex;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    utils::Ganache,
};
use ethers::abi::AbiEncode;
use ethers::abi::ParamType::Address;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let mnemonic = std::env::var("MNEMONIC_KEY").unwrap();
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("Endpoint: {}", ganache.endpoint());

    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!("wallet first address {}", first_address.encode_hex());

    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));
    let first_balance = provider.get_balance(first_address, None).await?;
    println!("wallet first address balance: {}", first_balance);

    let random_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;
    let address_hex = "";
}