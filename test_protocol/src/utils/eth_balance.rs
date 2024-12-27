use ethers::prelude::*;
use eyre::Result;
use eyre::WrapErr;

use crate::constants::env_vars::{get_env_vars, EnvVars};

pub async fn eth_balance(address: &str) -> Result<U256> {
    let env = get_env_vars();
    let rpc_url = env.rpc_url;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let _address: Address = address.parse().wrap_err("Invalid contract address")?;
    let balance = provider.get_balance(_address, None).await?;
    println!("\nETH balance of {:?}: {:?}", address, balance);
    Ok(balance)
}
