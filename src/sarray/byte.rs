/*! ByteArray (special case of Array of Scalars) */
use crate::*;

#[inline]
pub unsafe fn lean_mk_empty_byte_array(capacity: b_lean_obj_arg) -> lean_obj_res {
    if !lean_is_scalar(capacity) {
        lean_internal_panic_out_of_memory()
    }
    lean_alloc_sarray(1, 0, lean_unbox(capacity))
}

#[inline(always)]
pub unsafe fn lean_byte_array_size(a: b_lean_obj_arg) -> lean_obj_res {
    lean_box(lean_sarray_size(a))
}

#[inline(always)]
pub unsafe fn lean_byte_array_uget(a: b_lean_obj_arg, i: usize) -> u8 {
    debug_assert!(i < lean_sarray_size(a));
    *lean_sarray_cptr(a).add(i)
}

#[inline]
pub unsafe fn lean_byte_array_get(a: b_lean_obj_arg, i: b_lean_obj_arg) -> u8 {
    if lean_is_scalar(i) {
        let i = lean_unbox(i);
        if i < lean_sarray_size(a) {
            lean_byte_array_uget(a, i)
        } else {
            0
        }
    } else {
        /* The index must be out of bounds. Otherwise we would be out of memory. */
        0
    }
}

#[inline(always)]
pub unsafe fn lean_byte_array_fget(a: b_lean_obj_arg, i: b_lean_obj_arg) -> u8 {
    lean_byte_array_uget(a, lean_unbox(i))
}

#[inline(always)]
pub unsafe fn lean_byte_array_uset(a: lean_obj_arg, i: usize, v: u8) -> *mut lean_object {
    let r = if lean_is_exclusive(a) {
        a
    } else {
        lean_copy_byte_array(a)
    };
    let it = lean_sarray_cptr(r).add(i);
    *it = v;
    r
}

#[inline]
pub unsafe fn lean_byte_array_set(a: lean_obj_arg, i: b_lean_obj_arg, v: u8) -> *mut lean_object {
    if !lean_is_scalar(i) {
        a
    } else {
        let i = lean_unbox(i);
        if i >= lean_sarray_size(a) {
            a
        } else {
            lean_byte_array_uset(a, i, v)
        }
    }
}

#[inline(always)]
pub unsafe fn lean_byte_array_fset(a: lean_obj_arg, i: b_lean_obj_arg, v: u8) -> *mut lean_object {
    lean_byte_array_uset(a, lean_unbox(i), v)
}

#[link(name = "leanshared")]
extern "C" {
    pub fn lean_byte_array_mk(a: lean_obj_arg) -> lean_obj_res;
    pub fn lean_byte_array_data(a: lean_obj_arg) -> lean_obj_res;
    pub fn lean_copy_byte_array(a: lean_obj_arg) -> lean_obj_res;
    pub fn lean_byte_array_push(a: lean_obj_arg, b: u8) -> lean_obj_res;
}
