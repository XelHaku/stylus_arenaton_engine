use crate::call_contract::{call_contract_method, call_contract_method_signed};
use crate::constants::env_vars::{get_env_vars, EnvVars};
use crate::constants::wallets::{Wallet, WALLETS};
use ethers::prelude::*;
use eyre::Result;
use serde::de::value;
use tracing::event;
use std::sync::Arc;



pub async fn initialize(aton_address: &str, core_address: &str,_wallet: &Wallet) -> Result<()> {
    let abi_json = r#"[
  {
    "inputs": [
      { "internalType": "address", "name": "_aton_address", "type": "address" },
      { "internalType": "address", "name": "_core_address", "type": "address" }
    ],
    "name": "initialize",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "nonpayable", 
    "type": "function"
  }
]"#;

let _aton_address: H160 = aton_address.parse().unwrap();

let _core_address: H160 = core_address.parse().unwrap();



    // function initialize(address _aton_address, address _core_address) external returns (bool);

    let env = get_env_vars();

    let stake_address = env.stake_address;
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
        (_aton_address,_core_address),
        abi_json,
        &stake_address,
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



    // function readEventCore(string calldata _event_id) external view returns (string memory, uint64, uint8, uint8);
pub async fn read_event_core(_event_id: &str, _wallet: &Wallet) -> Result<()> {
let abi_json = r#"[
    {
        "inputs": [
            { "internalType": "string", "name": "_event_id", "type": "string" }
        ],
        "name": "readEventCore",
        "outputs": [
            { "internalType": "string", "name": "", "type": "string" },
            { "internalType": "uint64", "name": "", "type": "uint64" },
            { "internalType": "uint8", "name": "", "type": "uint8" },
            { "internalType": "uint8", "name": "", "type": "uint8" }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]"#;
    let env = get_env_vars();

    let stake_address = env.stake_address;
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
        "readEventCore",
        (_event_id.to_string(),),
        abi_json,
        &stake_address,
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


    // function stake(string calldata _event_id, uint256 _amount, uint8 _team) external payable returns (bool);
pub async fn stake(
    event_id: &str,
    _amount: U256,
    _value: U256,
    _team: u8,
    _wallet: &Wallet,
) -> Result<()> {
    let abi_json = r#"[
        {
            "inputs": [
                { "internalType": "string", "name": "_event_id", "type": "string" },
                { "internalType": "uint256", "name": "_amount", "type": "uint256" },
                { "internalType": "uint8", "name": "_team", "type": "uint8" }
            ],
            "name": "stake",
            "outputs": [
                { "internalType": "bool", "name": "", "type": "bool" }
            ],
            "stateMutability": "payable",
            "type": "function"
        }
    ]"#;

    let env = get_env_vars();
    let stake_address = env.stake_address;
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
        "stake",
        (event_id.to_string(), _amount, _team),
        abi_json,
        &stake_address,
        signer,
        _value,
    )
    .await?;

    match receipt {
        Some(receipt) => println!("\nStake successful. Gas used: {:?}", receipt.gas_used),
        None => println!("\nTransaction executed but no receipt received"),
    }

    Ok(())
}



pub async fn get_stakes(event_id: &str, page_size: u64, page: u64) -> Result<Vec<(U256, u8, u64)>> {
    let abi_json = r#"[
        {
            "type": "function",
            "name": "getStakes",
            "inputs": [
                {"name": "_event_id", "type": "string", "internalType": "string"},
                {"name": "page_size", "type": "uint64", "internalType": "uint64"},
                {"name": "page", "type": "uint64", "internalType": "uint64"}
            ],
            "outputs": [
                {
                    "type": "tuple[]",
                    "internalType": "struct MyContract.Stake[]",
                    "components": [
                        {"type": "uint256", "name": "amount", "internalType": "uint256"},
                        {"type": "uint8", "name": "team", "internalType": "uint8"},
                        {"type": "uint64", "name": "timestamp", "internalType": "uint64"}
                    ]
                }
            ],
            "stateMutability": "view"
        }
    ]"#;

    let env = get_env_vars();
    let result: Vec<(U256, u8, u64)> = call_contract_method(
        "getStakes",
        (event_id.to_string(), page_size, page),
        abi_json,
        &env.erc20aton_address,
        &env.rpc_url,
    )
    .await?;

    Ok(result)
}

pub async fn get_open_events_list(page_size: u64, page: u64) -> Result<Vec<(String, u8, u64, u8, U256, U256, u8)>> {
    let abi_json = r#"[
        {
            "type": "function",
            "name": "getOpenEventsList",
            "inputs": [
                {"name": "page_size", "type": "uint64", "internalType": "uint64"},
                {"name": "page", "type": "uint64", "internalType": "uint64"}
            ],
            "outputs": [
                {
                    "type": "tuple[]",
                    "internalType": "struct MyContract.Event[]",
                    "components": [
                        {"type": "string", "name": "eventId", "internalType": "string"},
                        {"type": "uint8", "name": "status", "internalType": "uint8"},
                        {"type": "uint64", "name": "startDate", "internalType": "uint64"},
                        {"type": "uint8", "name": "result", "internalType": "uint8"},
                        {"type": "uint256", "name": "totalStakeTeam1", "internalType": "uint256"},
                        {"type": "uint256", "name": "totalStakeTeam2", "internalType": "uint256"},
                        {"type": "uint8", "name": "sportType", "internalType": "uint8"}
                    ]
                }
            ],
            "stateMutability": "view"
        }
    ]"#;

    let env = get_env_vars();
    let result: Vec<(String, u8, u64, u8, U256, U256, u8)> = call_contract_method(
        "getOpenEventsList",
        (page_size, page),
        abi_json,
        &env.erc20aton_address,
        &env.rpc_url,
    )
    .await?;

    Ok(result)
}