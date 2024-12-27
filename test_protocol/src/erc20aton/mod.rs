use crate::call_contract::{call_contract_method, call_contract_method_signed};
use crate::constants::env_vars::{get_env_vars, EnvVars};
use ethers::prelude::*;
use eyre::Result;
use serde::de::value;
use std::ops::Add;
use std::sync::Arc;

use crate::constants::wallets::{Wallet, WALLETS};

/// Function to get the owner of the contract
pub async fn owner() -> Result<Address> {
    let abi_json = r#"
    [
        {
            "inputs": [],
            "name": "owner",
            "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]
    "#;
    let env = get_env_vars();

    // Debug: Print the ABI to ensure it's correct

    let contract_owner: Address =
        call_contract_method("owner", (), abi_json, &env.erc20aton_address, &env.rpc_url).await?;

    println!("\nContract owner: {}", contract_owner);
    Ok(contract_owner)
}

pub async fn vault() -> Result<(Address)> {
    let abi_json = r#"
    [
        {
            "inputs": [],
            "name": "vault",
            "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]
    "#;
    let env = get_env_vars();


    let contract_vault: Address =
        call_contract_method("vault", (), abi_json, &env.erc20aton_address, &env.rpc_url).await?;

    println!("\nContract vault: {}", contract_vault);
    Ok((contract_vault  ))
}

/// Function to get the name of the contract
pub async fn name() -> Result<String> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "name",
            "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
            "stateMutability": "pure",
            "type": "function"
        }
    ]"#;
    let env = get_env_vars();

    let contract_name: String = call_contract_method(
        "name",
        (), // No arguments
        abi_json,
        &env.erc20aton_address,
        &env.rpc_url,
    )
    .await?;

    println!("\nContract Name: {}", contract_name);
    Ok(contract_name)
}

/// Function to get the total supply of the contract
pub async fn total_supply() -> Result<U256> {
    let abi_json = r#"[
        {
            "inputs": [],
            "name": "totalSupply",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;
    let env = get_env_vars();

    let total_supply: U256 = call_contract_method(
        "totalSupply",
        (), // No arguments
        abi_json,
        &env.erc20aton_address,
        &env.rpc_url,
    )
    .await?;

    println!("\nTotal Supply: {}", total_supply);
    Ok(total_supply)
}

/// Function to get the balance of a specific address
pub async fn balance_of(owner_address: &str) -> Result<U256> {
    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "address", "name": "owner", "type": "address" }
            ],
            "name": "balanceOf",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;
    let env = get_env_vars();

    let erc20aton_address = env.erc20aton_address;
    let rpc_url = env.rpc_url;
    let owner: Address = owner_address.parse()?;

    let balance: U256 = call_contract_method(
        "balanceOf",
        owner, // Pass owner as argument
        abi_json,
        &erc20aton_address,
        &rpc_url,
    )
    .await?;

    println!("\nATON balance of {}: {}", owner, balance);
    Ok(balance)
}

/// Function to get the balance of a specific address
pub async fn is_stake_engine(engine_address: &str) -> Result<bool> {
    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "address", "name": "account", "type": "address" }
            ],
            "name": "isStakeEngine",
            "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#;

    // function isStakeEngine(address account) external view returns (bool);

    let env = get_env_vars();

    let erc20aton_address = env.erc20aton_address;
    let rpc_url = env.rpc_url;
    let engine: Address = engine_address.parse()?;

    let result: bool = call_contract_method(
        "isStakeEngine",
        engine, // Pass engine as argument
        abi_json,
        &erc20aton_address,
        &rpc_url,
    )
    .await?;

    println!("\nATON result of {}: {}", engine, result);
    Ok(result)
}




