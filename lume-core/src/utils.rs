
pub fn take_lower(val: u16) -> u8 {
    (val & 0xFF) as u8
}

pub fn take_upper(val:u16) -> u8{
    ((val >> 8) & 0xFF) as u8
}

pub fn take_even(val: u16) -> u8 {
    let even_bits = val & 0x5555; 
    
    let x = even_bits;
    let x = (x | (x >> 1)) & 0x3333;
    let x = (x | (x >> 2)) & 0x0F0F;
    let x = (x | (x >> 4)) & 0x00FF;
    x as u8
}
pub fn usage_to_u16_simple(usage: f32) -> u16 {
    let clamped_usage = usage.min(100.0).max(0.0);
    
    let num_bits = (clamped_usage / 100.0 * 16.0).round() as u32;

    if num_bits == 0 {
        0
    } else if num_bits >= 16 {
        u16::MAX
    } else {
        (1u16 << num_bits) - 1
    }
}

