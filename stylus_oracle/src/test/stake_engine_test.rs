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

        // ORACLE
        let sender = msg::sender();
        assert!(!contract.is_oracle());

        assert!(contract.set_oracle(sender));
        assert!(contract.is_oracle());
        // ADD EVENT;

        assert!(contract.is_oracle());

        let event_id = "WXHG1234".to_string();
        let start_date = 1735689600u64;
        let sport = 14u8;

        let event_id2 = "ABCR3570".to_string();
        let start_date2 = 1735689601u64;
        let sport2 = 13u8;

        let event_id3 = "XGTY8844".to_string();
        let start_date3 = 1735689901u64;
        let sport3 = 1u8;
        assert!(contract
            .add_event(event_id, start_date, sport)
            .unwrap_or_default());
        assert!(contract
            .add_event(event_id2, start_date2, sport2)
            .unwrap_or_default());
        assert!(contract
            .add_event(event_id3, start_date3, sport3)
            .unwrap_or_default());

        let _event_count = contract.active_events_length().unwrap_or_default();
        assert!(_event_count == U256::from(3));

        // STAKE
        let _team = 1u8;
        let _amount = 100u64;

        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(2000000000u64), 1u8)
            .unwrap_or_default());
        assert!(contract
            ._add_stake("WXHG1234".to_string(), U256::from(3000000000u64), 1u8)
            .unwrap_or_default());
        // assert!(contract._add_stake("WXHG1234".to_string(), U256::from(1000000000u64), 2u8).unwrap_or_default());
        // END STAKE

        let _event_list: Vec<(String, u8, u64, u8, U256, U256, U256, U256, U256, U256, u8)> =
            contract.get_event_list(100, 0).unwrap_or_default();
        let a: u64 = block::timestamp();
        println!("event_list: {:?}  {:?}", _event_list[0], a);
        // println!("event_list: {:?}  {:?}", _event_list[1], a);
        // println!("event_list: {:?}  {:?}", _event_list[2], a);

        // STAKES LIST
        let _stake_list: Vec<(U256, u8, u64)> = contract
            .get_stakes("WXHG1234".to_string(), 100, 0)
            .unwrap_or_default();
        println!("stakes_list: {:?}  ", _stake_list[0]);
        println!("stakes_list: {:?}  ", _stake_list[1]);
        println!("stakes_list: {:?}  ", _stake_list[2]);
        println!("stakes_list: {:?}  ", _stake_list[3]);
        println!("stakes_list: {:?}  ", _stake_list[4]);
        println!("stakes_list: {:?}  ", _stake_list[5]);
        println!("stakes_list: {:?}  ", _stake_list[6]);
        println!("stakes_list: {:?}  ", _stake_list[7]);
        println!("stakes_list: {:?}  ", _stake_list[8]);
        println!("stakes_list: {:?}  ", _stake_list[9]);
        println!("stakes_list: {:?}  ", _stake_list[10]);
        println!("stakes_list: {:?}  ", _stake_list[11]);
        println!("stakes_list: {:?}  ", _stake_list[12]);
        println!("stakes_list: {:?}  ", _stake_list[13]);
        println!("stakes_list: {:?}  ", _stake_list[14]);
        println!("stakes_list: {:?}  ", _stake_list[15]);
        println!("stakes_list: {:?}  ", _stake_list[16]);
        println!("stakes_list: {:?}  ", _stake_list[17]);
        println!("stakes_list: {:?}  ", _stake_list[18]);
        println!("stakes_list: {:?}  ", _stake_list[19]);
        println!("stakes_list: {:?}  ", _stake_list[20]);
        println!("stakes_list: {:?}  ", _stake_list[21]);
        println!("stakes_list: {:?}  ", _stake_list[22]);
        println!("stakes_list: {:?}  ", _stake_list[23]);
        println!("stakes_list: {:?}  ", _stake_list[24]);
        println!("stakes_list: {:?}  ", _stake_list[25]);
        println!("stakes_list: {:?}  ", _stake_list[26]);

        assert!(!contract.is_oracle());
    }
}
