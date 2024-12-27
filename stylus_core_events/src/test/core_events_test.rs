#[cfg(test)]
mod tests {
    use crate::CoreEvents;
    use stylus_sdk::{
        alloy_primitives::{address, Address, U256},
        block, msg,
        prelude::*,
    };

    #[motsu::test]
    fn initialize(contract: CoreEvents) {
        let sender = msg::sender();

        assert!(contract.initialize().unwrap_or_default());
        assert!(contract.set_oracle(sender).unwrap_or_default());

        let event_id = "WXHG1234".to_string();
        let start_date = 1735689600u64;

        let event_id2 = "ABCR3570".to_string();
        let start_date2 = 1735689601u64;

        let event_id3 = "XGTY8844".to_string();
        let start_date3 = 1735689901u64;

        assert!(contract
            .add_event(event_id.clone(), start_date)
            .unwrap_or_default());
        assert!(contract
            .add_event(event_id2.clone(), start_date2)
            .unwrap_or_default());
        assert!(contract
            .add_event(event_id3.clone(), start_date3)
            .unwrap_or_default());

  
    }
}
