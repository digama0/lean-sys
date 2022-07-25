/*! Arrays of objects (high level API) */
use crate::*;

#[inline]
pub unsafe fn lean_array_sz(a: lean_obj_arg) -> *mut lean_object {
    let r = lean_box(lean_array_size(a));
    lean_dec(a);
    r
}

#[inline(always)]
pub unsafe fn lean_array_get_size(a: b_lean_obj_arg) -> *mut lean_object {
    lean_box(lean_array_size(a))
}

#[inline(always)]
pub unsafe fn lean_mk_empty_array() -> *mut lean_object {
    lean_alloc_array(0, 0)
}

#[inline(always)]
pub unsafe fn lean_mk_empty_array_with_capacity(capacity: b_lean_obj_arg) -> *mut lean_object {
    if !lean_is_scalar(capacity) {
        lean_internal_panic_out_of_memory()
    }
    lean_alloc_array(0, lean_unbox(capacity))
}

#[inline]
pub unsafe fn lean_array_uget(a: b_lean_obj_arg, i: usize) -> lean_obj_res {
    let r = lean_array_get_core(a, i);
    lean_inc(r);
    r
}

#[inline(always)]
pub unsafe fn lean_array_fget(a: b_lean_obj_arg, i: b_lean_obj_arg) -> lean_obj_res {
    lean_array_uget(a, lean_unbox(i))
}

#[inline]
pub unsafe fn lean_array_get(
    def_val: b_lean_obj_arg,
    a: b_lean_obj_arg,
    i: b_lean_obj_arg,
) -> lean_obj_res {
    if lean_is_scalar(i) {
        let idx = lean_unbox(i);
        if idx < lean_array_size(a) {
            lean_dec(def_val);
            return lean_array_uget(a, idx);
        }
    }
    /* Recall that if `i` is not a scalar, then it must be out of bounds because
    i > LEAN_MAX_SMALL_NAT == MAX_UNSIGNED >> 1
    but each array entry is 8 bytes in 64-bit machines and 4 in 32-bit ones.
    In both cases, we would be out-of-memory. */
    lean_array_get_panic(def_val)
}

#[inline(always)]
pub unsafe fn lean_copy_array(a: lean_obj_arg) -> lean_obj_res {
    lean_copy_expand_array(a, false)
}

#[inline(always)]
pub unsafe fn lean_ensure_exclusive_array(a: lean_obj_arg) -> lean_obj_res {
    if lean_is_exclusive(a) {
        a
    } else {
        lean_copy_array(a)
    }
}

#[inline]
pub unsafe fn lean_array_uset(a: lean_obj_arg, i: usize, v: lean_obj_arg) -> *mut lean_object {
    let r = lean_ensure_exclusive_array(a);
    let it = lean_array_cptr(r).add(i);
    lean_dec(*it);
    *it = v;
    r
}

#[inline]
pub unsafe fn lean_array_fset(
    a: lean_obj_arg,
    i: b_lean_obj_arg,
    v: lean_obj_arg,
) -> *mut lean_object {
    lean_array_uset(a, lean_unbox(i), v)
}

#[inline]
pub unsafe fn lean_array_set(
    a: lean_obj_arg,
    i: b_lean_obj_arg,
    v: lean_obj_arg,
) -> *mut lean_object {
    if lean_is_scalar(i) {
        let idx = lean_unbox(i);
        if idx < lean_array_size(a) {
            return lean_array_uset(a, idx, v);
        }
    }
    lean_array_set_panic(a, v)
}

#[inline]
pub unsafe fn lean_array_pop(a: lean_obj_arg) -> *mut lean_object {
    let r = lean_ensure_exclusive_array(a);
    let sz = lean_array_size(a);
    if sz == 0 {
        return r;
    }
    let sz = sz - 1;
    let last = lean_array_cptr(r).add(sz);
    *(raw_field!(lean_to_array(r), lean_array_object, m_size) as *mut usize) = sz;
    lean_dec(*last);
    r
}

#[inline]
pub unsafe fn lean_array_uswap(a: lean_obj_arg, i: usize, j: usize) -> *mut lean_object {
    let r = lean_ensure_exclusive_array(a);
    let it = lean_array_cptr(r);
    std::ptr::swap(it.add(i), it.add(j));
    r
}

#[inline(always)]
pub unsafe fn lean_array_fswap(
    a: lean_obj_arg,
    i: b_lean_obj_arg,
    j: b_lean_obj_arg,
) -> *mut lean_object {
    lean_array_uswap(a, lean_unbox(i), lean_unbox(j))
}

#[inline(always)]
pub unsafe fn lean_array_swap(
    a: lean_obj_arg,
    i: b_lean_obj_arg,
    j: b_lean_obj_arg,
) -> *mut lean_object {
    if !lean_is_scalar(i) || !lean_is_scalar(j) {
        return a;
    }
    let i = lean_unbox(i);
    let j = lean_unbox(j);
    let sz = lean_array_size(a);
    if i >= sz || j >= sz {
        a
    } else {
        lean_array_uswap(a, i, j)
    }
}

#[link(name = "leanshared")]
extern "C" {
    pub fn lean_array_get_panic(def_val: lean_obj_arg) -> lean_obj_res;
    pub fn lean_copy_expand_array(a: lean_obj_arg, expand: bool) -> lean_obj_res;
    pub fn lean_array_set_panic(a: lean_obj_arg, v: lean_obj_arg) -> lean_obj_res;
    pub fn lean_array_push(a: lean_obj_arg, v: lean_obj_arg) -> *mut lean_object;
    pub fn lean_mk_array(n: lean_obj_arg, v: lean_obj_arg) -> *mut lean_object;

}
