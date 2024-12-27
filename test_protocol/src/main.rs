// main.rs
// Arenaton Sports Betting Platform - Integration Test Suite
// Implements core functionality described in the whitepaper

/* System Architecture Components:
- ERC20 (ATON): Platform utility token with dynamic supply
- Vault: Commission management & reward distribution
- CoreEvents: Event lifecycle management
- StakeEngine: Parimutuel betting implementation
- Oracle: Result verification system
*/

mod core_events;
mod stake_engine;
mod call_contract;
mod constants;
mod erc20aton;
mod utils;
mod vault;

use crate::utils::eth_balance::eth_balance;
use crate::utils::{fund_player_eth, get_block_time, sleep_ms};
use crate::utils::assert_contract;
use erc20aton::{name, total_supply};

use constants::env_vars::{get_env_vars, EnvVars};
use constants::wallets::{Wallet, WALLETS};
use ethers::types::{U256, Address};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Environment Setup
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    // 2. Load contract addresses from deployment (run ./local_deploy.sh first)
    let env = get_env_vars();
    println!("[Network Configuration]");
    println!("RPC URL: {}", env.rpc_url);
    println!("ATON Token: {}", env.erc20aton_address);
    println!("Vault Contract: {}", env.vault_address);
    println!("Core Events: {}", env.core_address);
    println!("Stake Engine: {}", env.stake_address);
    println!("Chain ID: {}", env.chain_id);

    // 3. Initialize Participant Wallets
    let _owner_wallet = &WALLETS[0];   // Contract administrator
    let _oracle_wallet = &WALLETS[10]; // Verified result provider
    let _player1_wallet = &WALLETS[1]; // Betting participant
    // let _player2_wallet = &WALLETS[2];
    // let _player3_wallet = &WALLETS[3];
    // let _player4_wallet = &WALLETS[4];

    // 4. Core Contract Initialization Sequence
    println!("\n[Phase 1: System Initialization]");
    
    // 4.1 Validate ATON ERC20 Pre-Initialization State
    assert_contract(
        erc20aton::name().await? == "ATON Stylus",
        "Token contract name verification failed"
    ).await?;
    
    assert_contract(
        erc20aton::total_supply().await? == U256::zero(),
        "Initial token supply must be zero"
    ).await?;

    // 4.2 Fund Test Accounts with ETH
    fund_player_eth("100000000000000000000", _owner_wallet.address).await?; // Owner
    fund_player_eth("100000000000000000", _oracle_wallet.address).await?;   // Oracle
    fund_player_eth("10000000000000000", _player1_wallet.address).await?;   // Player 1

    // 4.3 Initialize ATON ERC20 Contract
    erc20aton::initialize(_owner_wallet).await?;
    assert_contract(
        erc20aton::owner().await? == _owner_wallet.address.parse()?,
        "Contract ownership transfer failed"
    ).await?;

    // 4.4 Initialize Vault & Configure Token Relationship
    vault::initialize(&env.erc20aton_address, _owner_wallet).await?;
    erc20aton::set_vault(&env.vault_address, _owner_wallet).await?;
    println!("Vault configured: {}", erc20aton::vault().await?);

    // 4.5 Initialize Core Subsystems
    core_events::initialize(_owner_wallet).await?;
    stake_engine::initialize(&env.erc20aton_address, &env.core_address, _owner_wallet).await?;
    erc20aton::update_stake_engine(&env.stake_address, true,_owner_wallet).await?;
    core_events::set_oracle(_oracle_wallet.address, _owner_wallet).await?;

    // 5. Parimutuel Event Lifecycle Test
    println!("\n[Phase 2: Event Simulation]");
    
    // 5.1 Create New Betting Event (1000 blocks duration)
    let latest_block_time = get_block_time().await?;
    let event_id = "WXHG1224".to_string();
    core_events::add_event(&event_id, latest_block_time + 1000u64, _oracle_wallet).await?;
    println!("Event {} created | Ends at block {}", event_id, latest_block_time + 1000);

    // 5.2 Player Interaction Sequence
    println!("\n[Betting Phase]");
    stake_engine::read_event_core(&event_id, _player1_wallet).await?; // Sync event data
    eth_balance(_player1_wallet.address).await?;
    
    // Place sample bets (Parimutuel pool creation)
    stake_engine::stake(&event_id, U256::from(0), U256::from(10), 1, _player1_wallet).await?;
    // stake_engine::stake(&event_id, U256::from(0), U256::from(2), 2, _player2_wallet).await?;
    eth_balance(_player1_wallet.address).await?;
    
    // Verify token emission from bets
    assert_contract(
        erc20aton::total_supply().await? > U256::zero(),
        "ATON tokens should be minted from stakes"
    ).await?;

    // // 5.3 Event Monitoring & Status Checks
    // println!("\n[Event Status Checks]");
    // stake_engine::get_open_events_list(100u64, 0u64).await?;
    // stake_engine::get_stakes(&event_id, 100u64, 0u64).await?;

    // // 5.4 Oracle Resolution & Payouts
    // println!("\n[Event Resolution]");
    // sleep_ms(2000); // Simulate time passage
    // core_events::close_event(&event_id, 2, _oracle_wallet).await?; // Oracle declares outcome
    
    // // Verify final balances and payouts
    // stake_engine::read_event_core(&event_id, _player1_wallet).await?;
    // eth_balance(_player1_wallet.address).await?;

    println!("\n[Test Completed Successfully]");
    Ok(())
}