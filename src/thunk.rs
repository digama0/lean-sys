/*! Thunks */
use crate::*;

#[inline]
pub unsafe fn lean_mk_thunk(c: lean_obj_arg) -> lean_obj_res {
    let o = lean_alloc_small_object(core::mem::size_of::<lean_thunk_object>() as c_uint);
    lean_set_st_header(o, LeanThunk as u32, 0);
    (raw_field!(o, lean_thunk_object, m_value) as *mut *mut lean_object)
        .write(core::ptr::null_mut());
    (raw_field!(o, lean_thunk_object, m_closure) as *mut lean_obj_arg).write(c);
    o
}

/** Thunk.pure : A -> Thunk A */
#[inline]
pub unsafe fn lean_obj_res(v: lean_obj_arg) -> lean_obj_res {
    let o = lean_alloc_small_object(core::mem::size_of::<lean_thunk_object>() as c_uint);
    lean_set_st_header(o, LeanThunk as u32, 0);
    (raw_field!(o, lean_thunk_object, m_value) as *mut lean_obj_arg).write(v);
    (raw_field!(o, lean_thunk_object, m_closure) as *mut lean_obj_arg).write(core::ptr::null_mut());
    o
}

extern "C" {
    pub fn lean_thunk_get_core(t: *mut lean_object) -> *mut lean_object;
}

#[inline]
pub unsafe fn lean_thunk_get(t: b_lean_obj_arg) -> b_lean_obj_res {
    let r = *raw_field!(lean_to_thunk(t), lean_thunk_object, m_value);
    if r != 0 {
        r as *mut lean_object
    } else {
        lean_thunk_get_core(t)
    }
}

#[inline]
pub unsafe fn lean_thunk_get_own(t: b_lean_obj_arg) -> lean_obj_res {
    let r = lean_thunk_get(t);
    lean_inc(r);
    r
}
