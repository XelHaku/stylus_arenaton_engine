pub mod eth_balance;
pub mod fund_players_eth;


// src/functions/fund_players_eth.rs

use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

use crate::constants::env_vars::{get_env_vars, EnvVars};

pub async fn fund_player_eth(
    amount: &str, // Amount in Wei
    _player_address: &str, // Address of the player to fund
) -> Result<()> {
    let env = get_env_vars();
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;
    let whale_private_key = env.whale_private_key;

    // Set up the whale signer
    let whale_wallet = whale_private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let whale_signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        whale_wallet,
    ));

    // Parse the amount from string to U256
    let amount = U256::from_dec_str(amount)?;

    // Fund the specified amount to each player from the whale, up to the limit
            let tx = whale_signer
                .send_transaction(
                    TransactionRequest::pay(
                        _player_address.parse::<Address>()?,
                        amount, // Use the parsed amount
                    ),
                    None,
                )
                .await?
                .await?;

            match tx {
                Some(receipt) => {
                    println!(
                        "Funded player {} with {} Wei. Transaction hash: {:?}",
                        _player_address, amount, receipt.transaction_hash
                    );
                }
                None => {
                    println!("Failed to fund player {}", _player_address);
                }

     
    }

    Ok(())
}


use ethers::types::Address;


pub async fn assert_contract(condition: bool, message: &str) -> Result<()> {
    eyre::ensure!(condition, "Contract assertion failed: {}", message);
    Ok(())
}

use eyre::{ Context};


/// Returns the latest block timestamp in Unix time (seconds since epoch)
pub async fn get_block_time() -> Result<u64> {
    let env = get_env_vars();
    let provider_url = env.rpc_url;
    // Create provider
    let provider = Provider::<Http>::try_from(provider_url)
        .wrap_err("Failed to create provider")?;

    // Get latest block
    let block = provider
        .get_block(BlockNumber::Latest)
        .await
        .wrap_err("Failed to retrieve latest block")?;

    match block {
        Some(b) => {
            // Convert U256 to u64 using low_u64() method
            Ok(b.timestamp.low_u64())
        }
        None => Err(eyre::eyre!("Block not found")),
    }
}

use std::thread;
use std::time::Duration;

/// Sleep for the specified number of milliseconds
pub fn sleep_ms(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}