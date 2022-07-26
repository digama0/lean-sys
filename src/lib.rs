/*!
Low-level bindings for the Lean 4 interactive theorem prover.

Functions manually translated from `lean.h`
*/
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use memoffset::raw_field;
use static_assertions::const_assert;
use std::os::raw::*;
use std::sync::atomic::{AtomicI32, AtomicI64, Ordering};

pub mod array;
pub mod closure;
pub mod constructor;
pub mod dbg;
pub mod external;
pub mod fixpoint;
pub mod int;
pub mod io;
pub mod nat;
pub mod primitive;
pub mod sarray;
pub mod string;
pub mod task;
pub mod thunk;

pub use array::*;
pub use closure::*;
pub use constructor::*;
pub use dbg::*;
pub use external::*;
pub use fixpoint::*;
pub use int::*;
pub use io::*;
pub use nat::*;
pub use primitive::*;
pub use sarray::*;
pub use string::*;
pub use task::*;
pub use thunk::*;

pub const LEAN_SMALL_ALLOCATOR: bool = cfg!(feature = "small_allocator");
pub const LEAN_CLOSURE_MAX_ARGS: u32 = 16;
pub const LEAN_OBJECT_SIZE_DELTA: usize = 8;
pub const LEAN_MAX_SMALL_OBJECT_SIZE: u32 = 4096;
pub const LeanMaxCtorTag: u8 = 244;
pub const LeanClosure: u8 = 245;
pub const LeanArray: u8 = 246;
pub const LeanStructArray: u8 = 247;
pub const LeanScalarArray: u8 = 248;
pub const LeanString: u8 = 249;
pub const LeanMPZ: u8 = 250;
pub const LeanThunk: u8 = 251;
pub const LeanTask: u8 = 252;
pub const LeanRef: u8 = 253;
pub const LeanExternal: u8 = 254;
pub const LeanReserved: u8 = 255;
pub const LEAN_MAX_CTOR_FIELDS: u32 = 256;
pub const LEAN_MAX_CTOR_SCALARS_SIZE: u32 = 1024;

const_assert! {
    layout_compat::<c_int, i32>() ||
    layout_compat::<c_int, i64>()
}

#[inline(always)]
pub const fn layout_compat<A, B>() -> bool {
    std::mem::size_of::<A>() == std::mem::size_of::<B>()
        && std::mem::align_of::<A>() == std::mem::align_of::<B>()
}

#[inline(always)]
pub unsafe fn c_int_load(p: *const c_int, order: Ordering) -> c_int {
    match c_int::BITS {
        32 => (*(p as *const AtomicI32)).load(order) as c_int,
        64 => (*(p as *const AtomicI64)).load(order) as c_int,
        _ => unreachable!(),
    }
}

mod types;
pub use types::*;

#[inline]
pub fn lean_is_big_object_tag(tag: u8) -> bool {
    matches!(
        tag,
        LeanArray | LeanStructArray | LeanScalarArray | LeanString
    )
}

/// Whether this lean object is a scalar
///
/// # Examples
/// ```rust
/// # use lean_sys::*;
/// unsafe { lean_initialize_runtime_module(); }
/// let arr = unsafe { lean_mk_empty_array() };
/// assert!(lean_is_scalar(lean_box(5)));
/// assert!(!lean_is_scalar(arr));
/// ```
#[inline(always)]
pub fn lean_is_scalar(obj: *const lean_object) -> bool {
    (obj as usize & 1) == 1
}

/// Box a `usize` into a `lean_object`
///
/// # Examples
/// ```rust
/// # use lean_sys::*;
/// for i in [0, 5, 658948, 1 << (usize::BITS - 2)] {
///     assert_eq!(lean_unbox(lean_box(i)), i);
/// }
/// ```
#[inline(always)]
pub fn lean_box(n: usize) -> *mut lean_object {
    ((n << 1) | 1) as *mut lean_object
}

/// Unbox a `lean_object` into a `usize`
///
/// See [`lean_box`] for usage examples.
#[inline(always)]
pub fn lean_unbox(o: *const lean_object) -> usize {
    o as usize >> 1
}

#[inline]
pub fn lean_align(v: usize, a: usize) -> usize {
    (v / a).wrapping_mul(a) + a.wrapping_mul((v % a != 0) as usize)
}

