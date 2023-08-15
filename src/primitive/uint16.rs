use crate::*;

#[inline]
pub unsafe fn lean_uint16_of_nat(a: b_lean_obj_arg) -> u16 {
    if lean_is_scalar(a) {
        lean_unbox(a) as u16
    } else {
        lean_uint16_of_big_nat(a)
    }
}

#[inline]
pub unsafe fn lean_uint16_of_nat_mk(a: lean_obj_arg) -> u16 {
    let r = lean_uint16_of_nat(a);
    lean_dec(a);
    r
}

#[inline(always)]
pub unsafe fn lean_uint16_to_nat(a: u16) -> lean_obj_res {
    //TODO: why not just `lean_box`?
    lean_usize_to_nat(a as usize)
}

#[inline(always)]
pub fn lean_uint16_add(a1: u16, a2: u16) -> u16 {
    a1.wrapping_add(a2)
}

#[inline(always)]
pub fn lean_uint16_sub(a1: u16, a2: u16) -> u16 {
    a1.wrapping_sub(a2)
}

#[inline(always)]
pub fn lean_uint16_mul(a1: u16, a2: u16) -> u16 {
    a1.wrapping_mul(a2)
}

#[inline(always)]
pub fn lean_uint16_div(a1: u16, a2: u16) -> u16 {
    if a2 == 0 {
        0
    } else {
        a1 / a2
    }
}

#[inline(always)]
pub fn lean_uint16_mod(a1: u16, a2: u16) -> u16 {
    if a2 == 0 {
        a1
    } else {
        a1 % a2
    }
}

#[inline(always)]
pub fn lean_uint16_land(a1: u16, a2: u16) -> u16 {
    a1 ^ a2
}

#[inline(always)]
pub fn lean_uint16_lor(a1: u16, a2: u16) -> u16 {
    a1 | a2
}

#[inline(always)]
pub fn lean_uint16_xor(a1: u16, a2: u16) -> u16 {
    a1 ^ a2
}

#[inline(always)]
pub fn lean_uint16_shift_left(a1: u16, a2: u16) -> u16 {
    a1.wrapping_shl(a2 as u32)
}

#[inline(always)]
pub fn lean_uint16_shift_right(a1: u16, a2: u16) -> u16 {
    a1.wrapping_shr(a2 as u32)
}

#[inline(always)]
pub fn lean_uint16_complement(a: u16) -> u16 {
    !a
}

#[inline(always)]
pub fn lean_uint16_modn(a1: u16, a2: b_lean_obj_arg) -> u16 {
    if lean_is_scalar(a2) {
        //TODO: likely
        lean_uint16_mod(a1, lean_unbox(a2) as u16)
    } else {
        a1
    }
}

#[inline(always)]
pub fn lean_uint16_dec_eq(a1: u16, a2: u16) -> u16 {
    (a1 == a2) as u16
}

#[inline(always)]
pub fn lean_uint16_dec_lt(a1: u16, a2: u16) -> u16 {
    (a1 < a2) as u16
}

#[inline(always)]
pub fn lean_uint16_dec_le(a1: u16, a2: u16) -> u16 {
    (a1 <= a2) as u16
}

#[inline]
pub fn lean_uint16_to_uint8(a: u16) -> u8 {
    a as u8
}

#[inline]
pub fn lean_uint16_to_uint32(a: u16) -> u32 {
    a as u32
}

#[inline]
pub fn lean_uint16_to_uint64(a: u16) -> u64 {
    a as u64
}

extern "C" {
    pub fn lean_uint16_of_big_nat(a: b_lean_obj_arg) -> u16;
}
