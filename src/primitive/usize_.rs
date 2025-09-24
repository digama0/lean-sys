use crate::*;

#[inline]
pub unsafe fn lean_usize_of_nat(a: b_lean_obj_arg) -> usize {
    if lean_is_scalar(a) {
        lean_unbox(a)
    } else {
        lean_usize_of_big_nat(a)
    }
}

#[inline]
pub unsafe fn lean_usize_of_nat_mk(a: lean_obj_arg) -> usize {
    let r = lean_usize_of_nat(a);
    lean_dec(a);
    r
}

#[inline(always)]
pub fn lean_usize_add(a1: usize, a2: usize) -> usize {
    a1.wrapping_add(a2)
}

#[inline(always)]
pub fn lean_usize_sub(a1: usize, a2: usize) -> usize {
    a1.wrapping_sub(a2)
}

#[inline(always)]
pub fn lean_usize_mul(a1: usize, a2: usize) -> usize {
    a1.wrapping_mul(a2)
}

#[inline(always)]
pub fn lean_usize_div(a1: usize, a2: usize) -> usize {
    if a2 == 0 {
        0
    } else {
        a1 / a2
    }
}

#[inline(always)]
pub fn lean_usize_mod(a1: usize, a2: usize) -> usize {
    if a2 == 0 {
        a1
    } else {
        a1 % a2
    }
}

#[inline(always)]
pub fn lean_usize_land(a1: usize, a2: usize) -> usize {
    a1 & a2
}

#[inline(always)]
pub fn lean_usize_lor(a1: usize, a2: usize) -> usize {
    a1 | a2
}

#[inline(always)]
pub fn lean_usize_xor(a1: usize, a2: usize) -> usize {
    a1 ^ a2
}

#[inline(always)]
pub fn lean_usize_shift_left(a1: usize, a2: usize) -> usize {
    a1.wrapping_shl(a2 as u32)
}

#[inline(always)]
pub fn lean_usize_shift_right(a1: usize, a2: usize) -> usize {
    a1.wrapping_shr(a2 as u32)
}

#[inline(always)]
pub fn lean_usize_complement(a: usize) -> usize {
    !a
}

#[inline(always)]
pub fn lean_usize_neg(a: usize) -> usize {
    0 - a
}

#[inline(always)]
pub fn lean_usize_dec_eq(a1: usize, a2: usize) -> usize {
    (a1 == a2) as usize
}

#[inline(always)]
pub fn lean_usize_dec_lt(a1: usize, a2: usize) -> usize {
    (a1 < a2) as usize
}

#[inline(always)]
pub fn lean_usize_dec_le(a1: usize, a2: usize) -> usize {
    (a1 <= a2) as usize
}

#[inline]
pub fn lean_usize_to_uint8(a: usize) -> u8 {
    a as u8
}
#[inline]
pub fn lean_usize_to_uint16(a: usize) -> u16 {
    a as u16
}
#[inline]
pub fn lean_usize_to_uint32(a: usize) -> u32 {
    a as u32
}
#[inline]
pub fn lean_usize_to_uint64(a: usize) -> u64 {
    a as u64
}

extern "C" {
    pub fn lean_usize_of_big_nat(a: b_lean_obj_arg) -> usize;
    pub fn lean_usize_mix_hash(a1: usize, a2: usize) -> usize;
}
