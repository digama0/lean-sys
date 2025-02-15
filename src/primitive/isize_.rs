use crate::*;

#[inline]
pub unsafe fn lean_isize_of_int(a: b_lean_obj_arg) -> usize {
    if lean_is_scalar(a) {
        lean_scalar_to_int64(a) as isize as usize
    } else {
        lean_isize_of_big_int(a) as usize
    }
}

#[inline]
pub unsafe fn lean_isize_of_nat(a: b_lean_obj_arg) -> usize {
    if lean_is_scalar(a) {
        lean_unbox(a) as isize as usize
    } else {
        lean_isize_of_big_int(a) as usize
    }
}

#[inline(always)]
pub unsafe fn lean_isize_to_int(a: usize) -> lean_obj_res {
    lean_int64_to_int(a as i64)
}

#[inline(always)]
pub fn lean_isize_neg(a: usize) -> usize {
    (a as isize).wrapping_neg() as usize
}

#[inline(always)]
pub fn lean_isize_add(a1: usize, a2: usize) -> usize {
    (a1 as isize).wrapping_add(a2 as isize) as usize
}

#[inline(always)]
pub fn lean_isize_sub(a1: usize, a2: usize) -> usize {
    (a1 as isize).wrapping_sub(a2 as isize) as usize
}

#[inline(always)]
pub fn lean_isize_mul(a1: usize, a2: usize) -> usize {
    (a1 as isize).wrapping_mul(a2 as isize) as usize
}

#[inline(always)]
pub fn lean_isize_div(a1: usize, a2: usize) -> usize {
    if a2 == 0 {
        0
    } else {
        (a1 as isize).wrapping_div(a2 as isize) as usize
    }
}

#[inline(always)]
pub fn lean_isize_mod(a1: usize, a2: usize) -> usize {
    if a2 == 0 {
        a1
    } else {
        (a1 as isize).wrapping_rem(a2 as isize) as usize
    }
}

#[inline(always)]
pub fn lean_isize_land(a1: usize, a2: usize) -> usize {
    ((a1 as isize) & (a2 as isize)) as usize
}

#[inline(always)]
pub fn lean_isize_lor(a1: usize, a2: usize) -> usize {
    ((a1 as isize) | (a2 as isize)) as usize
}

#[inline(always)]
pub fn lean_isize_xor(a1: usize, a2: usize) -> usize {
    ((a1 as isize) ^ (a2 as isize)) as usize
}

#[inline(always)]
pub fn lean_isize_shift_left(a1: usize, a2: usize) -> usize {
    const SIZE: isize = isize::BITS as isize;
    let rhs = (a2 as isize % SIZE + SIZE) % SIZE; // this is smod
    (a1 as isize).wrapping_shl(rhs as u32) as usize
}

#[inline(always)]
pub fn lean_isize_shift_right(a1: usize, a2: usize) -> usize {
    const SIZE: isize = isize::BITS as isize;
    let rhs = (a2 as isize % SIZE + SIZE) % SIZE; // this is smod
    (a1 as isize).wrapping_shr(rhs as u32) as usize
}

#[inline(always)]
pub fn lean_isize_complement(a: usize) -> usize {
    !(a as isize) as usize
}

#[inline(always)]
pub fn lean_isize_dec_eq(a1: usize, a2: usize) -> usize {
    (a1 as isize == a2 as isize) as usize
}

#[inline(always)]
pub fn lean_isize_dec_lt(a1: usize, a2: usize) -> usize {
    ((a1 as isize) < a2 as isize) as usize
}

#[inline(always)]
pub fn lean_isize_dec_le(a1: usize, a2: usize) -> usize {
    (a1 as isize <= a2 as isize) as usize
}

#[inline]
pub fn lean_isize_to_int32(a: usize) -> u32 {
    a as isize as i32 as u32
}
#[inline]
pub fn lean_isize_to_int64(a: usize) -> u64 {
    a as isize as i64 as u64
}

extern "C" {
    pub fn lean_isize_of_big_int(a: b_lean_obj_arg) -> isize;
}
