use crate::{bigint::Integer, binary_fixed::Fixed};

pub struct Drm3Term {
    pub x: Integer,
    pub y: Integer,
    pub z: Integer,
}

pub trait Formula {
    fn compute(&self, digits: usize) -> Fixed;
}

pub trait DrmFormula<T>: Formula {
    fn term(&self, k: usize) -> T;
    fn terms_for_digits(&self, digits: usize) -> usize;
    fn run(&self, n: usize) -> (Integer, Integer);
}

pub trait SquareRootFormula: Formula {
    fn inv_sqrt_u64(&self) -> u64;
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

#[derive(Debug, Clone, Copy)]
pub struct Chudnovsky;

impl DrmFormula<Drm3Term> for Chudnovsky {
    fn term(&self, k: usize) -> Drm3Term {
        let k = k as u64;
        let mut x = Integer::from(k * Self::C);
        x *= k * Self::C;
        x *= k * Self::C / 24;
        let y = Integer::from(Self::A + Self::B * k);
        let mut z = Integer::from(6 * k - 1);
        z *= 2 * k - 1;
        z *= 6 * k - 5;
        Drm3Term { x, y, z }
    }

    fn terms_for_digits(&self, digits: usize) -> usize {
        (digits as f64 / 14.181_647_462_725_477).ceil() as usize + 1
    }

    fn run(&self, n: usize) -> (Integer, Integer) {
        let term = self.drm(0, n);
        let denominator = term.x * Self::A - term.y * 5;
        let numerator = term.x * 4270934400;
        (numerator, denominator)
    }
}

impl SignedDrm3Formula for Chudnovsky {}

// TODO: Make this part as a trait.
impl Formula for Chudnovsky {
    fn compute(&self, digits: usize) -> Fixed {
        let terms = self.terms_for_digits(digits);
        let precision_bits = bits_for_decimal_digits(digits + 12);
        // BUG: Final part of DRM is not well defined.
        let drm_terms = self.drm(0, terms);
        // `z` is no longer used.
        let (x, y) = (drm_terms.x, drm_terms.y);
        let reciprocal_sum = Fixed::divide(&y, &x, precision_bits);
        let mut value = Fixed::inv_sqrt_u64(self.inv_sqrt_u64(), precision_bits);
        value *= &reciprocal_sum;
        value
    }
}

impl SquareRootFormula for Chudnovsky {
    fn inv_sqrt_u64(&self) -> u64 {
        10005
    }
}

impl Chudnovsky {
    const A: u64 = 13591409;
    const B: u64 = 545140134;
    const C: u64 = 640320;
}

fn bits_for_decimal_digits(digits: usize) -> usize {
    ((digits as f64) * std::f64::consts::LOG2_10).ceil() as usize
}
