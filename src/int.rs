/*! Integers */
use crate::*;

pub const LEAN_MAX_SMALL_INT: c_int = if std::mem::size_of::<*const ()>() == 8 {
    c_int::MAX
} else {
    1 << 30
};
pub const LEAN_MIN_SMALL_INT: c_int = if std::mem::size_of::<*const ()>() == 8 {
    c_int::MIN
} else {
    -(1 << 30)
};

#[inline]
pub unsafe fn lean_int_to_int(n: c_int) -> lean_obj_res {
    #[allow(clippy::absurd_extreme_comparisons, clippy::manual_range_contains)]
    if n <= LEAN_MAX_SMALL_INT && n >= LEAN_MIN_SMALL_INT {
        lean_box(n as usize)
    } else {
        lean_big_int_to_int(n)
    }
}

#[inline]
pub unsafe fn lean_int64_to_int(n: i64) -> lean_obj_res {
    if n <= LEAN_MAX_SMALL_INT as i64 && n >= LEAN_MIN_SMALL_INT as i64 {
        //TODO: likely
        lean_box(n as usize)
    } else {
        lean_big_int64_to_int(n)
    }
}

#[inline(always)]
pub unsafe fn lean_scalar_to_int64(a: b_lean_obj_arg) -> i64 {
    debug_assert!(lean_is_scalar(a));
    lean_unbox(a) as i64
}

#[inline(always)]
pub unsafe fn lean_scalar_to_int(a: b_lean_obj_arg) -> c_int {
    debug_assert!(lean_is_scalar(a));
    lean_unbox(a) as c_int
}

#[inline]
pub unsafe fn lean_nat_to_int(a: lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a) {
        let v = lean_unbox(a);
        if v <= LEAN_MAX_SMALL_INT as usize {
            a
        } else {
            lean_big_size_t_to_int(v)
        }
    } else {
        a
    }
}

#[inline]
pub unsafe fn lean_int_neg(a: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a) {
        //TODO: likely
        lean_int64_to_int(-lean_scalar_to_int64(a))
    } else {
        lean_int_big_neg(a)
    }
}

#[inline]
pub unsafe fn lean_int_neg_succ_of_nat(a: lean_obj_arg) -> lean_obj_res {
    let s = lean_nat_succ(a);
    lean_dec(a);
    let i = lean_nat_to_int(s); /* Recall that `lean_nat_to_int` consumes the argument */
    let r = lean_int_neg(i);
    lean_dec(i);
    r
}

#[inline]
pub unsafe fn lean_int_add(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        lean_int64_to_int(lean_scalar_to_int64(a1) + lean_scalar_to_int64(a2))
    } else {
        lean_int_big_add(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_int_sub(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        lean_int64_to_int(lean_scalar_to_int64(a1) - lean_scalar_to_int64(a2))
    } else {
        lean_int_big_sub(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_int_mul(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        lean_int64_to_int(lean_scalar_to_int64(a1) * lean_scalar_to_int64(a2))
    } else {
        lean_int_big_mul(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_int_div(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        let a2 = lean_scalar_to_int64(a2);
        if a2 == 0 {
            lean_box(0)
        } else {
            lean_int64_to_int(lean_scalar_to_int64(a1) / a2)
        }
    } else {
        lean_int_big_div(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_int_mod(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        let a2 = lean_scalar_to_int64(a2);
        if a2 == 0 {
            a1
        } else {
            lean_int64_to_int(lean_scalar_to_int64(a1) % a2)
        }
    } else {
        lean_int_big_mod(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_int_eq(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> bool {
    a1 == a2 || lean_int_big_eq(a1, a2)
}

#[inline(always)]
pub unsafe fn lean_int_ne(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> bool {
    !lean_int_eq(a1, a2)
}

#[inline]
pub unsafe fn lean_int_le(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> bool {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        lean_scalar_to_int64(a1) <= lean_scalar_to_int64(a2)
    } else {
        lean_int_big_le(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_int_lt(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> bool {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        lean_scalar_to_int64(a1) < lean_scalar_to_int64(a2)
    } else {
        lean_int_big_lt(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_int_to_nat(a: lean_obj_arg) -> lean_obj_res {
    debug_assert!(!lean_int_lt(a, lean_box(0)));
    if lean_is_scalar(a) {
        a
    } else {
        lean_big_int_to_nat(a)
    }
}

#[inline]
pub unsafe fn lean_nat_abs(i: b_lean_obj_arg) -> lean_obj_res {
    if lean_int_lt(i, lean_box(0)) {
        lean_int_to_nat(lean_int_neg(i))
    } else {
        lean_inc(i);
        lean_int_to_nat(i)
    }
}

#[inline]
pub unsafe fn lean_int_dec_eq(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> u8 {
    lean_int_eq(a1, a2) as u8
}

#[inline]
pub unsafe fn lean_int_dec_le(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> u8 {
    lean_int_le(a1, a2) as u8
}

#[inline]
pub unsafe fn lean_int_dec_lt(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> u8 {
    lean_int_lt(a1, a2) as u8
}

#[inline]
pub unsafe fn lean_int_dec_nonneg(a: b_lean_obj_arg) -> u8 {
    (if lean_is_scalar(a) {
        //TODO: likely
        lean_scalar_to_int(a) >= 0
    } else {
        lean_int_big_nonneg(a)
    }) as u8
}

extern "C" {
    pub fn lean_int_big_neg(a: *mut lean_object) -> *mut lean_object;
    pub fn lean_int_big_add(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_int_big_sub(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_int_big_mul(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_int_big_div(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_int_big_mod(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_int_big_eq(a1: *mut lean_object, a2: *mut lean_object) -> bool;
    pub fn lean_int_big_le(a1: *mut lean_object, a2: *mut lean_object) -> bool;
    pub fn lean_int_big_lt(a1: *mut lean_object, a2: *mut lean_object) -> bool;
    pub fn lean_int_big_nonneg(a: *mut lean_object) -> bool;

    pub fn lean_cstr_to_int(n: *const u8) -> *mut lean_object;
    pub fn lean_big_int_to_int(n: c_int) -> *mut lean_object;
    pub fn lean_big_size_t_to_int(n: usize) -> *mut lean_object;
    pub fn lean_big_int64_to_int(n: i64) -> *mut lean_object;

    pub fn lean_big_int_to_nat(a: lean_obj_arg) -> lean_obj_res;
}
