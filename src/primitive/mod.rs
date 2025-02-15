/*!
Primitive operations
*/
use crate::*;

mod boxing;
mod float;
mod float32;
mod int16;
mod int32;
mod int64;
mod int8;
mod isize_;
mod name;
mod st;
mod uint16;
mod uint32;
mod uint64;
mod uint8;
mod usize_;
pub use boxing::*;
pub use float::*;
pub use float32::*;
pub use int16::*;
pub use int32::*;
pub use int64::*;
pub use int8::*;
pub use isize_::*;
pub use name::*;
pub use st::*;
pub use uint16::*;
pub use uint32::*;
pub use uint64::*;
pub use uint8::*;
pub use usize_::*;

#[inline(always)]
pub fn lean_bool_to_uint8(a: u8) -> u8 {
    a
}
#[inline(always)]
pub fn lean_bool_to_uint16(a: u8) -> u16 {
    a as u16
}
#[inline(always)]
pub fn lean_bool_to_uint32(a: u8) -> u32 {
    a as u32
}
#[inline(always)]
pub fn lean_bool_to_uint64(a: u8) -> u64 {
    a as u64
}
#[inline(always)]
pub fn lean_bool_to_usize(a: u8) -> usize {
    a as usize
}
#[inline(always)]
pub fn lean_bool_to_int8(a: u8) -> u8 {
    a as i8 as u8
}
#[inline(always)]
pub fn lean_bool_to_int16(a: u8) -> u16 {
    a as i16 as u16
}
#[inline(always)]
pub fn lean_bool_to_int32(a: u8) -> u32 {
    a as i32 as u32
}
#[inline(always)]
pub fn lean_bool_to_int64(a: u8) -> u64 {
    a as i64 as u64
}
#[inline(always)]
pub fn lean_bool_to_isize(a: u8) -> usize {
    a as isize as usize
}

/** pointer address unsafe primitive  */
#[inline(always)]
pub fn lean_ptr_addr(a: b_lean_obj_arg) -> usize {
    a as usize
}
