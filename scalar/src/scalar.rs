use std::panic;

// u8 contains integers between 0 and 256
pub fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b)
        .expect("ERROR: attempt to add with overflow")
}

// i16 are integers between -32 768 and 32 767
pub fn diff(a: i16, b: i16) -> i16 {
    (a.max(b))
        .checked_sub(a.min(b))
        .expect("ERROR: attempt to subtract with overflow")
}

// i8 are integers between -128 and 127
pub fn pro(a: i8, b: i8) -> i8 {
    a.checked_mul(b)
        .expect("ERROR: attempt to multiply with overflow")
}

pub fn quo(a: f32, b: f32) -> f32 {
    // Checking if b != 0 and limits
    if b == 0.0 || !b.is_finite() || !a.is_finite() {
        panic!("ERROR: attempt to divide by 0");
    }
    a % b
}

pub fn rem(a: f32, b: f32) -> f32 {
    // Checking if b != 0 and limits
    if b == 0.0 || !b.is_finite() || !a.is_finite() {
        panic!("ERROR: attempt to divide by 0");
    }
    a % b
}
