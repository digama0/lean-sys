/*! IO Helper functions */

use crate::*;

#[inline(always)]
pub const fn lean_io_mk_world() -> lean_obj_res {
    lean_box(0)
}

#[inline(always)]
pub unsafe fn lean_io_result_is_ok(r: b_lean_obj_arg) -> bool {
    lean_ptr_tag(r) == 0
}

#[inline(always)]
pub unsafe fn lean_io_result_is_error(r: b_lean_obj_arg) -> bool {
    lean_ptr_tag(r) == 1
}

#[inline(always)]
pub unsafe fn lean_io_result_get_value(r: b_lean_obj_arg) -> b_lean_obj_res {
    debug_assert!(lean_io_result_is_ok(r));
    lean_ctor_get(r, 0)
}

#[inline(always)]
pub unsafe fn lean_io_result_get_error(r: b_lean_obj_arg) -> b_lean_obj_res {
    debug_assert!(lean_io_result_is_error(r));
    lean_ctor_get(r, 0)
}

#[inline(always)]
pub unsafe fn lean_io_result_mk_ok(a: lean_obj_arg) -> lean_obj_res {
    let r = lean_alloc_ctor(0, 2, 0);
    lean_ctor_set(r, 0, a);
    lean_ctor_set(r, 1, lean_box(0));
    r
}

#[inline(always)]
pub unsafe fn lean_io_result_mk_error(a: lean_obj_arg) -> lean_obj_res {
    let r = lean_alloc_ctor(1, 2, 0);
    lean_ctor_set(r, 0, a);
    lean_ctor_set(r, 1, lean_box(0));
    r
}

extern "C" {
    pub fn lean_decode_io_error(errnum: c_int, fname: b_lean_obj_arg) -> lean_obj_res;

    pub fn lean_io_result_show_error(r: b_lean_obj_arg);
    pub fn lean_io_mark_end_initialization();

    pub fn lean_mk_io_error_already_exists(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_already_exists_file(
        _: lean_obj_arg,
        _: u32,
        _: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_mk_io_error_eof(_: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_hardware_fault(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_illegal_operation(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_inappropriate_type(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_inappropriate_type_file(
        _: lean_obj_arg,
        _: u32,
        _: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_mk_io_error_interrupted(_: lean_obj_arg, _: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_invalid_argument(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_invalid_argument_file(
        _: lean_obj_arg,
        _: u32,
        _: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_mk_io_error_no_file_or_directory(
        _: lean_obj_arg,
        _: u32,
        _: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_mk_io_error_no_such_thing(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_no_such_thing_file(
        _: lean_obj_arg,
        _: u32,
        _: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_mk_io_error_other_error(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_permission_denied(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_permission_denied_file(
        _: lean_obj_arg,
        _: u32,
        _: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_mk_io_error_protocol_error(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_resource_busy(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_resource_exhausted(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_resource_exhausted_file(
        _: lean_obj_arg,
        _: u32,
        _: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_mk_io_error_resource_vanished(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_time_expired(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_unsatisfied_constraints(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_error_unsupported_operation(_: u32, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_mk_io_user_error(s: lean_obj_arg) -> lean_obj_res;
}
