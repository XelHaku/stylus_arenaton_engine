use ethers::abi::{Abi, Detokenize, Tokenize};
use ethers::prelude::*;
use eyre::{Result, WrapErr};
use serde::de::value;
use serde_json::from_str;
use std::sync::Arc;

/// Generalized function to call a contract method (unsigned)
pub async fn call_contract_method<T: Detokenize>(
    method_name: &str,
    args: impl Tokenize,
    abi_json: &str,
    contract_address_str: &str,
    provider_url: &str,
) -> Result<T> {
    print!(
        "\nCalling '{}' method on contract at address '{}'",
        method_name, contract_address_str
    );
    let provider =
        Provider::<Http>::try_from(provider_url).wrap_err("Failed to create provider")?;
    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    let abi: Abi = from_str(abi_json).wrap_err("Error parsing ABI")?;
    let contract = Contract::new(contract_address, abi, Arc::new(provider));

    let result: T = contract
        .method::<_, T>(method_name, args)?
        .call()
        .await
        .wrap_err(format!("Failed to call '{}' method", method_name))?;

    Ok(result)
}
pub async fn call_contract_method_signed(
    method_name: &str,
    args: impl Tokenize,
    abi_json: &str,
    contract_address_str: &str,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    value: U256,
) -> Result<Option<TransactionReceipt>> {
    let contract_address: Address = contract_address_str
        .parse()
        .wrap_err("Invalid contract address")?;
    let abi: Abi = serde_json::from_str(abi_json).wrap_err("Error parsing ABI")?;
    let contract = Contract::new(contract_address, abi, signer.clone());
    
    print!("\nCalling '{}' method on contract at address '{}'", method_name, contract_address_str);

    // Create method builder
    let method = contract.method::<_, ()>(method_name, args)?;
    
    // Estimate gas requirements
    let gas_estimate = match method.estimate_gas().await {
        Ok(gas) => gas,
        Err(e) => {
            let revert_reason = e
                .decode_revert::<String>()
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(eyre::eyre!(
                "Transaction simulation failed: {}",
                revert_reason
            ));
        }
    };

    // Get current gas price
    let gas_price = signer
        .get_gas_price()
        .await
        .wrap_err("Failed to get gas price")?;

    // Calculate total cost (value + gas cost)
    let total_cost = value + gas_estimate * gas_price;

    // Check account balance
    let signer_address = signer.address();
    let balance = signer
        .get_balance(signer_address, None)
        .await
        .wrap_err("Failed to get account balance")?;

    if balance < total_cost {
        return Err(eyre::eyre!(
            "Insufficient funds. Required: {} wei ({} ETH), Available: {} wei ({} ETH)",
            total_cost,
            format_ether(total_cost),
            balance,
            format_ether(balance)
        ));
    }

    // Send transaction with estimated gas
    let tx = method
        .value(value)
        .gas(gas_estimate)
        .send()
        .await?
        .await
        .wrap_err(format!("Failed to execute '{}' method", method_name))?;

    Ok(tx)
}

// Helper function to format ether values
fn format_ether(wei: U256) -> String {
    let eth: f64 = wei.as_u128() as f64 / 1e18;
    format!("{:.6}", eth)
}