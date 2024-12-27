// test_contracts/src/constants/mod.rs

/// Re-export the `wallets` module.
pub mod wallets;

/// A submodule to manage environment variables and other constants.
pub mod env_vars {
    use std::env;

    /// A struct to hold the relevant environment variables.
    pub struct EnvVars {
        pub rpc_url: String,
        pub erc20aton_address: String,
        pub vault_address: String,
        pub core_address: String,
        pub stake_address: String,
        pub whale_private_key: String,
        pub chain_id: u64,
    }

    /// Reads and returns the environment variables in a single struct.
    pub fn get_env_vars() -> EnvVars {
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8547".into());
        let erc20aton_address = env::var("ERC20ATON_ADDRESS")
            .unwrap_or_else(|_| "0xa6e41ffd769491a42a6e5ce453259b93983a22ef".into());
        let vault_address =
            env::var("VAULT_ADDRESS").unwrap_or_else(|_| "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a".into());
            let core_address = 
            env::var("CORE_ADDRESS").unwrap_or_else(|_| "0x85d9a8a4bd77b9b5559c1b7fcb8ec9635922ed49".into());
           
           
            let stake_address =
            env::var("STAKE_ENGINE_ADDRESS").unwrap_or_else(|_| "0x4af567288e68cad4aa93a272fe6139ca53859c70".into());
      
              let whale_private_key =
            env::var("WHALE_PRIVATE_KEY").unwrap_or_else(|_| "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659".into());
      
      
      
      
        let chain_id = env::var("CHAIN_ID")
            .unwrap_or_else(|_| "412346".to_string())
            .parse::<u64>()
            .expect("CHAIN_ID is not a valid u64");

        EnvVars {
            rpc_url,
            erc20aton_address,
            vault_address,
            core_address,
            stake_address,
            whale_private_key,
            chain_id,
        }
    }
}

// local_deploy.sh
// ERC20ATON_ADDRESS=0xa6e41ffd769491a42a6e5ce453259b93983a22ef
// VAULT_ADDRESS=0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a
// CORE_ADDRESS=0x85d9a8a4bd77b9b5559c1b7fcb8ec9635922ed49
// STAKE_ENGINE_ADDRESS=0x4af567288e68cad4aa93a272fe6139ca53859c70