pub async fn approve( spender: &str, value: U256,_wallet: &Wallet) -> Result<()> {


let spender: H160 = spender.parse().unwrap();

    let abi_json = r#"[
  {
    "inputs": [
      { "internalType": "address", "name": "spender", "type": "address" },
      { "internalType": "uint256", "name": "value", "type": "uint256" }
    ],
    "name": "approve",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "nonpayable",
    "type": "function"
  }
]"#;
    let env = get_env_vars();

    let erc20aton_address: String = env.erc20aton_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;

    // Create signer from private key
    let wallet = _wallet.private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let signer = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_from(rpc_url)?,
        wallet,
    ));

    let receipt = call_contract_method_signed(
        // Remove <bool>
        "approve",
        (spender, value),
        abi_json,
        &erc20aton_address,
        signer,
        U256::from(0),
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt.gas_used),
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

    let erc20aton_address = env.erc20aton_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;
    // Create signer from private key
    let wallet = WALLETS[0]
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
        &erc20aton_address,
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


pub async fn update_stake_engine(
    account: &str,
    status: bool,_wallet: &Wallet
) -> Result<()> {
    // Use the correct ABI for the ERC20ATON contract
    let abi_json = r#"[
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "account",
                    "type": "address"
                },
                {
                    "internalType": "bool",
                    "name": "status",
                    "type": "bool"
                }
            ],
            "name": "updateStakeEngine",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;
    // function updateStakeEngine(address account, bool status) external;
    let env = get_env_vars();

    let erc20aton_address = env.erc20aton_address;
    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;
    // Parse contract addresses

    // Create signer from private key
    let wallet = WALLETS[0]
        .private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    // Call the contract method, passing arenaton_engine_addr as the function argument
    let receipt = call_contract_method_signed(
        "updateStakeEngine",
        (account.parse::<Address>()?, status), // the argument to 'updateStakeEngine'
        abi_json,
        &erc20aton_address,
        signer,
        U256::zero(),
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nTransaction successful: {:?}", receipt.gas_used),
        None => println!("\nTransaction executed successfully, but no receipt was returned."),
    }

    Ok(())
}
pub async fn set_vault(vault_address: &str,_wallet: &Wallet) -> Result<()> {
    // Use the correct ABI for the ERC20ATON contract
    let abi_json = r#"[
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "vault_address",
                    "type": "address"
                }
            ],
            "name": "setVault",
            "outputs": [
                {
                    "internalType": "bool",
                    "name": "",
                    "type": "bool"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#;

    let env = get_env_vars();

    let rpc_url = env.rpc_url;
    let chain_id = env.chain_id;

    // Parse the contract addresses
    let erc20aton_address = env.erc20aton_address;
    let vault_addr = vault_address.parse::<Address>()?;

    // Create signer from private key
    let wallet = WALLETS[0]
        .private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    // Prepare the function call parameters
    let params = (vault_addr,);

    // Call the contract method with the correct contract address and parameters
    let receipt = call_contract_method_signed(
        "setVault",
        params,
        abi_json,
        &erc20aton_address,
        signer,
        U256::zero(),
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

// interface IERC20ATON  {
//     function name() external pure returns (string memory);

//     function symbol() external pure returns (string memory);

//     function decimals() external pure returns (uint8);

//     function totalSupply() external view returns (uint256);

//     function balanceOf(address owner) external view returns (uint256);

//     function transferFrom(address from, address to, uint256 value) external returns (bool);

//     function approve(address spender, uint256 value) external returns (bool);

//     function allowance(address owner, address spender) external view returns (uint256);

//     function owner() external view returns (address);

//     function transferOwnership(address new_owner) external;

//     function initialize() external returns (bool);

//     function transfer(address to, uint256 amount) external returns (bool);

//     function mintAtonFromEth() external payable returns (bool);

//     function swap(uint256 amount) external returns (bool);

//     function updateStakeEngine(address account, bool status) external;

//     error InsufficientBalance(address, uint256, uint256);

//     error Zero(address);

//     error InsufficientAllowance(address, address, uint256, uint256);

//     error UnauthorizedAccount(address);
// }
