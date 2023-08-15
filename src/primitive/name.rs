/*! Name primitives */
use crate::*;

extern "C" {
    pub fn lean_name_eq(n1: b_lean_obj_arg, n2: b_lean_obj_arg) -> bool;
}

#[inline(always)]
pub unsafe fn lean_name_hash_ptr(n: b_lean_obj_arg) -> u64 {
    debug_assert!(!lean_is_scalar(n));
    lean_ctor_get_uint64(n, (std::mem::size_of::<*mut lean_object>() * 2) as u32)
}

#[inline]
pub unsafe fn lean_name_hash(n: b_lean_obj_arg) -> u64 {
    if lean_is_scalar(n) {
        //TODO: why?
        1273
    } else {
        lean_name_hash_ptr(n)
    }
}
