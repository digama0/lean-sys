/*! Closures */
use crate::*;

#[inline(always)]
pub unsafe fn lean_closure_fun(o: *mut lean_object) -> *mut c_void {
    *raw_field!(lean_to_closure(o), lean_closure_object, m_fun)
}

#[inline(always)]
pub unsafe fn lean_closure_arity(o: *mut lean_object) -> c_uint {
    *raw_field!(lean_to_closure(o), lean_closure_object, m_arity) as u32
}

#[inline(always)]
pub unsafe fn lean_closure_num_fixed(o: *mut lean_object) -> c_uint {
    *raw_field!(lean_to_closure(o), lean_closure_object, m_num_fixed) as u32
}

#[inline(always)]
pub unsafe fn lean_closure_arg_cptr(o: *mut lean_object) -> *mut *mut lean_object {
    raw_field!(lean_to_closure(o), lean_closure_object, m_objs) as *mut _
}

#[inline]
pub unsafe fn lean_alloc_closure(
    fun: *mut c_void,
    arity: c_uint,
    num_fixed: c_uint,
) -> lean_obj_res {
    debug_assert!(arity > 0);
    debug_assert!(num_fixed < arity);
    let o = lean_alloc_small_object(
        std::mem::size_of::<lean_closure_object>() as c_uint
            + std::mem::size_of::<*const ()>() as c_uint * num_fixed,
    );
    lean_set_st_header(o as *mut lean_object, LeanClosure as u32, 0);
    (raw_field!(o, lean_closure_object, m_fun) as *mut *mut c_void).write(fun);
    (raw_field!(o, lean_closure_object, m_arity) as *mut u16).write(arity as u16);
    (raw_field!(o, lean_closure_object, m_num_fixed) as *mut u16).write(num_fixed as u16);
    o
}

#[inline(always)]
pub unsafe fn lean_closure_get(o: b_lean_obj_arg, i: c_uint) -> b_lean_obj_res {
    debug_assert!(i <= lean_closure_num_fixed(o));
    *(raw_field!(lean_to_closure(o), lean_closure_object, m_objs) as *mut *mut lean_object)
        .add(i as usize)
}

#[inline(always)]
pub unsafe fn lean_closure_set(o: u_lean_obj_arg, i: c_uint, a: lean_obj_arg) {
    debug_assert!(i <= lean_closure_num_fixed(o));
    // Why is the reference count not decremented here?
    (raw_field!(lean_to_closure(o), lean_closure_object, m_objs) as *mut *mut lean_object)
        .add(i as usize)
        .write(a)
}

#[link(name = "leanshared")]
extern "C" {
    pub fn lean_apply_1(f: *mut lean_object, a1: *mut lean_object) -> *mut lean_object;
    pub fn lean_apply_2(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_3(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_4(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_5(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_6(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_7(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_8(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_9(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
        a9: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_10(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
        a9: *mut lean_object,
        a10: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_11(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
        a9: *mut lean_object,
        a10: *mut lean_object,
        a11: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_12(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
        a9: *mut lean_object,
        a10: *mut lean_object,
        a11: *mut lean_object,
        a12: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_13(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
        a9: *mut lean_object,
        a10: *mut lean_object,
        a11: *mut lean_object,
        a12: *mut lean_object,
        a13: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_14(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
        a9: *mut lean_object,
        a10: *mut lean_object,
        a11: *mut lean_object,
        a12: *mut lean_object,
        a13: *mut lean_object,
        a14: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_15(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
        a9: *mut lean_object,
        a10: *mut lean_object,
        a11: *mut lean_object,
        a12: *mut lean_object,
        a13: *mut lean_object,
        a14: *mut lean_object,
        a15: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_16(
        f: *mut lean_object,
        a1: *mut lean_object,
        a2: *mut lean_object,
        a3: *mut lean_object,
        a4: *mut lean_object,
        a5: *mut lean_object,
        a6: *mut lean_object,
        a7: *mut lean_object,
        a8: *mut lean_object,
        a9: *mut lean_object,
        a10: *mut lean_object,
        a11: *mut lean_object,
        a12: *mut lean_object,
        a13: *mut lean_object,
        a14: *mut lean_object,
        a15: *mut lean_object,
        a16: *mut lean_object,
    ) -> *mut lean_object;
    pub fn lean_apply_n(f: *mut lean_object, args: *mut *mut lean_object) -> *mut lean_object;
    /** Pre: n > 16 */
    pub fn lean_apply_m(f: *mut lean_object, args: *mut *mut lean_object) -> *mut lean_object;
}
