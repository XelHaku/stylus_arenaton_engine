// structs/mod.rs

use alloy_primitives::U256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerSummary {
    // pub level: U256,
    // pub eth_balance: U256,
    pub aton_balance: U256,
    // pub unclaimed_commission: U256,
    // pub claimed_commission: U256,
}

// impl PlayerSummary {
//     pub fn new(
//         // level: U256,
//         // eth_balance: U256,
//         aton_balance: U256
//         // unclaimed_commission: U256,
//         // claimed_commission: U256,
//     ) -> Self {
//         Self {
//             // level,
//             // eth_balance,
//             aton_balance,
//             // unclaimed_commission,
//             // claimed_commission,
//         }
//     }
// }
