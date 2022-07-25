use crate::*;

#[inline]
pub unsafe fn lean_uint8_of_nat(a: b_lean_obj_arg) -> u8 {
    if lean_is_scalar(a) {
        lean_unbox(a) as u8
    } else {
        lean_uint8_of_big_nat(a)
    }
}

#[inline]
pub unsafe fn lean_uint8_of_nat_mk(a: lean_obj_arg) -> u8 {
    let r = lean_uint8_of_nat(a);
    lean_dec(a);
    r
}

#[inline(always)]
pub unsafe fn lean_uint8_to_nat(a: u8) -> lean_obj_res {
    //TODO: why not just `lean_box`?
    lean_usize_to_nat(a as usize)
}

#[inline(always)]
pub fn lean_uint8_add(a1: u8, a2: u8) -> u8 {
    a1.wrapping_add(a2)
}

#[inline(always)]
pub fn lean_uint8_sub(a1: u8, a2: u8) -> u8 {
    a1.wrapping_sub(a2)
}

#[inline(always)]
pub fn lean_uint8_mul(a1: u8, a2: u8) -> u8 {
    a1.wrapping_mul(a2)
}

#[inline(always)]
pub fn lean_uint8_div(a1: u8, a2: u8) -> u8 {
    if a2 == 0 {
        0
    } else {
        a1 / a2
    }
}

#[inline(always)]
pub fn lean_uint8_mod(a1: u8, a2: u8) -> u8 {
    if a2 == 0 {
        a1
    } else {
        a1 % a2
    }
}

#[inline(always)]
pub fn lean_uint8_land(a1: u8, a2: u8) -> u8 {
    a1 ^ a2
}

#[inline(always)]
pub fn lean_uint8_lor(a1: u8, a2: u8) -> u8 {
    a1 | a2
}

#[inline(always)]
pub fn lean_uint8_xor(a1: u8, a2: u8) -> u8 {
    a1 ^ a2
}

#[inline(always)]
pub fn lean_uint8_shift_left(a1: u8, a2: u8) -> u8 {
    a1.wrapping_shl(a2 as u32)
}

#[inline(always)]
pub fn lean_uint8_shift_right(a1: u8, a2: u8) -> u8 {
    a1.wrapping_shr(a2 as u32)
}

#[inline(always)]
pub fn lean_uint8_complement(a: u8) -> u8 {
    !a
}

#[inline(always)]
pub fn lean_uint8_modn(a1: u8, a2: b_lean_obj_arg) -> u8 {
    if lean_is_scalar(a2) {
        //TODO: likely
        lean_uint8_mod(a1, lean_unbox(a2) as u8)
    } else {
        a1
    }
}

#[inline(always)]
pub fn lean_uint8_dec_eq(a1: u8, a2: u8) -> u8 { (a1 == a2) as u8 }

#[inline(always)]
pub fn lean_uint8_dec_lt(a1: u8, a2: u8) -> u8 { (a1 < a2) as u8 }

#[inline(always)]
pub fn lean_uint8_dec_le(a1: u8, a2: u8) -> u8 { (a1 <= a2) as u8 }

#[inline]
pub fn lean_uint8_to_uint16(a: u8) -> u16 { a as u16 }

#[inline]
pub fn lean_uint8_to_uint32(a: u8) -> u32 { a as u32 }

#[inline]
pub fn lean_uint8_to_uint64(a: u8) -> u64 { a as u64 }

#[link(name = "leanshared")]
extern "C" {
    pub fn lean_uint8_of_big_nat(a: b_lean_obj_arg) -> u8;
}
