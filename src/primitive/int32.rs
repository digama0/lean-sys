use crate::*;

#[inline]
pub unsafe fn lean_int32_of_int(a: b_lean_obj_arg) -> u32 {
    if lean_is_scalar(a) {
        lean_scalar_to_int64(a) as i32 as u32
    } else {
        lean_int32_of_big_int(a) as u32
    }
}

#[inline]
pub unsafe fn lean_int32_of_nat(a: b_lean_obj_arg) -> u32 {
    if lean_is_scalar(a) {
        lean_unbox(a) as i32 as u32
    } else {
        lean_int32_of_big_int(a) as u32
    }
}

#[inline(always)]
pub unsafe fn lean_int32_to_int(a: u32) -> lean_obj_res {
    lean_int64_to_int(a as i32 as i64)
}

#[inline(always)]
pub fn lean_int32_neg(a: u32) -> u32 {
    (a as i32).wrapping_neg() as u32
}

#[inline(always)]
pub fn lean_int32_add(a1: u32, a2: u32) -> u32 {
    (a1 as i32).wrapping_add(a2 as i32) as u32
}

#[inline(always)]
pub fn lean_int32_sub(a1: u32, a2: u32) -> u32 {
    (a1 as i32).wrapping_sub(a2 as i32) as u32
}

#[inline(always)]
pub fn lean_int32_mul(a1: u32, a2: u32) -> u32 {
    (a1 as i32).wrapping_mul(a2 as i32) as u32
}

#[inline(always)]
pub fn lean_int32_div(a1: u32, a2: u32) -> u32 {
    if a2 == 0 {
        0
    } else {
        (a1 as i32).wrapping_div(a2 as i32) as u32
    }
}

#[inline(always)]
pub fn lean_int32_mod(a1: u32, a2: u32) -> u32 {
    if a2 == 0 {
        a1
    } else {
        (a1 as i32).wrapping_rem(a2 as i32) as u32
    }
}

#[inline(always)]
pub fn lean_int32_land(a1: u32, a2: u32) -> u32 {
    ((a1 as i32) & (a2 as i32)) as u32
}

#[inline(always)]
pub fn lean_int32_lor(a1: u32, a2: u32) -> u32 {
    ((a1 as i32) | (a2 as i32)) as u32
}

#[inline(always)]
pub fn lean_int32_xor(a1: u32, a2: u32) -> u32 {
    ((a1 as i32) ^ (a2 as i32)) as u32
}

#[inline(always)]
pub fn lean_int32_shift_left(a1: u32, a2: u32) -> u32 {
    let rhs = (a2 as i32 % 32 + 32) % 32; // this is smod 32
    (a1 as i32).wrapping_shl(rhs as u32) as u32
}

#[inline(always)]
pub fn lean_int32_shift_right(a1: u32, a2: u32) -> u32 {
    let rhs = (a2 as i32 % 32 + 32) % 32; // this is smod 32
    (a1 as i32).wrapping_shr(rhs as u32) as u32
}

#[inline(always)]
pub fn lean_int32_complement(a: u32) -> u32 {
    !(a as i32) as u32
}

#[inline(always)]
pub fn lean_int32_dec_eq(a1: u32, a2: u32) -> u32 {
    (a1 as i32 == a2 as i32) as u32
}

#[inline(always)]
pub fn lean_int32_dec_lt(a1: u32, a2: u32) -> u32 {
    ((a1 as i32) < a2 as i32) as u32
}

#[inline(always)]
pub fn lean_int32_dec_le(a1: u32, a2: u32) -> u32 {
    (a1 as i32 <= a2 as i32) as u32
}

#[inline]
pub fn lean_int32_to_int8(a: u32) -> u8 {
    a as i32 as i8 as u8
}
#[inline]
pub fn lean_int32_to_int16(a: u32) -> u16 {
    a as i32 as i16 as u16
}
#[inline]
pub fn lean_int32_to_int64(a: u32) -> u64 {
    a as i32 as i64 as u64
}
#[inline]
pub fn lean_int32_to_isize(a: u32) -> usize {
    a as i32 as isize as usize
}

extern "C" {
    pub fn lean_int32_of_big_int(a: b_lean_obj_arg) -> i32;
}
