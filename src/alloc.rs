use crate::lean_inc_heartbeat;
use crate::LEAN_OBJECT_SIZE_DELTA;
use core::alloc::GlobalAlloc;
use core::alloc::Layout;

/// A global allocator that uses Lean's allocator. This is useful when
/// writing FFI libraries for Lean, where people may want to disable rust's `std`.
/// ```ignore
/// #![no_std]
/// #[global_allocator]
/// static ALLOC : lean_sys::alloc::LeanAlloc = lean_sys::alloc::LeanAlloc;
/// ```
pub struct LeanAlloc;

extern "C" {
    fn aligned_alloc(align: usize, size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

unsafe impl GlobalAlloc for LeanAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alignment = layout.align().max(LEAN_OBJECT_SIZE_DELTA);
        let offset = layout.size().wrapping_neg() & (alignment - 1);
        let size = layout.size() + offset;

        #[cfg(feature = "small_allocator")]
        if alignment == LEAN_OBJECT_SIZE_DELTA && size <= crate::LEAN_MAX_SMALL_OBJECT_SIZE as usize
        {
            let slot_idx = crate::lean_get_slot_idx(size as _);
            return crate::lean_alloc_small(size as _, slot_idx).cast();
        }

        lean_inc_heartbeat();
        aligned_alloc(alignment, size)
    }

    #[allow(unused_variables)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        #[cfg(feature = "small_allocator")]
        {
            let alignment = layout.align().max(LEAN_OBJECT_SIZE_DELTA);
            let offset = layout.size().wrapping_neg() & (alignment - 1);
            let size = layout.size() + offset;
            if alignment == LEAN_OBJECT_SIZE_DELTA
                && size <= crate::LEAN_MAX_SMALL_OBJECT_SIZE as usize
            {
                return crate::lean_free_small(ptr.cast());
            }
        }

        free(ptr);
    }
}

#[cfg(test)]
mod test {
    use crate::lean_initialize_locked;
    extern crate std;

    #[test]
    fn various_sized_allocations() {
        unsafe {
            lean_initialize_locked();
        }
        use core::alloc::GlobalAlloc;
        let alloc = super::LeanAlloc;
        for size in [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 80, 123, 255, 1024, 2048, 4095, 4096, 4097,
            16000, 16384,
        ] {
            for align in [1, 2, 4, 8, 16, 32, 64, 128, 256, 512] {
                let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
                let ptr = unsafe { alloc.alloc_zeroed(layout) };
                assert_eq!(ptr as usize % align, 0);
                unsafe { alloc.dealloc(ptr, layout) };
            }
        }
    }
}
