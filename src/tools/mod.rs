// pub  fn string_to_bytes32(input: &str) -> [u8; 32] {
//     let mut bytes = [0u8; 32];
//     let bytes = input.as_bytes();
//     let copy_len = bytes.len().min(bytes.len());
//     bytes[..copy_len].copy_from_slice(&bytes[..copy_len]);
//     bytes
// }

use alloy_primitives::FixedBytes;

pub fn string_to_bytes32(input: &str) -> FixedBytes<8> {
    let mut bytes = [0u8; 8];
    let input_bytes = input.as_bytes();
    let copy_len = input_bytes.len().min(8);
    bytes[..copy_len].copy_from_slice(&input_bytes[..copy_len]);
    FixedBytes::<8>::from(bytes)
}
pub fn bytes32_to_string(input: FixedBytes<32>) -> String {
    String::from_utf8_lossy(&input.0).to_string()
}
