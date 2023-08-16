/*! Boxing primitives */
use crate::*;

#[inline]
pub unsafe fn lean_box_uint32(v: u32) -> lean_obj_res {
    if core::mem::size_of::<*const ()>() == 4 {
        // 32-bit implementation
        let r = lean_alloc_ctor(0, 0, core::mem::size_of::<u32>() as c_uint);
        lean_ctor_set_uint32(r, 0, v);
        r
    } else {
        // 64-bit implementation
        lean_box(v as usize)
    }
}

#[inline]
pub unsafe fn lean_unbox_uint32(o: b_lean_obj_arg) -> u32 {
    if core::mem::size_of::<*const ()>() == 4 {
        // 32-bit implementation
        lean_ctor_get_uint32(o, 0)
    } else {
        // 64-bit implementation
        lean_unbox(o) as u32
    }
}

#[inline]
pub unsafe fn lean_box_uint64(v: u64) -> lean_obj_res {
    let r = lean_alloc_ctor(0, 0, core::mem::size_of::<u64>() as c_uint);
    lean_ctor_set_uint64(r, 0, v);
    r
}

#[inline(always)]
pub unsafe fn lean_unbox_uint64(o: b_lean_obj_arg) -> u64 {
    lean_ctor_get_uint64(o, 0)
}

#[inline]
pub unsafe fn lean_box_usize(v: usize) -> lean_obj_res {
    let r = lean_alloc_ctor(0, 0, core::mem::size_of::<usize>() as c_uint);
    lean_ctor_set_usize(r, 0, v);
    r
}

#[inline(always)]
pub unsafe fn lean_unbox_usize(o: b_lean_obj_arg) -> usize {
    lean_ctor_get_usize(o, 0)
}

#[inline]
pub unsafe fn lean_box_float(v: f64) -> lean_obj_res {
    let r = lean_alloc_ctor(0, 0, core::mem::size_of::<f64>() as c_uint);
    lean_ctor_set_float(r, 0, v);
    r
}

#[inline(always)]
pub unsafe fn lean_unbox_float(o: b_lean_obj_arg) -> f64 {
    lean_ctor_get_float(o, 0)
}