#[inline]
pub fn lean_get_slot_idx(sz: c_uint) -> c_uint {
    debug_assert_ne!(sz, 0);
    debug_assert_eq!(
        lean_align(sz as usize, LEAN_OBJECT_SIZE_DELTA as usize),
        sz as usize
    );
    sz / ((LEAN_OBJECT_SIZE_DELTA as c_uint) - 1)
}

#[inline]
pub unsafe fn lean_alloc_small_object(sz: c_uint) -> *mut lean_object {
    if LEAN_SMALL_ALLOCATOR {
        let sz = lean_align(sz as usize, LEAN_OBJECT_SIZE_DELTA) as c_uint;
        let slot_idx = lean_get_slot_idx(sz);
        debug_assert!(sz <= LEAN_MAX_SMALL_OBJECT_SIZE);
        lean_alloc_small(sz, slot_idx) as *mut _
    } else {
        lean_inc_heartbeat();
        let mem = libc::malloc(std::mem::size_of::<usize>() + sz as usize) as *mut usize;
        if mem.is_null() {
            lean_internal_panic_out_of_memory()
        }
        *mem = sz as usize;
        mem.add(1) as *mut _
    }
}

#[inline]
pub unsafe fn lean_alloc_ctor_memory(sz: c_uint) -> *mut lean_object {
    if LEAN_SMALL_ALLOCATOR {
        let sz1 = lean_align(sz as usize, LEAN_OBJECT_SIZE_DELTA) as c_uint;
        let slot_idx = lean_get_slot_idx(sz1);
        debug_assert!(sz1 <= LEAN_MAX_SMALL_OBJECT_SIZE);
        let r = lean_alloc_small(sz1, slot_idx);
        if sz1 > sz {
            //TODO: "two structurally equal objects"?
            /* Initialize last word.
            In our runtime `lean_object_byte_size` is always
            a multiple of the machine word size for constructors.

            By setting the last word to 0, we make sure the sharing
            maximizer procedures at `maxsharing.cpp` and `compact.cpp` are
            not affected by uninitialized data at the (sz1 - sz) last bytes.
            Otherwise, we may mistakenly assume to structurally equal
            objects are not identical because of this uninitialized memory. */
            let end = (r as *mut u8).add(sz1 as usize) as *mut usize;
            *end.sub(1) = 0;
        }
        r as *mut lean_object
    } else {
        lean_alloc_small_object(sz)
    }
}

#[inline(always)]
pub unsafe fn lean_small_object_size(o: *mut lean_object) -> c_uint {
    if LEAN_SMALL_ALLOCATOR {
        lean_small_mem_size(o as *mut _)
    } else {
        *(o as *mut usize).sub(1) as c_uint
    }
}

#[inline(always)]
pub unsafe fn lean_free_small_object(o: *mut lean_object) {
    if LEAN_SMALL_ALLOCATOR {
        lean_free_small(o as *mut _)
    } else {
        libc::free((o as *mut usize).sub(1) as *mut _)
    }
}

#[inline(always)]
pub unsafe fn lean_ptr_tag(o: *const lean_object) -> u8 {
    // Read m_tag
    (*raw_field!(o, lean_object, _bitfield_1)).get(24usize, 8u8) as u8
}

#[inline(always)]
pub unsafe fn lean_ptr_other(o: *const lean_object) -> c_uint {
    // Read m_other
    (*raw_field!(o, lean_object, _bitfield_1)).get(16usize, 8u8) as c_uint
}

/// Whether an object is multi-threaded
#[inline(always)]
pub unsafe fn lean_is_mt(o: *const lean_object) -> bool {
    relaxed_rc_load(o) < 0
}

/// Whether an object is single-threaded
#[inline(always)]
pub unsafe fn lean_is_st(o: *const lean_object) -> bool {
    relaxed_rc_load(o) > 0
}

/// Perform a relaxed load of the reference counter of `o`, which must be a pointer into a valid [`lean_object`]
#[inline(always)]
pub unsafe fn relaxed_rc_load(o: *const lean_object) -> c_int {
    c_int_load(lean_get_rc_mt_addr(o as *mut _), Ordering::Relaxed)
}

/* We never update the reference counter of objects stored in compact regions and allocated at initialization time. */
#[inline(always)]
pub unsafe fn lean_is_persistent(o: *const lean_object) -> bool {
    relaxed_rc_load(o) == 0
}

/// Whether an object is reference counted
#[inline(always)]
pub unsafe fn lean_has_rc(o: *const lean_object) -> bool {
    relaxed_rc_load(o) != 0
}

