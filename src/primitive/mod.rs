/*!
Primitive operations
*/
use crate::*;

pub mod boxing;
pub mod float;
pub mod name;
pub mod st;
pub mod uint16;
pub mod uint32;
pub mod uint64;
pub mod uint8;
pub mod usize_;
pub use boxing::*;
pub use float::*;
pub use name::*;
pub use std::*;
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
