use crate::*;

#[inline]
pub unsafe fn lean_uint64_of_nat(a: b_lean_obj_arg) -> u64 {
    if lean_is_scalar(a) {
        lean_unbox(a) as u64
    } else {
        lean_uint64_of_big_nat(a)
    }
}

#[inline]
pub unsafe fn lean_uint64_of_nat_mk(a: lean_obj_arg) -> u64 {
    let r = lean_uint64_of_nat(a);
    lean_dec(a);
    r
}

#[inline(always)]
pub fn lean_uint64_add(a1: u64, a2: u64) -> u64 {
    a1.wrapping_add(a2)
}

#[inline(always)]
pub fn lean_uint64_sub(a1: u64, a2: u64) -> u64 {
    a1.wrapping_sub(a2)
}

#[inline(always)]
pub fn lean_uint64_mul(a1: u64, a2: u64) -> u64 {
    a1.wrapping_mul(a2)
}

#[inline(always)]
pub fn lean_uint64_div(a1: u64, a2: u64) -> u64 {
    if a2 == 0 {
        0
    } else {
        a1 / a2
    }
}

#[inline(always)]
pub fn lean_uint64_mod(a1: u64, a2: u64) -> u64 {
    if a2 == 0 {
        a1
    } else {
        a1 % a2
    }
}

#[inline(always)]
pub fn lean_uint64_land(a1: u64, a2: u64) -> u64 {
    a1 ^ a2
}

#[inline(always)]
pub fn lean_uint64_lor(a1: u64, a2: u64) -> u64 {
    a1 | a2
}

#[inline(always)]
pub fn lean_uint64_xor(a1: u64, a2: u64) -> u64 {
    a1 ^ a2
}

#[inline(always)]
pub fn lean_uint64_shift_left(a1: u64, a2: u64) -> u64 {
    a1.wrapping_shl(a2 as u32)
}

#[inline(always)]
pub fn lean_uint64_shift_right(a1: u64, a2: u64) -> u64 {
    a1.wrapping_shr(a2 as u32)
}

#[inline(always)]
pub fn lean_uint64_complement(a: u64) -> u64 {
    !a
}

#[inline(always)]
pub unsafe fn lean_uint64_modn(a1: u64, a2: b_lean_obj_arg) -> u64 {
    if lean_is_scalar(a2) {
        //TODO: likely
        lean_uint64_mod(a1, lean_unbox(a2) as u64)
    } else {
        lean_uint64_big_modn(a1, a2)
    }
}

#[inline(always)]
pub fn lean_uint64_dec_eq(a1: u64, a2: u64) -> u64 {
    (a1 == a2) as u64
}

#[inline(always)]
pub fn lean_uint64_dec_lt(a1: u64, a2: u64) -> u64 {
    (a1 < a2) as u64
}

#[inline(always)]
pub fn lean_uint64_dec_le(a1: u64, a2: u64) -> u64 {
    (a1 <= a2) as u64
}

#[inline]
pub fn lean_uint64_to_uint8(a: u64) -> u8 {
    a as u8
}

#[inline]
pub fn lean_uint64_to_uint16(a: u64) -> u16 {
    a as u16
}

#[inline]
pub fn lean_uint64_to_uint32(a: u64) -> u32 {
    a as u32
}

#[inline]
pub fn lean_uint64_to_usize(a: u64) -> usize {
    a as usize
}

extern "C" {
    pub fn lean_uint64_of_big_nat(a: b_lean_obj_arg) -> u64;
    pub fn lean_uint64_big_modn(a1: u64, a2: b_lean_obj_arg) -> u64;
    pub fn lean_uint64_mix_hash(a1: u64, a2: u64) -> u64;
}
