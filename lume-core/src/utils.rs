
pub fn condense_u16_to_u8(val: u16) -> u8 {
    let odd_bits = (val & 0xAAAA) >> 1;
    let even_bits = val & 0x5555;
    let combined = odd_bits | even_bits;

    let mut result: u8 = 0;
    result |= (combined & 0x0001) as u8;       // bit 0
    result |= ((combined & 0x0004) >> 1) as u8; // bit 2 -> 1
    result |= ((combined & 0x0010) >> 2) as u8; // bit 4 -> 2
    result |= ((combined & 0x0040) >> 3) as u8; // bit 6 -> 3
    result |= ((combined & 0x0100) >> 4) as u8; // bit 8 -> 4
    result |= ((combined & 0x0400) >> 5) as u8; // bit 10 -> 5
    result |= ((combined & 0x1000) >> 6) as u8; // bit 12 -> 6
    result |= ((combined & 0x4000) >> 7) as u8; // bit 14 -> 7

    result
}
