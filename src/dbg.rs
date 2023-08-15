/*! Debugging helper functions */
use crate::*;

extern "C" {
    pub fn lean_dbg_trace(s: lean_obj_arg, f: lean_obj_arg) -> *mut lean_object;
    pub fn lean_dbg_sleep(ms: u32, f: lean_obj_arg) -> *mut lean_object;
    pub fn lean_dbg_trace_if_shared(s: lean_obj_arg, a: lean_obj_arg) -> *mut lean_object;
}
