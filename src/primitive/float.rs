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
pub fn lean_float_to_int8(a: f64) -> u8 {
    //NOTE: this performs a saturating cast, as desired
    a as i8 as u8
}
pub fn lean_float_to_int16(a: f64) -> u16 {
    //NOTE: this performs a saturating cast, as desired
    a as i16 as u16
}
pub fn lean_float_to_int32(a: f64) -> u32 {
    //NOTE: this performs a saturating cast, as desired
    a as i32 as u32
}
pub fn lean_float_to_int64(a: f64) -> u64 {
    //NOTE: this performs a saturating cast, as desired
    a as i64 as u64
}
pub fn lean_float_to_isize(a: f64) -> usize {
    //NOTE: this performs a saturating cast, as desired
    a as isize as usize
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
pub fn lean_uint8_to_float(a: u8) -> f64 {
    a as f64
}
pub fn lean_uint16_to_float(a: u16) -> f64 {
    a as f64
}
pub fn lean_uint32_to_float(a: u32) -> f64 {
    a as f64
}
pub fn lean_uint64_to_float(a: u64) -> f64 {
    a as f64
}
pub fn lean_usize_to_float(a: usize) -> f64 {
    a as f64
}
pub fn lean_int8_to_float(a: u8) -> f64 {
    a as i8 as f64
}
pub fn lean_int16_to_float(a: u16) -> f64 {
    a as i16 as f64
}
pub fn lean_int32_to_float(a: u32) -> f64 {
    a as i32 as f64
}
pub fn lean_int64_to_float(a: u64) -> f64 {
    a as i64 as f64
}
pub fn lean_isize_to_float(a: usize) -> f64 {
    a as isize as f64
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
