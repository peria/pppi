use crate::formula::{PiFormula, SplitTerm};

pub fn split<F: PiFormula>(formula: &F, a: usize, b: usize) -> SplitTerm {
    assert!(a < b, "empty DRM interval");

    if b - a == 1 {
        return formula.term(a);
    }

    let m = (a + b) / 2;
    let left = split(formula, a, m);
    let right = split(formula, m, b);

    SplitTerm {
        p: left.p.mul(&right.p),
        q: left.q.mul(&right.q),
        t_positive: left
            .t_positive
            .mul(&right.q)
            .add(&left.p.mul(&right.t_positive)),
        t_negative: left
            .t_negative
            .mul(&right.q)
            .add(&left.p.mul(&right.t_negative)),
    }
}
