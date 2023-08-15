/*!
Primitive operations
*/
use crate::*;

mod boxing;
mod float;
mod name;
mod st;
mod uint16;
mod uint32;
mod uint64;
mod uint8;
mod usize_;
pub use boxing::*;
pub use float::*;
pub use name::*;
pub use st::*;
pub use uint16::*;
pub use uint32::*;
pub use uint64::*;
pub use uint8::*;
pub use usize_::*;

#[inline(always)]
pub fn lean_bool_to_uint64(a: u8) -> u64 {
    a as u64
}

/** pointer address unsafe primitive  */
#[inline(always)]
pub fn lean_ptr_addr(a: b_lean_obj_arg) -> usize {
    a as usize
}
