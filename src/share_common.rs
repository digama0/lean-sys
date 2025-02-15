use crate::*;

extern "C" {
    pub fn lean_sharecommon_eq(o1: b_lean_obj_arg, o2: b_lean_obj_arg) -> u8;
    pub fn lean_sharecommon_hash(o1: b_lean_obj_arg) -> u64;
}
