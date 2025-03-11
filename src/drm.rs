use crate::number::Digit;
use crate::number::integer::Integer;

fn chudnovsky_drm(num_digits: usize) -> (Integer, Integer) {
    let num_terms = get_num_terms(num_digits);
    let (x, y, z) = chudnovsky_drm_recursive(0, num_digits);
    (x, y)
}

fn get_num_terms(digits: usize) -> usize {
    // TODO: Use more precise estimation.
    const DIGITS_PER_TERM: f64 = 14.1816474627;

    let terms = digits as f64 / DIGITS_PER_TERM;
    let terms = terms as usize + 3;
    terms
}

fn chudnovsky_drm_recursive(n0: usize, n1: usize) -> (Integer, Integer, Integer) {
    const A: Digit = 13591409;
    const B: Digit = 545140134;
    const C: Digit = 640320;

    if n1 - n0 == 1 {
        let n = n0 as u64;

        let mut x = Integer::from(n * C / 24);
        x *= n * C;
        x *= n * C;
        let y = Integer::from(A + B * n);
        let mut z = Integer::from(n * 6 + 5);
        z *= n * 6 + 1;
        z *= n * 2 + 1;

        return (x, y, z);
    }

    let m = (n0 + n1) / 2;
    let (mut x0, mut y0, mut z0) = chudnovsky_drm_recursive(n0, m);
    let (x1, mut y1, z1) = chudnovsky_drm_recursive(m, n1);
    x0 *= &x1;
    y0 *= &x1;
    y1 *= &z0;
    y0 += &y1;
    z0 *= &z1;

    (x0, y0, z0)
}

// TODO: Write a test