#[inline(always)]
pub unsafe fn lean_get_rc_mt_addr(o: *mut lean_object) -> *mut c_int {
    raw_field!(o, lean_object, m_rc) as *mut c_int
}

#[inline]
pub unsafe fn lean_inc_ref(o: *mut lean_object) {
    if lean_is_st(o) {
        *(raw_field!(o, lean_object, m_rc) as *mut c_int) += 1
    } else {
        lean_inc_ref_cold(o)
    }
}

#[inline]
pub unsafe fn lean_inc_ref_n(o: *mut lean_object, n: usize) {
    if lean_is_st(o) {
        *(raw_field!(o, lean_object, m_rc) as *mut c_int) += n as c_int
    } else {
        lean_inc_ref_n_cold(o, n as c_uint)
    }
}

#[inline]
pub unsafe fn lean_dec_ref(o: *mut lean_object) {
    if relaxed_rc_load(o) > 1 {
        *(raw_field!(o, lean_object, m_rc) as *mut c_int) -= 1
    } else {
        lean_dec_ref_cold(o)
    }
}

#[inline]
pub fn lean_inc(o: *mut lean_object) {
    if !lean_is_scalar(o) {
        lean_inc(o)
    }
}

#[inline]
pub fn lean_inc_n(o: *mut lean_object, n: usize) {
    if !lean_is_scalar(o) {
        lean_inc_n(o, n)
    }
}

#[inline]
pub fn lean_dec(o: *mut lean_object) {
    if !lean_is_scalar(o) {
        lean_dec(o)
    }
}

#[inline(always)]
pub unsafe fn lean_is_ctor(o: *const lean_object) -> bool {
    lean_ptr_tag(o) <= LeanMaxCtorTag
}

#[inline(always)]
pub unsafe fn lean_is_closure(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanClosure
}

#[inline(always)]
pub unsafe fn lean_is_array(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanArray
}

#[inline(always)]
pub unsafe fn lean_is_sarray(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanScalarArray
}

#[inline(always)]
pub unsafe fn lean_is_string(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanString
}

#[inline(always)]
pub unsafe fn lean_is_mpz(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanMPZ
}

#[inline(always)]
pub unsafe fn lean_is_thunk(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanThunk
}

#[inline(always)]
pub unsafe fn lean_is_task(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanTask
}

#[inline(always)]
pub unsafe fn lean_is_external(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanExternal
}

#[inline(always)]
pub unsafe fn lean_is_ref(o: *const lean_object) -> bool {
    lean_ptr_tag(o) == LeanRef
}

#[inline(always)]
pub unsafe fn lean_obj_tag(o: *const lean_object) -> c_uint {
    if lean_is_scalar(o) {
        lean_unbox(o) as c_uint
    } else {
        lean_ptr_tag(o) as c_uint
    }
}

#[inline(always)]
pub unsafe fn lean_to_ctor(o: *mut lean_object) -> *mut lean_ctor_object {
    debug_assert!(lean_is_ctor(o));
    o as *mut lean_ctor_object
}

#[inline(always)]
pub unsafe fn lean_to_closure(o: *mut lean_object) -> *mut lean_closure_object {
    debug_assert!(lean_is_closure(o));
    o as *mut lean_closure_object
}

#[inline(always)]
pub unsafe fn lean_to_array(o: *mut lean_object) -> *mut lean_array_object {
    debug_assert!(lean_is_array(o));
    o as *mut lean_array_object
}

#[inline(always)]
pub unsafe fn lean_to_sarray(o: *mut lean_object) -> *mut lean_sarray_object {
    debug_assert!(lean_is_sarray(o));
    o as *mut lean_sarray_object
}

#[inline(always)]
pub unsafe fn lean_to_string(o: *mut lean_object) -> *mut lean_string_object {
    debug_assert!(lean_is_string(o));
    o as *mut lean_string_object
}

#[inline(always)]
pub unsafe fn lean_to_thunk(o: *mut lean_object) -> *mut lean_thunk_object {
    debug_assert!(lean_is_thunk(o));
    o as *mut lean_thunk_object
}

#[inline(always)]
pub unsafe fn lean_to_task(o: *mut lean_object) -> *mut lean_task_object {
    debug_assert!(lean_is_task(o));
    o as *mut lean_task_object
}

