use crate::*;

#[inline]
pub unsafe fn lean_int8_of_int(a: b_lean_obj_arg) -> u8 {
    if lean_is_scalar(a) {
        lean_scalar_to_int64(a) as i8 as u8
    } else {
        lean_int8_of_big_int(a) as u8
    }
}

#[inline]
pub unsafe fn lean_int8_of_nat(a: b_lean_obj_arg) -> u8 {
    if lean_is_scalar(a) {
        lean_unbox(a) as i8 as u8
    } else {
        lean_int8_of_big_int(a) as u8
    }
}

#[inline(always)]
pub unsafe fn lean_int8_to_int(a: u8) -> lean_obj_res {
    lean_int64_to_int(a as i8 as i64)
}

#[inline(always)]
pub fn lean_int8_neg(a: u8) -> u8 {
    (a as i8).wrapping_neg() as u8
}

#[inline(always)]
pub fn lean_int8_add(a1: u8, a2: u8) -> u8 {
    (a1 as i8).wrapping_add(a2 as i8) as u8
}

#[inline(always)]
pub fn lean_int8_sub(a1: u8, a2: u8) -> u8 {
    (a1 as i8).wrapping_sub(a2 as i8) as u8
}

#[inline(always)]
pub fn lean_int8_mul(a1: u8, a2: u8) -> u8 {
    (a1 as i8).wrapping_mul(a2 as i8) as u8
}

#[inline(always)]
pub fn lean_int8_div(a1: u8, a2: u8) -> u8 {
    if a2 == 0 {
        0
    } else {
        (a1 as i8).wrapping_div(a2 as i8) as u8
    }
}

#[inline(always)]
pub fn lean_int8_mod(a1: u8, a2: u8) -> u8 {
    if a2 == 0 {
        a1
    } else {
        (a1 as i8).wrapping_rem(a2 as i8) as u8
    }
}

#[inline(always)]
pub fn lean_int8_land(a1: u8, a2: u8) -> u8 {
    ((a1 as i8) & (a2 as i8)) as u8
}

#[inline(always)]
pub fn lean_int8_lor(a1: u8, a2: u8) -> u8 {
    ((a1 as i8) | (a2 as i8)) as u8
}

#[inline(always)]
pub fn lean_int8_xor(a1: u8, a2: u8) -> u8 {
    ((a1 as i8) ^ (a2 as i8)) as u8
}

#[inline(always)]
pub fn lean_int8_shift_left(a1: u8, a2: u8) -> u8 {
    let rhs = (a2 as i8 % 8 + 8) % 8; // this is smod 8
    (a1 as i8).wrapping_shl(rhs as u32) as u8
}

#[inline(always)]
pub fn lean_int8_shift_right(a1: u8, a2: u8) -> u8 {
    let rhs = (a2 as i8 % 8 + 8) % 8; // this is smod 8
    (a1 as i8).wrapping_shr(rhs as u32) as u8
}

#[inline(always)]
pub fn lean_int8_complement(a: u8) -> u8 {
    !(a as i8) as u8
}

#[inline(always)]
pub fn lean_int8_abs(a: u8) -> u8 {
    (a as i8).unsigned_abs()
}

#[inline(always)]
pub fn lean_int8_dec_eq(a1: u8, a2: u8) -> u8 {
    (a1 as i8 == a2 as i8) as u8
}

#[inline(always)]
pub fn lean_int8_dec_lt(a1: u8, a2: u8) -> u8 {
    ((a1 as i8) < a2 as i8) as u8
}

#[inline(always)]
pub fn lean_int8_dec_le(a1: u8, a2: u8) -> u8 {
    (a1 as i8 <= a2 as i8) as u8
}

#[inline]
pub fn lean_int8_to_int16(a: u8) -> u16 {
    a as i8 as i16 as u16
}
#[inline]
pub fn lean_int8_to_int32(a: u8) -> u32 {
    a as i8 as i32 as u32
}
#[inline]
pub fn lean_int8_to_int64(a: u8) -> u64 {
    a as i8 as i64 as u64
}

#[inline]
pub fn lean_int8_to_isize(a: u8) -> usize {
    a as i8 as isize as usize
}

extern "C" {
    pub fn lean_int8_of_big_int(a: b_lean_obj_arg) -> i8;
}
