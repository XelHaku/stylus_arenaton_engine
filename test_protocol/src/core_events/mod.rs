use crate::call_contract::{call_contract_method, call_contract_method_signed};
use crate::constants::env_vars::{get_env_vars, EnvVars};
use crate::constants::wallets::{Wallet, WALLETS};
use ethers::prelude::*;
use eyre::Result;
use serde::de::value;
use std::sync::Arc;

pub async fn stake_eth(
    contract_address: &str,
    player_private_key: &str,
    rpc_url: &str,
    chain_id: u64,
    player: Address,
    value: U256,
) -> Result<()> {
    let abi_json = r#"[
{
        "inputs": [
            { "internalType": "address", "name": "_player", "type": "address" }
        ],
        "name": "stakeEth",
        "outputs": [
            { "internalType": "bool", "name": "", "type": "bool" }
        ],
        "stateMutability": "payable",
        "type": "function"
    }
]"#;

    let env = get_env_vars();

    // Create signer from private key
    let wallet = player_private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));
    let core_address = env.core_address;

    let receipt =
        call_contract_method_signed("stakeEth", player, abi_json, &core_address, signer, value)
            .await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt.gas_used),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}
pub async fn add_event(event_id: &str, start_date: u64, _wallet: &Wallet) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "string", "name": "event_id", "type": "string" },
                { "internalType": "uint64", "name": "start_date", "type": "uint64" }
            ],
            "name": "addEvent",
            "outputs": [
                { "internalType": "bool", "name": "", "type": "bool" }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#; // Removed trailing comma in inputs array

    // Rest of the function remains the same
    let event_id = event_id.to_string();
    let env = get_env_vars();
    
    let core_address = env.core_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;

    let wallet = _wallet
        .private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        "addEvent",
        (event_id, start_date),
        abi_json,
        &core_address,
        signer,
        U256::zero(),
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nEvent added successfully: {:?}", receipt.gas_used),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}
pub async fn close_event(event_id: &str, winner: u8,_wallet: &Wallet) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "string", "name": "event_id", "type": "string" },
                { "internalType": "uint8", "name": "winner", "type": "uint8" },
            ],
            "name": "closeEvent",
            "outputs": [
                { "internalType": "bool", "name": "", "type": "bool" }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;


    //  function addEvent(string calldata event_id, uint64 start_date) external returns (bool);

    // Convert event_id to String
    let event_id = event_id.to_string();
    let env = get_env_vars();

    let core_address = env.core_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;

    let wallet = _wallet
        .private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        "closeEvent",
        (event_id, winner),
        abi_json,
        &core_address,
        signer,
        U256::zero(),
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nEvent added successfully: {:?}", receipt.gas_used),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}

pub async fn set_oracle(oracle_address: &str,_wallet: &Wallet) -> Result<()> {

        // function setOracle(address _oracle_address) external returns (bool);

    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "address", "name": "_oracle_address", "type": "address" }
            ],
            "name": "setOracle",
            "outputs": [ { "internalType": "bool", "name": "", "type": "bool" } ],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;


//  function setOracle(address _oracle_address) external returns (bool);
    let env = get_env_vars();

    let core_address = env.core_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;

    // Create signer from private key
    let wallet = _wallet
        .private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    // Convert oracle_address to Address type
    let oracle_address = oracle_address.parse::<Address>()?;

    let receipt = call_contract_method_signed(
        "setOracle",
        oracle_address,
        abi_json,
        &core_address,
        signer,
        U256::zero(),
    )
    .await?;



    match receipt {
        Some(receipt) => println!("\nOracle role granted successfully: {:?}", receipt.gas_used),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}

pub async fn initialize(_wallet: &Wallet) -> Result<()> {
    let abi_json = r#"[
  {
    "inputs": [],
    "name": "initialize",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "nonpayable", 
    "type": "function"
  }
]"#;
    let env = get_env_vars();

    let core_address = env.core_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;
    // Create signer from private key
    let wallet = _wallet
        .private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        "initialize",
        (), // No arguments
        abi_json,
        &core_address,
        signer,
        U256::zero(), // No value sent
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt.gas_used),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}




// /**
//  * This file was automatically generated by Stylus and represents a Rust program.
//  * For more information, please see [The Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs).
//  */

// // SPDX-License-Identifier: MIT-OR-APACHE-2.0
// pragma solidity ^0.8.23;

// interface ICoreEvents  {
//     function initialize() external returns (bool);

//     function setOracle(address _oracle_address) external returns (bool);

//     function addEvent(string calldata event_id, uint64 start_date) external returns (bool);

//     function closeEvent(string calldata event_id, uint8 winner) external returns (bool);

//     function getOpenedEventList(uint64 page_size, uint64 page) external view returns (string,uint64,uint8,uint8)[] memory;

//     function getClosedEventList(uint64 page_size, uint64 page) external view returns (string,uint64,uint8,uint8)[] memory;

//     function getEvent(string calldata _event_id_string) external view returns (string memory, uint64, uint8, uint8);

//     function owner() external view returns (address);

//     function transferOwnership(address new_owner) external;

//     function renounceOwnership() external;

//     error AlreadyInitialized();

//     error AleadyAdded();

//     error AlreadyStarted();

//     error NotStartedYet();

//     error NotAuthorized();

//     error WrongStatus();

//     error WrongWinner();

//     error InvalidTeam();

//     error OwnableUnauthorizedAccount(address);

//     error UnauthorizedOracle(address);

//     error OwnableInvalidOwner(address);
// }
// xel@X1Carbon:~/git/arenaton-stylus/stylus_core_events$ 