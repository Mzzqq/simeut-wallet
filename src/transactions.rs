use std::time::Duration;
use std::env;
use dotenv::dotenv;
use eyre::{ContextCompat, Result};
use hex::ToHex;

use ethers::{
    prelude::{LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
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

    let address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let random_address = address_hex.parse::<Address>()?;
    let address_balance = provider.get_balance(random_address, None).await?;
    println!("Balance for address {}: {}",
             address_hex, address_balance
    );

    let tx = TransactionRequest::pay(random_address, U256::from(1000u64)).from(first_address);
    let receipt =  provider
        .send_transaction(tx, None)
        .await?
        .log_msg("Pending transfer")
        .await?
        .context("Missing receipt")?;

    println!(
        "TX mined in block {}",
        receipt.block_number.context("cannot get block number")?
    );
    println!(
        "Balance of {} {}",
        address_hex, provider.get_balance(random_address, None).await?
    );
    Ok(())
}