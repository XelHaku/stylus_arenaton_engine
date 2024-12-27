// tests/stake_engine_test.rs

#[cfg(test)]
mod tests {
    use crate::StakeEngine;
    use stylus_sdk::{
        alloy_primitives::{address, Address, U256},
        block, msg,
        prelude::*,
    };
    //     // If you are not actually using these two, comment them out:
    //     // use crate::test::constants::env_vars::{get_env_vars, EnvVars};
    const VAULT_ADDRESS: &str = "0x7e32B54800705876D3B5CfBC7d9C226A211F7C1A";
    const ATON_ADDRESS: &str = "0xa6e41ffd769491a42a6e5ce453259b93983a22ef";

    #[motsu::test]
    fn initialize(contract: StakeEngine) {
        //   Instead of parse_checksummed, just parse hex ignoring checksums:
        let aton_address = ATON_ADDRESS;
        let parsed: Address = aton_address
            .parse()
            .expect("Should parse valid hex address");

        let vault_address = VAULT_ADDRESS;
        let parsed_vault: Address = vault_address
            .parse()
            .expect("Should parse valid hex address");
        let _ = contract._set_vault_aton(parsed, parsed_vault);

        let _aton_address = contract.aton_address.get();
        let _vault_address = contract.vault_address.get();

        println!(
            "_aton_address: {:?}, _vault_address: {:?}",
            _aton_address, _vault_address
        );
        assert_eq!(_aton_address, parsed);
        assert_eq!(_vault_address, parsed_vault);

        assert!(true);


    }
}
