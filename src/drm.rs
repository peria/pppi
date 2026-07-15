use crate::formula::{DrmTerm, PiFormula};

pub fn split<F: PiFormula>(formula: &F, a: usize, b: usize) -> DrmTerm {
    assert!(a < b, "empty DRM interval");

    if b - a == 1 {
        return formula.term(a);
    }

    let m = (a + b) / 2;
    let left = split(formula, a, m);
    let right = split(formula, m, b);

    DrmTerm {
        p: &left.p * &right.p,
        q: &left.q * &right.q,
        t_positive: &(&left.t_positive * &right.q) + &(&left.p * &right.t_positive),
        t_negative: &(&left.t_negative * &right.q) + &(&left.p * &right.t_negative),
    }
}
