/*! External objects */
use crate::*;

#[inline]
pub unsafe fn lean_alloc_external(
    cls: *mut lean_external_class,
    data: *mut c_void,
) -> *mut lean_object {
    let o = lean_alloc_small_object(core::mem::size_of::<lean_external_object>() as c_uint);
    lean_set_st_header(o, LeanExternal as u32, 0);
    (raw_field!(o, lean_external_object, m_class) as *mut *mut lean_external_class).write(cls);
    (raw_field!(o, lean_external_object, m_data) as *mut *mut c_void).write(data);
    o
}

#[inline(always)]
pub unsafe fn lean_get_external_class(o: *mut lean_object) -> *mut lean_external_class {
    *raw_field!(lean_to_external(o), lean_external_object, m_class)
}

#[inline(always)]
pub unsafe fn lean_get_external_data(o: *mut lean_object) -> *mut c_void {
    *raw_field!(lean_to_external(o), lean_external_object, m_data)
}

extern "C" {
    pub fn lean_register_external_class(
        _: lean_external_finalize_proc,
        _: lean_external_foreach_proc,
    ) -> *mut lean_external_class;
}
