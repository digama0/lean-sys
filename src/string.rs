/*! Strings */
use crate::*;

#[inline(always)]
pub unsafe fn lean_alloc_string(size: usize, capacity: usize, len: usize) -> lean_obj_res {
    let o = lean_alloc_object(core::mem::size_of::<lean_string_object>() + capacity)
        as *mut lean_string_object;
    lean_set_st_header(o as *mut _, LeanString as u32, 0);
    (raw_field!(o, lean_string_object, m_size) as *mut usize).write(size);
    (raw_field!(o, lean_string_object, m_capacity) as *mut usize).write(capacity);
    (raw_field!(o, lean_string_object, m_length) as *mut usize).write(len);
    o as *mut _
}

#[inline(always)]
pub unsafe fn lean_string_capacity(o: *mut lean_object) -> usize {
    *raw_field!(lean_to_string(o), lean_string_object, m_capacity)
}

#[inline(always)]
pub unsafe fn lean_string_byte_size(o: *mut lean_object) -> usize {
    core::mem::size_of::<lean_string_object>() + lean_string_capacity(o)
}

/** instance : inhabited char := ⟨'A'⟩ */
#[inline(always)]
pub fn lean_char_default_value() -> c_char {
    'A' as c_char
}

#[inline(always)]
pub unsafe fn lean_string_cstr(o: b_lean_obj_arg) -> *const u8 {
    debug_assert!(lean_is_string(o));
    raw_field!(lean_to_string(o), lean_string_object, m_data) as *const _
}

#[inline(always)]
pub unsafe fn lean_string_size(o: b_lean_obj_arg) -> usize {
    *raw_field!(lean_to_string(o), lean_string_object, m_size)
}

#[inline(always)]
pub unsafe fn lean_string_len(o: b_lean_obj_arg) -> usize {
    *raw_field!(lean_to_string(o), lean_string_object, m_length)
}

#[inline(always)]
pub unsafe fn lean_string_length(o: b_lean_obj_arg) -> lean_obj_res {
    lean_box(lean_string_len(o))
}

#[inline]
pub unsafe fn lean_string_utf8_at_end(s: b_lean_obj_arg, i: b_lean_obj_arg) -> u8 {
    (!lean_is_scalar(i) || lean_unbox(i) >= lean_string_size(s) - 1) as u8
}

#[inline(always)]
pub unsafe fn lean_string_utf8_byte_size(s: b_lean_obj_arg) -> lean_obj_res {
    lean_box(lean_string_size(s) - 1)
}

#[inline]
pub unsafe fn lean_string_eq(s1: b_lean_obj_arg, s2: b_lean_obj_arg) -> bool {
    s1 == s2 || (lean_string_size(s1) == lean_string_size(s2) && lean_string_eq_cold(s1, s2))
}

#[inline(always)]
pub unsafe fn lean_string_ne(s1: b_lean_obj_arg, s2: b_lean_obj_arg) -> bool {
    !lean_string_eq(s1, s2)
}

#[inline(always)]
pub unsafe fn lean_string_dec_eq(s1: b_lean_obj_arg, s2: b_lean_obj_arg) -> u8 {
    lean_string_eq(s1, s2) as u8
}

#[inline(always)]
pub unsafe fn lean_string_dec_lt(s1: b_lean_obj_arg, s2: b_lean_obj_arg) -> u8 {
    lean_string_lt(s1, s2) as u8
}

#[inline(always)]
pub unsafe fn lean_string_utf8_get_fast(s: b_lean_obj_arg, i: b_lean_obj_arg) -> u32 {
    let st = lean_string_cstr(s);
    let idx = lean_unbox(i);
    let c = *st.add(idx);
    if c & 0x80 == 0 {
        c as u32
    } else {
        lean_string_utf8_get_fast_cold(st, idx, lean_string_size(s), c)
    }
}

#[inline(always)]
pub unsafe fn lean_string_get_byte_fast(s: b_lean_obj_arg, i: b_lean_obj_arg) -> u8 {
    let str = lean_string_cstr(s);
    let idx = lean_unbox(i);
    *str.add(idx)
}

#[inline(always)]
pub unsafe fn lean_string_utf8_next_fast(s: b_lean_obj_arg, i: b_lean_obj_arg) -> lean_obj_res {
    let s = lean_string_cstr(s);
    let idx = lean_unbox(i);
    let c = *s.add(idx);
    if c & 0x80 == 0 {
        lean_box(idx + 1)
    } else {
        lean_string_utf8_next_fast_cold(idx, c)
    }
}

extern "C" {
    pub fn lean_utf8_strlen(str: *const u8) -> usize;
    pub fn lean_utf8_n_strlen(str: *const u8, n: usize) -> usize;
    pub fn lean_mk_string_from_bytes(s: *const u8, sz: usize) -> lean_obj_res;
    pub fn lean_mk_string(s: *const u8) -> lean_obj_res;
    pub fn lean_string_push(s: lean_obj_arg, c: u32) -> lean_obj_res;
    pub fn lean_string_append(s1: lean_obj_arg, s2: lean_obj_arg) -> lean_obj_res;
    pub fn lean_string_mk(cs: lean_obj_arg) -> lean_obj_res;
    pub fn lean_string_data(s: lean_obj_arg) -> lean_obj_res;
    pub fn lean_string_utf8_get(s: b_lean_obj_arg, i: b_lean_obj_arg) -> u32;
    pub fn lean_string_utf8_get_fast_cold(s: *const u8, i: usize, size: usize, c: u8) -> u32;
    pub fn lean_string_utf8_next(s: b_lean_obj_arg, i: b_lean_obj_arg) -> lean_obj_res;
    pub fn lean_string_utf8_next_fast_cold(i: usize, c: u8) -> lean_obj_res;
    pub fn lean_string_utf8_prev(s: b_lean_obj_arg, i: b_lean_obj_arg) -> lean_obj_res;
    pub fn lean_string_utf8_set(s: lean_obj_arg, i: b_lean_obj_arg, c: u32) -> lean_obj_res;
    pub fn lean_string_utf8_extract(
        s: b_lean_obj_arg,
        b: b_lean_obj_arg,
        e: b_lean_obj_arg,
    ) -> lean_obj_res;
    #[cold]
    pub fn lean_string_eq_cold(s1: b_lean_obj_arg, s2: b_lean_obj_arg) -> bool;
    pub fn lean_string_lt(s1: b_lean_obj_arg, s2: b_lean_obj_arg) -> bool;
    pub fn lean_string_hash(s: b_lean_obj_arg) -> u64;
}
