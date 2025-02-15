/*! Arrays of objects (low level API) */
use crate::*;

#[inline]
pub unsafe fn lean_alloc_array(size: usize, capacity: usize) -> *mut lean_object {
    let o = lean_alloc_object(
        core::mem::size_of::<lean_array_object>() + core::mem::size_of::<*mut ()>() * capacity,
    );
    lean_set_st_header(o as *mut _, LeanArray as u32, 0);
    (raw_field!(o, lean_array_object, m_size) as *mut usize).write(size);
    (raw_field!(o, lean_array_object, m_capacity) as *mut usize).write(capacity);
    o as *mut _
}

#[inline(always)]
pub unsafe fn lean_array_size(o: b_lean_obj_arg) -> usize {
    *(raw_field!(o, lean_array_object, m_size))
}

#[inline(always)]
pub unsafe fn lean_array_capacity(o: b_lean_obj_arg) -> usize {
    *(raw_field!(o, lean_array_object, m_capacity))
}

#[inline]
pub unsafe fn lean_array_byte_size(o: b_lean_obj_arg) -> usize {
    core::mem::size_of::<lean_array_object>()
        + core::mem::size_of::<*mut ()>() * lean_array_capacity(o)
}

#[inline]
pub unsafe fn lean_array_data_byte_size(o: b_lean_obj_arg) -> usize {
    core::mem::size_of::<lean_array_object>() + core::mem::size_of::<*mut ()>() * lean_array_size(o)
}

#[inline(always)]
pub unsafe fn lean_array_cptr(o: b_lean_obj_arg) -> *mut *mut lean_object {
    raw_field!(o, lean_array_object, m_data) as *mut _
}

#[inline(always)]
pub unsafe fn lean_array_set_size(o: u_lean_obj_arg, sz: usize) {
    debug_assert!(lean_is_array(o));
    debug_assert!(lean_is_exclusive(o));
    debug_assert!(sz <= lean_array_capacity(o));
    (raw_field!(lean_to_array(o), lean_array_object, m_size) as *mut usize).write(sz)
}

#[inline(always)]
pub unsafe fn lean_array_get_core(o: b_lean_obj_arg, i: usize) -> b_lean_obj_res {
    debug_assert!(i < lean_array_size(o));
    *(raw_field!(lean_to_array(o), lean_array_object, m_data) as *mut *mut lean_object).add(i)
}

#[inline(always)]
pub unsafe fn lean_array_set_core(o: u_lean_obj_arg, i: usize, v: lean_obj_arg) {
    debug_assert!(!lean_has_rc(o) || lean_is_exclusive(o));
    debug_assert!(i < lean_array_size(o));
    (raw_field!(lean_to_array(o), lean_array_object, m_data) as *mut *mut lean_object)
        .add(i)
        .write(v)
}

extern "C" {
    pub fn lean_array_mk(l: lean_obj_arg) -> *mut lean_object;
    pub fn lean_array_to_list(a: lean_obj_arg) -> *mut lean_object;
}
