/*! Automatically generated incomplete array code from bindgen */

use core::marker::PhantomData;

#[repr(C)]
#[derive(Default)]
pub struct IncompleteArrayField<T>(PhantomData<T>, [T; 0]);
impl<T> IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        IncompleteArrayField(PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> core::fmt::Debug for IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.write_str("IncompleteArrayField")
    }
}
