/*! Natural numbers */
use crate::*;

pub const LEAN_MAX_SMALL_NAT: usize = usize::MAX >> 1;

#[inline]
pub unsafe fn lean_usize_to_nat(n: usize) -> lean_obj_res {
    if n <= LEAN_MAX_SMALL_NAT {
        //TODO: likely
        lean_box(n)
    } else {
        lean_big_usize_to_nat(n)
    }
}

#[inline(always)]
pub unsafe fn lean_unsigned_to_nat(n: c_uint) -> lean_obj_res {
    lean_usize_to_nat(n as usize)
}

#[inline]
pub unsafe fn lean_uint64_to_nat(n: u64) -> lean_obj_res {
    if n <= LEAN_MAX_SMALL_NAT as u64 {
        //TODO: likely
        lean_box(n as usize)
    } else {
        lean_big_uint64_to_nat(n)
    }
}

#[inline]
pub unsafe fn lean_nat_succ(a: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a) {
        //TODO: likely
        lean_usize_to_nat(lean_unbox(a) + 1)
    } else {
        lean_nat_big_succ(a)
    }
}

#[inline]
pub unsafe fn lean_nat_add(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        lean_usize_to_nat(lean_unbox(a1) + lean_unbox(a2))
    } else {
        lean_nat_big_add(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_nat_sub(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        lean_box(lean_unbox(a1).saturating_sub(lean_unbox(a2)))
    } else {
        lean_nat_big_sub(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_nat_mul(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        let a1 = lean_unbox(a1);
        let a2 = lean_unbox(a2);
        match a1.checked_mul(a2) {
            Some(r) if r <= LEAN_MAX_SMALL_NAT => lean_box(r),
            _ => lean_nat_overflow_mul(a1, a2),
        }
    } else {
        lean_nat_big_mul(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_nat_div(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        let n1 = lean_unbox(a1);
        let n2 = lean_unbox(a2);
        lean_box(if n2 == 0 { 0 } else { n1.saturating_div(n2) })
    } else {
        lean_nat_big_div(a1, a2)
    }
}

/// assumes that a1 % a2 = 0
#[inline]
pub unsafe fn lean_nat_div_exact(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        let n1 = lean_unbox(a1);
        let n2 = lean_unbox(a2);
        lean_box(if n2 == 0 { 0 } else { n1.saturating_div(n2) })
    } else {
        lean_nat_big_div_exact(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_nat_mod(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        let a2 = lean_unbox(a2);
        if a2 == 0 {
            a1
        } else {
            lean_box(lean_unbox(a1) % a2)
        }
    } else {
        lean_nat_big_mod(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_nat_eq(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> bool {
    a1 == a2 || lean_nat_big_eq(a1, a2)
}

#[inline(always)]
pub unsafe fn lean_nat_dec_eq(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> u8 {
    lean_nat_eq(a1, a2) as u8
}

#[inline]
pub unsafe fn lean_nat_le(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> bool {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        lean_unbox(a1) <= lean_unbox(a2)
    } else {
        lean_nat_big_le(a1, a2)
    }
}

#[inline(always)]
pub unsafe fn lean_nat_dec_le(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> u8 {
    lean_nat_le(a1, a2) as u8
}

#[inline]
pub unsafe fn lean_nat_lt(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> bool {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        lean_unbox(a1) < lean_unbox(a2)
    } else {
        lean_nat_big_lt(a1, a2)
    }
}

#[inline(always)]
pub unsafe fn lean_nat_dec_lt(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> u8 {
    lean_nat_lt(a1, a2) as u8
}

#[inline]
pub unsafe fn lean_nat_land(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        lean_box(lean_unbox(a1) & lean_unbox(a2))
    } else {
        lean_nat_big_land(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_nat_lor(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        lean_box(lean_unbox(a1) | lean_unbox(a2))
    } else {
        lean_nat_big_lor(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_nat_lxor(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        lean_box(lean_unbox(a1) ^ lean_unbox(a2))
    } else {
        lean_nat_big_xor(a1, a2)
    }
}

#[inline]
pub unsafe fn lean_nat_shiftr(a1: b_lean_obj_arg, a2: b_lean_obj_arg) -> lean_obj_res {
    if lean_is_scalar(a1) && lean_is_scalar(a2) {
        //TODO: likely
        let s1 = lean_unbox(a1);
        let s2 = lean_unbox(a2);
        let r = if s2 < size_of::<usize>() * 8 {
            s1 >> s2
        } else {
            0
        };
        lean_box(r)
    } else {
        lean_nat_big_shiftr(a1, a2)
    }
}

extern "C" {
    pub fn lean_nat_big_succ(a: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_add(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_sub(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_mul(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_overflow_mul(a1: usize, a2: usize) -> *mut lean_object;
    pub fn lean_nat_big_div(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_div_exact(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_mod(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_eq(a1: *mut lean_object, a2: *mut lean_object) -> bool;
    pub fn lean_nat_big_le(a1: *mut lean_object, a2: *mut lean_object) -> bool;
    pub fn lean_nat_big_lt(a1: *mut lean_object, a2: *mut lean_object) -> bool;
    pub fn lean_nat_big_land(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_lor(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_xor(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;

    pub fn lean_cstr_to_nat(n: *const u8) -> *mut lean_object;
    pub fn lean_big_usize_to_nat(n: usize) -> *mut lean_object;
    pub fn lean_big_uint64_to_nat(n: u64) -> *mut lean_object;

    pub fn lean_nat_shiftl(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_big_shiftr(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_pow(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_gcd(a1: *mut lean_object, a2: *mut lean_object) -> *mut lean_object;
    pub fn lean_nat_log2(a: *mut lean_object) -> *mut lean_object;
}

#[cfg(test)]
mod test {
    extern crate std;
    use rand::{prelude::SliceRandom, Rng, SeedableRng};

    use super::*;

    unsafe fn slice_mul(s: &[u64]) -> *mut lean_object {
        let r = s
            .iter()
            .map(|&x| lean_uint64_to_nat(x))
            .fold(lean_uint64_to_nat(0), |a, b| lean_nat_mul(a, b));
        assert!(lean_is_scalar(r) || lean_is_mpz(r));
        r
    }

    #[test]
    fn big_nat_multiplication_commutes_test() {
        let mut rng = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(0x568478687);
        unsafe {
            lean_initialize_runtime_module_locked();
            let mut vec = std::vec::Vec::with_capacity(100);
            for _ in 0..10 {
                for _ in 0..100 {
                    vec.push(rng.random::<u64>())
                }
                let r = slice_mul(&vec[..]);
                const M: u64 = 595468;
                let m = lean_nat_mod(r, lean_uint64_to_nat(M));
                assert!(lean_is_scalar(m));
                let p = vec.iter().copied().fold(1, |l: u64, r| l.wrapping_mul(r)) % M;
                assert_eq!(lean_unbox(m) as u64, p);
                for _ in 0..4 {
                    vec.shuffle(&mut rng);
                    assert!(lean_nat_eq(r, slice_mul(&vec[..])));
                }

                vec.clear();
            }
        }
    }
}
