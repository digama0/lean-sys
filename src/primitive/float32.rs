use crate::*;

pub fn lean_float32_to_uint8(a: f32) -> u8 {
    //NOTE: this performs a saturating cast, as desired
    a as u8
}
pub fn lean_float32_to_uint16(a: f32) -> u16 {
    //NOTE: this performs a saturating cast, as desired
    a as u16
}
pub fn lean_float32_to_uint32(a: f32) -> u32 {
    //NOTE: this performs a saturating cast, as desired
    a as u32
}
pub fn lean_float32_to_uint64(a: f32) -> u64 {
    //NOTE: this performs a saturating cast, as desired
    a as u64
}
pub fn lean_float32_to_usize(a: f32) -> usize {
    //NOTE: this performs a saturating cast, as desired
    a as usize
}
pub fn lean_float32_to_int8(a: f32) -> u8 {
    //NOTE: this performs a saturating cast, as desired
    a as i8 as u8
}
pub fn lean_float32_to_int16(a: f32) -> u16 {
    //NOTE: this performs a saturating cast, as desired
    a as i16 as u16
}
pub fn lean_float32_to_int32(a: f32) -> u32 {
    //NOTE: this performs a saturating cast, as desired
    a as i32 as u32
}
pub fn lean_float32_to_int64(a: f32) -> u64 {
    //NOTE: this performs a saturating cast, as desired
    a as i64 as u64
}
pub fn lean_float32_to_isize(a: f32) -> usize {
    //NOTE: this performs a saturating cast, as desired
    a as isize as usize
}
pub fn lean_float32_add(a: f32, b: f32) -> f32 {
    a + b
}
pub fn lean_float32_sub(a: f32, b: f32) -> f32 {
    a - b
}
pub fn lean_float32_mul(a: f32, b: f32) -> f32 {
    a * b
}
pub fn lean_float32_div(a: f32, b: f32) -> f32 {
    a / b
}
pub fn lean_float32_negate(a: f32) -> f32 {
    -a
}
pub fn lean_float32_beq(a: f32, b: f32) -> u8 {
    (a == b) as u8
}
pub fn lean_float32_decLe(a: f32, b: f32) -> u8 {
    (a <= b) as u8
}
pub fn lean_float32_decLt(a: f32, b: f32) -> u8 {
    (a < b) as u8
}
pub fn lean_uint8_to_float32(a: u8) -> f32 {
    a as f32
}
pub fn lean_uint16_to_float32(a: u16) -> f32 {
    a as f32
}
pub fn lean_uint32_to_float32(a: u32) -> f32 {
    a as f32
}
pub fn lean_uint64_to_float32(a: u64) -> f32 {
    a as f32
}
pub fn lean_usize_to_float32(a: usize) -> f32 {
    a as f32
}
pub fn lean_int8_to_float32(a: u8) -> f32 {
    a as i8 as f32
}
pub fn lean_int16_to_float32(a: u16) -> f32 {
    a as i16 as f32
}
pub fn lean_int32_to_float32(a: u32) -> f32 {
    a as i32 as f32
}
pub fn lean_int64_to_float32(a: u64) -> f32 {
    a as i64 as f32
}
pub fn lean_isize_to_float32(a: usize) -> f32 {
    a as isize as f32
}

pub fn lean_float_to_float32(a: f64) -> f32 {
    a as f32
}
pub fn lean_float32_to_float(a: f32) -> f64 {
    a as f64
}

extern "C" {
    pub fn lean_float32_to_string(a: f32) -> lean_obj_res;
    pub fn lean_float32_scaleb(a: f32, b: b_lean_obj_arg) -> f32;
    pub fn lean_float32_isnan(a: f32) -> u8;
    pub fn lean_float32_isfinite(a: f32) -> u8;
    pub fn lean_float32_isinf(a: f32) -> u8;
    pub fn lean_float32_frexp(a: f32) -> lean_obj_res;
    pub fn lean_float32_of_bits(a: u32) -> f32;
    pub fn lean_float32_to_bits(a: f32) -> u32;
}
