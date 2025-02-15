use crate::*;

#[inline]
pub unsafe fn lean_uint32_of_nat(a: b_lean_obj_arg) -> u32 {
    if lean_is_scalar(a) {
        lean_unbox(a) as u32
    } else {
        lean_uint32_of_big_nat(a)
    }
}

#[inline]
pub unsafe fn lean_uint32_of_nat_mk(a: lean_obj_arg) -> u32 {
    let r = lean_uint32_of_nat(a);
    lean_dec(a);
    r
}

#[inline(always)]
pub unsafe fn lean_uint32_to_nat(a: u32) -> lean_obj_res {
    lean_usize_to_nat(a as usize)
}

#[inline(always)]
pub fn lean_uint32_add(a1: u32, a2: u32) -> u32 {
    a1.wrapping_add(a2)
}

#[inline(always)]
pub fn lean_uint32_sub(a1: u32, a2: u32) -> u32 {
    a1.wrapping_sub(a2)
}

#[inline(always)]
pub fn lean_uint32_mul(a1: u32, a2: u32) -> u32 {
    a1.wrapping_mul(a2)
}

#[inline(always)]
pub fn lean_uint32_div(a1: u32, a2: u32) -> u32 {
    if a2 == 0 {
        0
    } else {
        a1 / a2
    }
}

#[inline(always)]
pub fn lean_uint32_mod(a1: u32, a2: u32) -> u32 {
    if a2 == 0 {
        a1
    } else {
        a1 % a2
    }
}

#[inline(always)]
pub fn lean_uint32_land(a1: u32, a2: u32) -> u32 {
    a1 & a2
}

#[inline(always)]
pub fn lean_uint32_lor(a1: u32, a2: u32) -> u32 {
    a1 | a2
}

#[inline(always)]
pub fn lean_uint32_xor(a1: u32, a2: u32) -> u32 {
    a1 ^ a2
}

#[inline(always)]
pub fn lean_uint32_shift_left(a1: u32, a2: u32) -> u32 {
    a1.wrapping_shl(a2)
}

#[inline(always)]
pub fn lean_uint32_shift_right(a1: u32, a2: u32) -> u32 {
    a1.wrapping_shr(a2)
}

#[inline(always)]
pub fn lean_uint32_complement(a: u32) -> u32 {
    !a
}

#[inline(always)]
pub fn lean_uint32_dec_eq(a1: u32, a2: u32) -> u32 {
    (a1 == a2) as u32
}

#[inline(always)]
pub fn lean_uint32_dec_lt(a1: u32, a2: u32) -> u32 {
    (a1 < a2) as u32
}

#[inline(always)]
pub fn lean_uint32_dec_le(a1: u32, a2: u32) -> u32 {
    (a1 <= a2) as u32
}

#[inline]
pub fn lean_uint32_to_uint8(a: u32) -> u8 {
    a as u8
}
#[inline]
pub fn lean_uint32_to_uint16(a: u32) -> u16 {
    a as u16
}
#[inline]
pub fn lean_uint32_to_uint64(a: u32) -> u64 {
    a as u64
}
#[inline]
pub fn lean_uint32_to_usize(a: u32) -> usize {
    a as usize
}

extern "C" {
    pub fn lean_uint32_of_big_nat(a: b_lean_obj_arg) -> u32;
}
