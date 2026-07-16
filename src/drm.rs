use crate::formula::{Drm3Term, PiFormula};

pub fn split<F: PiFormula>(formula: &F, a: usize, b: usize) -> Drm3Term {
    assert!(a < b, "empty DRM interval");

    if b - a == 1 {
        return formula.term(a);
    }

    let m = (a + b) / 2;
    let left = split(formula, a, m);
    let right = split(formula, m, b);

    Drm3Term {
        p: &left.p * &right.p,
        q: &left.q * &right.q,
        t_positive: &(&left.t_positive * &right.q) + &(&left.p * &right.t_positive),
        t_negative: &(&left.t_negative * &right.q) + &(&left.p * &right.t_negative),
    }
}
