use crate::number::Digit;
use crate::number::integer::Integer;

fn chudnovsky_drm(num_digits: usize) -> (Integer, Integer) {
    let num_terms = get_num_terms(num_digits);
    let mut x = Integer::from(1);
    let mut y = Integer::from(1);
    let mut z = Integer::from(1);

    chudnovsky_drm_recursive(0, num_digits, &mut x, &mut y, &mut z);
    (x, y)
}

fn get_num_terms(digits: usize) -> usize {
    // TODO: Use more precise estimation.
    const DIGITS_PER_TERM: f64 = 14.1816474627;

    let terms = digits as f64 / DIGITS_PER_TERM;
    let terms = terms as usize + 3;
    terms
}

fn chudnovsky_drm_recursive(
    n0: usize,
    n1: usize,
    x0: &mut Integer,
    y0: &mut Integer,
    z0: &mut Integer,
) {
    const A: Digit = 13591409;
    const B: Digit = 545140134;
    const C: Digit = 640320;

    if n1 - n0 == 1 {
        let n = n0 as u64;

        *x0 = Integer::from(n * C / 24);
        *x0 *= n * C;
        *x0 *= n * C;
        *y0 = Integer::from(A + B * n);
        *z0 = Integer::from(n * 6 + 5);
        *z0 *= n * 6 + 1;
        *z0 *= n * 2 + 1;

        return;
    }

    let m = (n0 + n1) / 2;
    chudnovsky_drm_recursive(n0, m, x0, y0, z0);

    let mut x1 = Integer::from(1);
    let mut y1 = Integer::from(1);
    let mut z1 = Integer::from(1);
    chudnovsky_drm_recursive(m, n1, &mut x1, &mut y1, &mut z1);

    *x0 *= &x1;
    *y0 *= &x1;
    y1 *= &*z0;
    *y0 += &y1;
    *z0 *= &z1;
}
