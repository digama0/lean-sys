use crate::*;

pub fn lean_float_to_uint8(a: f64) -> u8 {
    //NOTE: this performs a saturating cast, as desired
    a as u8
}
pub fn lean_float_to_uint16(a: f64) -> u16 {
    //NOTE: this performs a saturating cast, as desired
    a as u16
}
pub fn lean_float_to_uint32(a: f64) -> u32 {
    //NOTE: this performs a saturating cast, as desired
    a as u32
}
pub fn lean_float_to_uint64(a: f64) -> u64 {
    //NOTE: this performs a saturating cast, as desired
    a as u64
}
pub fn lean_float_to_usize(a: f64) -> usize {
    //NOTE: this performs a saturating cast, as desired
    a as usize
}
pub fn lean_float_add(a: f64, b: f64) -> f64 {
    a + b
}
pub fn lean_float_sub(a: f64, b: f64) -> f64 {
    a - b
}
pub fn lean_float_mul(a: f64, b: f64) -> f64 {
    a * b
}
pub fn lean_float_div(a: f64, b: f64) -> f64 {
    a / b
}
pub fn lean_float_negate(a: f64) -> f64 {
    -a
}
pub fn lean_float_beq(a: f64, b: f64) -> u8 {
    (a == b) as u8
}
pub fn lean_float_decLe(a: f64, b: f64) -> u8 {
    (a <= b) as u8
}
pub fn lean_float_decLt(a: f64, b: f64) -> u8 {
    (a < b) as u8
}
pub fn lean_uint64_to_float(a: u64) -> f64 {
    a as f64
}

extern "C" {
    pub fn lean_float_to_string(a: f64) -> lean_obj_res;
    pub fn lean_float_scaleb(a: f64, b: b_lean_obj_arg) -> f64;
    pub fn lean_float_isnan(a: f64) -> u8;
    pub fn lean_float_isfinite(a: f64) -> u8;
    pub fn lean_float_isinf(a: f64) -> u8;
    pub fn lean_float_frexp(a: f64) -> lean_obj_res;
    pub fn lean_float_of_bits(a: u64) -> f64;
    pub fn lean_float_to_bits(a: f64) -> u64;
}
