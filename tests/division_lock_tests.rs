use sovereign_math::math::division_lock::LockedDivMod;
use core::num::NonZeroI32;

#[test]
fn test_euclidean_correctness() {
    let cases = [
        (10, 3, 3, 1),
        (-10, 3, -4, 2),
        (10, -3, -3, 1),
        (-10, -3, 4, 2),
        (0, 5, 0, 0),
        (7, 7, 1, 0),
    ];
    for (n, d_raw, q_exp, r_exp) in cases {
        let d = NonZeroI32::new(d_raw).unwrap();
        let res = d.locked_div_mod(n);
        assert_eq!(res.q_euclid, q_exp, "Q mismatch for ({}, {})", n, d_raw);
        assert_eq!(res.r_euclid, r_exp, "R mismatch for ({}, {})", n, d_raw);
        assert_eq!(n, res.q_euclid.wrapping_mul(d_raw).wrapping_add(res.r_euclid));
    }
}

#[test]
fn test_java_native_semantics_preserved() {
    let d = NonZeroI32::new(3).unwrap();
    let res = d.locked_div_mod(-10);
    assert_eq!(res.q_trunc, -3);
    assert_eq!(res.r_trunc, -1);
    assert_eq!(-10, res.q_trunc * 3 + res.r_trunc);
}