#[inline(always)]
pub unsafe fn lean_to_ref(o: *mut lean_object) -> *mut lean_ref_object {
    debug_assert!(lean_is_ref(o));
    o as *mut lean_ref_object
}

#[inline(always)]
pub unsafe fn lean_to_external(o: *mut lean_object) -> *mut lean_external_object {
    debug_assert!(lean_is_external(o));
    o as *mut lean_external_object
}

#[inline]
pub unsafe fn lean_is_exclusive(o: *mut lean_object) -> bool {
    //TODO: likely... or, why not just relaxed_rc_load(o) == 1?
    if lean_is_st(o) {
        *raw_field!(o, lean_object, m_rc) == 1
    } else {
        false
    }
}

#[inline]
pub unsafe fn lean_is_shared(o: *mut lean_object) -> bool {
    //TODO: likely... or, why not just relaxed_rc_load(o) > 1?
    if lean_is_st(o) {
        *raw_field!(o, lean_object, m_rc) > 1
    } else {
        false
    }
}

#[inline]
pub unsafe fn lean_set_st_header(o: *mut lean_object, tag: c_uint, other: c_uint) {
    *lean_get_rc_mt_addr(o) = 1;
    let bitfield = lean_object::new_bitfield_1(0, other, tag);
    (raw_field!(o, lean_object, _bitfield_1) as *mut Bitfield<[u8; 4]>).write(bitfield);
}

/** Remark: we don't need a reference counter for objects that are not stored in the heap.
Thus, we use the area to store the object size for small objects. */
#[inline]
pub unsafe fn lean_set_non_heap_header(o: *mut lean_object, sz: usize, tag: c_uint, other: c_uint) {
    debug_assert!(sz > 0);
    debug_assert!(sz < (1 << 16));
    debug_assert!(sz == 1 || !lean_is_big_object_tag(tag as u8));
    *lean_get_rc_mt_addr(o) = 0;
    let bitfield = lean_object::new_bitfield_1(sz as u32, other, tag);
    (raw_field!(o, lean_object, _bitfield_1) as *mut Bitfield<[u8; 4]>).write(bitfield);
}

/** `lean_set_non_heap_header` for (potentially) big objects such as arrays and strings. */
#[inline(always)]
pub unsafe fn lean_set_non_heap_header_for_big(o: *mut lean_object, tag: c_uint, other: c_uint) {
    lean_set_non_heap_header(o, 1, tag, other)
}

#[link(name = "leanshared")]
extern "C" {
    pub fn lean_initialize_runtime_module();
    pub fn lean_initialize();

    pub fn lean_set_exit_on_panic(flag: bool);
    /// Enable/disable panic messages
    pub fn lean_set_panic_messages(flag: bool);
    pub fn lean_panic_fn(default_val: *mut lean_object, msg: *mut lean_object);
    pub fn lean_internal_panic(msg: *const c_char) -> !;
    pub fn lean_internal_panic_out_of_memory() -> !;
    pub fn lean_internal_panic_unreachable() -> !;
    pub fn lean_internal_panic_rc_overflow() -> !;
    pub fn lean_alloc_small(sz: c_uint, slot_idx: c_uint) -> *mut c_void;
    pub fn lean_free_small(p: *mut c_void);
    pub fn lean_small_mem_size(p: *mut c_void) -> c_uint;
    pub fn lean_inc_heartbeat();
    pub fn lean_alloc_object(sz: usize) -> *mut lean_object;
    pub fn lean_free_object(o: *mut lean_object);
    /** The object size may be slightly bigger for constructor objects.
    The runtime does not track the size of the scalar size area.
    All constructor objects are "small", and allocated into pages.
    We retrieve their size by accessing the page header. The size of
    small objects is a multiple of `LEAN_OBJECT_SIZE_DELTA` */
    pub fn lean_object_byte_size(o: *mut lean_object) -> usize;
    #[cold]
    pub fn lean_inc_ref_cold(o: *mut lean_object);
    #[cold]
    pub fn lean_inc_ref_n_cold(o: *mut lean_object, n: c_uint);
    #[cold]
    pub fn lean_dec_ref_cold(o: *mut lean_object);
    /** Just free memory */
    pub fn lean_dealloc(o: *mut lean_object);
    pub fn lean_mark_mt(o: *mut lean_object);
    pub fn lean_mark_persistent(o: *mut lean_object);
}

#[cfg(test)]
mod test {}
