// #![cfg_attr(not(feature = "export-abi"), no_main)]

// #[cfg(feature = "export-abi")]
// fn main() {
//     stylus_core_events::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
// }

#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

#[cfg(not(any(test, feature = "export-abi")))]
#[no_mangle]
pub extern "C" fn main() {}

#[cfg(feature = "export-abi")]
fn main() {
    stylus_core_events::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}
