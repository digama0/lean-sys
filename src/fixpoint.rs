/*! Fixpoint */
use crate::*;

#[link(name = "leanshared")]
extern "C" {
    pub fn lean_fixpoint(rec: lean_obj_arg, a1: lean_obj_arg) -> lean_obj_res;
    pub fn lean_fixpoint_2(rec: lean_obj_arg, a1: lean_obj_arg, a2: lean_obj_arg) -> lean_obj_res;
    pub fn lean_fixpoint_3(
        rec: lean_obj_arg,
        a1: lean_obj_arg,
        a2: lean_obj_arg,
        a3: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_fixpoint_4(
        rec: lean_obj_arg,
        a1: lean_obj_arg,
        a2: lean_obj_arg,
        a3: lean_obj_arg,
        a4: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_fixpoint_5(
        rec: lean_obj_arg,
        a1: lean_obj_arg,
        a2: lean_obj_arg,
        a3: lean_obj_arg,
        a4: lean_obj_arg,
        a5: lean_obj_arg,
    ) -> lean_obj_res;
    pub fn lean_fixpoint_6(
        rec: lean_obj_arg,
        a1: lean_obj_arg,
        a2: lean_obj_arg,
        a3: lean_obj_arg,
        a4: lean_obj_arg,
        a5: lean_obj_arg,
        a6: lean_obj_arg,
    ) -> lean_obj_res;
}
