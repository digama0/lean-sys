/*! ST Ref primitives */
use crate::*;

extern "C" {
    pub fn lean_st_mk_ref(_: lean_obj_arg, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_st_mk_ref_get(_: b_lean_obj_arg, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_st_mk_ref_set(_: b_lean_obj_arg, _: lean_obj_arg, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_st_mk_ref_reset(_: b_lean_obj_arg, _: lean_obj_arg) -> lean_obj_res;
    pub fn lean_st_mk_ref_swap(_: b_lean_obj_arg, _: lean_obj_arg, _: lean_obj_arg)
        -> lean_obj_res;
}
