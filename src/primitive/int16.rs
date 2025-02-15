use crate::*;

#[inline]
pub unsafe fn lean_int16_of_int(a: b_lean_obj_arg) -> u16 {
    if lean_is_scalar(a) {
        lean_scalar_to_int64(a) as i16 as u16
    } else {
        lean_int16_of_big_int(a) as u16
    }
}

#[inline]
pub unsafe fn lean_int16_of_nat(a: b_lean_obj_arg) -> u16 {
    if lean_is_scalar(a) {
        lean_unbox(a) as i16 as u16
    } else {
        lean_int16_of_big_int(a) as u16
    }
}

#[inline(always)]
pub unsafe fn lean_int16_to_int(a: u16) -> lean_obj_res {
    lean_int64_to_int(a as i16 as i64)
}

#[inline(always)]
pub fn lean_int16_neg(a: u16) -> u16 {
    (a as i16).wrapping_neg() as u16
}

#[inline(always)]
pub fn lean_int16_add(a1: u16, a2: u16) -> u16 {
    (a1 as i16).wrapping_add(a2 as i16) as u16
}

#[inline(always)]
pub fn lean_int16_sub(a1: u16, a2: u16) -> u16 {
    (a1 as i16).wrapping_sub(a2 as i16) as u16
}

#[inline(always)]
pub fn lean_int16_mul(a1: u16, a2: u16) -> u16 {
    (a1 as i16).wrapping_mul(a2 as i16) as u16
}

#[inline(always)]
pub fn lean_int16_div(a1: u16, a2: u16) -> u16 {
    if a2 == 0 {
        0
    } else {
        (a1 as i16).wrapping_div(a2 as i16) as u16
    }
}

#[inline(always)]
pub fn lean_int16_mod(a1: u16, a2: u16) -> u16 {
    if a2 == 0 {
        a1
    } else {
        (a1 as i16).wrapping_rem(a2 as i16) as u16
    }
}

#[inline(always)]
pub fn lean_int16_land(a1: u16, a2: u16) -> u16 {
    ((a1 as i16) & (a2 as i16)) as u16
}

#[inline(always)]
pub fn lean_int16_lor(a1: u16, a2: u16) -> u16 {
    ((a1 as i16) | (a2 as i16)) as u16
}

#[inline(always)]
pub fn lean_int16_xor(a1: u16, a2: u16) -> u16 {
    ((a1 as i16) ^ (a2 as i16)) as u16
}

#[inline(always)]
pub fn lean_int16_shift_left(a1: u16, a2: u16) -> u16 {
    let rhs = (a2 as i16 % 16 + 16) % 16; // this is smod 16
    (a1 as i16).wrapping_shl(rhs as u32) as u16
}

#[inline(always)]
pub fn lean_int16_shift_right(a1: u16, a2: u16) -> u16 {
    let rhs = (a2 as i16 % 16 + 16) % 16; // this is smod 16
    (a1 as i16).wrapping_shr(rhs as u32) as u16
}

#[inline(always)]
pub fn lean_int16_complement(a: u16) -> u16 {
    !(a as i16) as u16
}

#[inline(always)]
pub fn lean_int16_dec_eq(a1: u16, a2: u16) -> u16 {
    (a1 as i16 == a2 as i16) as u16
}

#[inline(always)]
pub fn lean_int16_dec_lt(a1: u16, a2: u16) -> u16 {
    ((a1 as i16) < a2 as i16) as u16
}

#[inline(always)]
pub fn lean_int16_dec_le(a1: u16, a2: u16) -> u16 {
    (a1 as i16 <= a2 as i16) as u16
}

#[inline]
pub fn lean_int16_to_int8(a: u16) -> u8 {
    a as i16 as i8 as u8
}
#[inline]
pub fn lean_int16_to_int32(a: u16) -> u32 {
    a as i16 as i32 as u32
}
#[inline]
pub fn lean_int16_to_int64(a: u16) -> u64 {
    a as i16 as i64 as u64
}

extern "C" {
    pub fn lean_int16_of_big_int(a: b_lean_obj_arg) -> i16;
}
