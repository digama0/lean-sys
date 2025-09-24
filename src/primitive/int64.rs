use crate::*;

#[inline]
pub unsafe fn lean_int64_of_int(a: b_lean_obj_arg) -> u64 {
    if lean_is_scalar(a) {
        lean_scalar_to_int64(a) as u64
    } else {
        lean_int64_of_big_int(a) as u64
    }
}

#[inline]
pub unsafe fn lean_int64_of_nat(a: b_lean_obj_arg) -> u64 {
    if lean_is_scalar(a) {
        lean_unbox(a) as i64 as u64
    } else {
        lean_int64_of_big_int(a) as u64
    }
}

#[inline(always)]
pub unsafe fn lean_int64_to_int_sint(a: u64) -> lean_obj_res {
    lean_int64_to_int(a as i64)
}

#[inline(always)]
pub fn lean_int64_neg(a: u64) -> u64 {
    (a as i64).wrapping_neg() as u64
}

#[inline(always)]
pub fn lean_int64_add(a1: u64, a2: u64) -> u64 {
    (a1 as i64).wrapping_add(a2 as i64) as u64
}

#[inline(always)]
pub fn lean_int64_sub(a1: u64, a2: u64) -> u64 {
    (a1 as i64).wrapping_sub(a2 as i64) as u64
}

#[inline(always)]
pub fn lean_int64_mul(a1: u64, a2: u64) -> u64 {
    (a1 as i64).wrapping_mul(a2 as i64) as u64
}

#[inline(always)]
pub fn lean_int64_div(a1: u64, a2: u64) -> u64 {
    if a2 == 0 {
        0
    } else {
        (a1 as i64).wrapping_div(a2 as i64) as u64
    }
}

#[inline(always)]
pub fn lean_int64_mod(a1: u64, a2: u64) -> u64 {
    if a2 == 0 {
        a1
    } else {
        (a1 as i64).wrapping_rem(a2 as i64) as u64
    }
}

#[inline(always)]
pub fn lean_int64_land(a1: u64, a2: u64) -> u64 {
    ((a1 as i64) & (a2 as i64)) as u64
}

#[inline(always)]
pub fn lean_int64_lor(a1: u64, a2: u64) -> u64 {
    ((a1 as i64) | (a2 as i64)) as u64
}

#[inline(always)]
pub fn lean_int64_xor(a1: u64, a2: u64) -> u64 {
    ((a1 as i64) ^ (a2 as i64)) as u64
}

#[inline(always)]
pub fn lean_int64_shift_left(a1: u64, a2: u64) -> u64 {
    let rhs = (a2 as i64 % 64 + 64) % 64; // this is smod 64
    (a1 as i64).wrapping_shl(rhs as u32) as u64
}

#[inline(always)]
pub fn lean_int64_shift_right(a1: u64, a2: u64) -> u64 {
    let rhs = (a2 as i64 % 64 + 64) % 64; // this is smod 64
    (a1 as i64).wrapping_shr(rhs as u32) as u64
}

#[inline(always)]
pub fn lean_int64_complement(a: u64) -> u64 {
    !(a as i64) as u64
}

#[inline(always)]
pub fn lean_int64_abs(a: u64) -> u64 {
    (a as i64).unsigned_abs()
}

#[inline(always)]
pub fn lean_int64_dec_eq(a1: u64, a2: u64) -> u64 {
    (a1 as i64 == a2 as i64) as u64
}

#[inline(always)]
pub fn lean_int64_dec_lt(a1: u64, a2: u64) -> u64 {
    ((a1 as i64) < a2 as i64) as u64
}

#[inline(always)]
pub fn lean_int64_dec_le(a1: u64, a2: u64) -> u64 {
    (a1 as i64 <= a2 as i64) as u64
}

#[inline]
pub fn lean_int64_to_int8(a: u64) -> u8 {
    a as i64 as i8 as u8
}
#[inline]
pub fn lean_int64_to_int16(a: u64) -> u16 {
    a as i64 as i16 as u16
}
#[inline]
pub fn lean_int64_to_int32(a: u64) -> u32 {
    a as i64 as i32 as u32
}
#[inline]
pub fn lean_int64_to_isize(a: u64) -> usize {
    a as i64 as isize as usize
}

extern "C" {
    pub fn lean_int64_of_big_int(a: b_lean_obj_arg) -> i64;
}
