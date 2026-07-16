use crate::bigint::Integer;
use crate::formula::Formula;

pub struct Drm3Term {
    pub x: Integer,
    pub y: Integer,
    pub z: Integer,
}

pub trait DrmFormula<T>: Formula {
    fn terms_for_digits(&self, digits: usize) -> usize;
    fn term(&self, k: usize) -> T;
    fn run(&self, n: usize) -> (Integer, Integer);
}

pub trait SignedDrm3Formula: DrmFormula<Drm3Term> {
    fn drm(&self, n0: usize, n1: usize) -> Drm3Term {
        if n0 + 1 >= n1 {
            return self.term(n1);
        }

        let m = (n0 + n1) / 2;
        let mut left = self.drm(n0, m);
        let mut right = self.drm(m, n1);
        left.y *= &right.x;
        right.y *= &left.z;
        if (m - n0) % 2 == 1 {
            left.y -= &right.y;
        } else {
            left.y += &right.y;
        }
        left.x *= &right.x;
        left.z *= &right.z;
        left
    }
}
