// test_contracts/src/constants/mod.rs

/// Re-export the `wallets` module.
// pub mod wallets;

/// A submodule to manage environment variables and other constants.
pub mod env_vars {
    use std::env;

    /// A struct to hold the relevant environment variables.
    pub struct EnvVars {
        pub rpc_url: String,
        pub erc20aton_address: String,
        pub engine_address: String,
        pub vault_address: String,
        pub chain_id: u64,
    }

    /// Reads and returns the environment variables in a single struct.
    pub fn get_env_vars() -> EnvVars {
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8547".into());
        let erc20aton_address = env::var("ERC20ATON_ADDRESS")
            .unwrap_or_else(|_| "0x000000000000000000000000000".into());
        let engine_address =
            env::var("ENGINE_ADDRESS").unwrap_or_else(|_| "0x000000000000000000000000000".into());
        let vault_address =
            env::var("VAULT_ADDRESS").unwrap_or_else(|_| "0x000000000000000000000000000".into());
        let chain_id = env::var("CHAIN_ID")
            .unwrap_or_else(|_| "412346".to_string())
            .parse::<u64>()
            .expect("CHAIN_ID is not a valid u64");

        EnvVars {
            rpc_url,
            erc20aton_address,
            engine_address,
            vault_address,
            chain_id,
        }
    }
}
