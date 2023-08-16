/*! Array of scalars */
use crate::*;

pub mod byte;
pub mod float;
pub use byte::*;
pub use float::*;

#[inline]
pub unsafe fn lean_alloc_sarray(elem_size: c_uint, size: usize, capacity: usize) -> lean_obj_res {
    let o = lean_alloc_object(
        core::mem::size_of::<lean_sarray_object>() + (elem_size as usize) * capacity,
    ) as *mut lean_sarray_object;
    lean_set_st_header(o as *mut _, LeanScalarArray as u32, elem_size);
    (raw_field!(o, lean_sarray_object, m_size) as *mut usize).write(size);
    (raw_field!(o, lean_sarray_object, m_capacity) as *mut usize).write(capacity);
    o as *mut _
}

#[inline(always)]
pub unsafe fn lean_sarray_elem_size(o: *const lean_object) -> c_uint {
    debug_assert!(lean_is_sarray(o));
    lean_ptr_other(o)
}

#[inline(always)]
pub unsafe fn lean_sarray_capacity(o: *const lean_object) -> usize {
    *raw_field!(lean_to_sarray(o as *mut _), lean_sarray_object, m_capacity)
}

#[inline]
pub unsafe fn lean_sarray_byte_size(o: *const lean_object) -> usize {
    core::mem::size_of::<lean_sarray_object>()
        + (lean_sarray_elem_size(o) as usize) * lean_sarray_capacity(o)
}

#[inline(always)]
pub unsafe fn lean_sarray_size(o: *const lean_object) -> usize {
    *raw_field!(lean_to_sarray(o as *mut _), lean_sarray_object, m_size)
}

#[inline(always)]
pub unsafe fn lean_sarray_set_size(o: u_lean_obj_arg, sz: usize) {
    debug_assert!(lean_is_exclusive(o));
    debug_assert!(sz <= lean_sarray_capacity(o));
    (raw_field!(lean_to_sarray(o), lean_sarray_object, m_size) as *mut usize).write(sz);
}

#[inline(always)]
pub unsafe fn lean_sarray_cptr(o: *mut lean_object) -> *mut u8 {
    raw_field!(lean_to_sarray(o), lean_sarray_object, m_data) as *mut _
}
