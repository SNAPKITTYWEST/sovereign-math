#![no_std]
#![cfg_attr(feature = "verification", feature(register_attr))]
#![cfg_attr(feature = "verification", register_attr(verifier))]

use core::num::NonZeroI32;

/// Result container holding both Java Native and Euclidean semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "verification", verifier::transparent)]
pub struct DivModResult {
    /// Java Truncated Division: n = q_trunc * d + r_trunc, sign(r_trunc) == sign(n)
    pub q_trunc: i32,
    pub r_trunc: i32,
    /// Euclidean Division: n = q_euclid * d + r_euclid, 0 <= r_euclid < |d|
    pub q_euclid: i32,
    pub r_euclid: i32,
}

/// Core Trait defining the Locked Division Contract.
pub trait LockedDivMod {
    fn locked_div_mod(self, n: i32) -> DivModResult;
}

impl LockedDivMod for NonZeroI32 {
    #[inline(always)]
    #[cfg_attr(feature = "verification", verifier::exec_static)]
    fn locked_div_mod(self, n: i32) -> DivModResult {
        let d = self.get();
        let (q_trunc, r_trunc) = if n == i32::MIN && d == -1 {
            (i32::MIN, 0)
        } else {
            (n / d, n % d)
        };
        let is_neg = (r_trunc >> 31) as i32;
        let d_abs = d.abs();
        let d_abs_safe = if d == i32::MIN { 0 } else { d_abs };
        let adjust = is_neg & d_abs_safe;
        let r_euclid = r_trunc + adjust;
        let q_adjust = is_neg * d.signum();
        let q_euclid = q_trunc - q_adjust;
        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(n, q_trunc.wrapping_mul(d).wrapping_add(r_trunc));
            debug_assert!(r_trunc > -d_abs && r_trunc < d_abs, "Java Remainder Bound Violated");
            debug_assert_eq!(n, q_euclid.wrapping_mul(d).wrapping_add(r_euclid));
            debug_assert!((0..d_abs as u32).contains(&(r_euclid as u32)), "Euclidean Remainder Bound Violated");
        }
        DivModResult { q_trunc, r_trunc, q_euclid, r_euclid }
    }
}

#[inline(always)]
pub fn locked_modulo_euclidean(n: i32, d: NonZeroI32) -> i32 {
    d.locked_div_mod(n).r_euclid
}

#[inline(always)]
pub fn locked_divide_euclidean(n: i32, d: NonZeroI32) -> i32 {
    d.locked_div_mod(n).q_euclid
}

#[cfg(feature = "verification")]
mod verification {
    use super::*;
    use core::num::NonZeroI32;
    #[kani::proof]
    #[kani::unwind(1)]
    fn verify_euclidean_lock() {
        let n: i32 = kani::any();
        let d_raw: i32 = kani::any();
        kani::assume(d_raw != 0);
        kani::assume(!(n == i32::MIN && d_raw == -1));
        let d = unsafe { NonZeroI32::new_unchecked(d_raw) };
        let res = d.locked_div_mod(n);
        assert_eq!(n, res.q_euclid.wrapping_mul(d_raw).wrapping_add(res.r_euclid));
        let d_abs = d_raw.abs() as u32;
        let r_u = res.r_euclid as u32;
        assert!(r_u < d_abs);
        assert!(res.r_euclid >= 0);
        assert_eq!(n, res.q_trunc.wrapping_mul(d_raw).wrapping_add(res.r_trunc));
        assert!(res.r_trunc > -(d_abs as i32));
        assert!(res.r_trunc < (d_abs as i32));
    }
}
