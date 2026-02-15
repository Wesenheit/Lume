
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
