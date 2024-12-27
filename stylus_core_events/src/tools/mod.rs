use stylus_sdk::alloy_primitives::FixedBytes;

pub fn string_to_bytes8(input: &str) -> FixedBytes<8> {
    // Utilize Rust's standard library more efficiently by only handling up to the needed length
    let mut bytes = [0u8; 8];
    if let Some(slice) = input.as_bytes().get(..8) {
        bytes[..slice.len()].copy_from_slice(slice);
    }
    FixedBytes::<8>::from(bytes)
}

pub fn bytes8_to_string(input: FixedBytes<8>) -> String {
    // Handle potential trailing zeros by trimming them
    if let Some(end) = input.0.iter().position(|&b| b == 0) {
        String::from_utf8_lossy(&input.0[..end]).to_string()
    } else {
        String::from_utf8_lossy(&input.0).to_string()
    }
}