/*! Constructor objects */
use crate::*;

#[inline(always)]
pub unsafe fn lean_ctor_num_objs(o: *mut lean_object) -> c_uint {
    debug_assert!(lean_is_ctor(o));
    lean_ptr_other(o)
}

#[inline(always)]
pub unsafe fn lean_ctor_obj_cptr(o: *mut lean_object) -> *mut *mut lean_object {
    debug_assert!(lean_is_ctor(o));
    raw_field!(lean_to_ctor(o), lean_ctor_object, m_objs) as *mut _
}

#[inline]
pub unsafe fn lean_ctor_scalar_cptr(o: *mut lean_object) -> *mut u8 {
    debug_assert!(lean_is_ctor(o));
    lean_ctor_obj_cptr(o).add(lean_ctor_num_objs(o) as usize) as *mut u8
}

#[inline]
pub unsafe fn lean_alloc_ctor(
    tag: c_uint,
    num_objs: c_uint,
    scalar_sz: c_uint,
) -> *mut lean_object {
    debug_assert!(
        tag <= LeanMaxCtorTag as c_uint
            && num_objs < LEAN_MAX_CTOR_FIELDS
            && scalar_sz < LEAN_MAX_CTOR_SCALARS_SIZE
    );
    let o = lean_alloc_ctor_memory(
        std::mem::size_of::<lean_ctor_object>() as c_uint
            + (std::mem::size_of::<*const ()>() as c_uint) * num_objs
            + scalar_sz,
    );
    lean_set_st_header(o, tag, num_objs);
    o
}

#[inline(always)]
pub unsafe fn lean_ctor_get(o: b_lean_obj_arg, i: c_uint) -> b_lean_obj_res {
    debug_assert!(i < lean_ctor_num_objs(o));
    *lean_ctor_obj_cptr(o).add(i as usize)
}

#[inline(always)]
pub unsafe fn lean_ctor_set(o: b_lean_obj_arg, i: c_uint, v: lean_obj_arg) {
    debug_assert!(i < lean_ctor_num_objs(o));
    *lean_ctor_obj_cptr(o).add(i as usize) = v;
}

#[inline(always)]
pub unsafe fn lean_ctor_set_tag(o: b_lean_obj_arg, new_tag: u8) {
    debug_assert!(new_tag <= LeanMaxCtorTag);
    (raw_field!(o, lean_object, m_tag) as *mut u8).write(new_tag as u8)
}

#[inline]
pub unsafe fn lean_ctor_release(o: b_lean_obj_arg, i: c_uint) {
    debug_assert!(i < lean_ctor_num_objs(o));
    let objs = lean_ctor_obj_cptr(o);
    lean_dec(*objs.add(1));
    *objs.add(i as usize) = lean_box(0)
}

#[inline(always)]
pub unsafe fn lean_ctor_get_usize(o: b_lean_obj_arg, i: c_uint) -> usize {
    debug_assert!(i >= lean_ctor_num_objs(o));
    *(lean_ctor_obj_cptr(o).add(i as usize) as *const usize)
}

#[inline(always)]
pub unsafe fn lean_ctor_get_uint8(o: b_lean_obj_arg, offset: c_uint) -> u8 {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    *(lean_ctor_obj_cptr(o) as *const u8).add(offset as usize)
}

#[inline(always)]
pub unsafe fn lean_ctor_get_uint16(o: b_lean_obj_arg, offset: c_uint) -> u16 {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    ((lean_ctor_obj_cptr(o) as *const u8).add(offset as usize) as *const u16).read_unaligned()
}

#[inline(always)]
pub unsafe fn lean_ctor_get_uint32(o: b_lean_obj_arg, offset: c_uint) -> u32 {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    ((lean_ctor_obj_cptr(o) as *const u8).add(offset as usize) as *const u32).read_unaligned()
}

#[inline(always)]
pub unsafe fn lean_ctor_get_uint64(o: b_lean_obj_arg, offset: c_uint) -> u64 {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    ((lean_ctor_obj_cptr(o) as *const u8).add(offset as usize) as *const u64).read_unaligned()
}

#[inline(always)]
pub unsafe fn lean_ctor_get_float(o: b_lean_obj_arg, offset: c_uint) -> f64 {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    //TODO: is read_unaligned necessary here?
    ((lean_ctor_obj_cptr(o) as *const u8).add(offset as usize) as *const f64).read_unaligned()
}

#[inline(always)]
pub unsafe fn lean_ctor_set_usize(o: b_lean_obj_arg, i: c_uint, v: usize) {
    debug_assert!(i >= lean_ctor_num_objs(o));
    (lean_ctor_obj_cptr(o).add(i as usize) as *mut usize).write(v)
}

#[inline(always)]
pub unsafe fn lean_ctor_set_uint8(o: b_lean_obj_arg, offset: c_uint, v: u8) {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    (lean_ctor_obj_cptr(o) as *mut u8)
        .add(offset as usize)
        .write(v)
}

#[inline(always)]
pub unsafe fn lean_ctor_set_uint16(o: b_lean_obj_arg, offset: c_uint, v: u16) {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    ((lean_ctor_obj_cptr(o) as *mut u8).add(offset as usize) as *mut u16).write_unaligned(v)
}

#[inline(always)]
pub unsafe fn lean_ctor_set_uint32(o: b_lean_obj_arg, offset: c_uint, v: u32) {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    ((lean_ctor_obj_cptr(o) as *mut u8).add(offset as usize) as *mut u32).write_unaligned(v)
}

#[inline(always)]
pub unsafe fn lean_ctor_set_uint64(o: b_lean_obj_arg, offset: c_uint, v: u64) {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    ((lean_ctor_obj_cptr(o) as *mut u8).add(offset as usize) as *mut u64).write_unaligned(v)
}

#[inline(always)]
pub unsafe fn lean_ctor_set_float(o: b_lean_obj_arg, offset: c_uint, v: f64) {
    debug_assert!(offset >= lean_ctor_num_objs(o) * std::mem::size_of::<*const ()>() as c_uint);
    ((lean_ctor_obj_cptr(o) as *mut u8).add(offset as usize) as *mut f64).write_unaligned(v)
}
