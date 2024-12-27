
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Define some persistent storage using the Solidity ABI.
// `Oracle` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Oracle {
    }
}

/// Declare that `Oracle` is a contract with the following external methods.
#[public]
impl Oracle {
  
}