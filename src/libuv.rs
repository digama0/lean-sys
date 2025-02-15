use crate::*;

extern "C" {
    pub fn lean_libuv_version(_: lean_obj_arg) -> lean_obj_res;
}
