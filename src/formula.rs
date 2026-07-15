use crate::{bigint::Integer, binary_fixed::Fixed};

pub struct DrmTerm {
    pub p: Integer,
    pub q: Integer,
    pub t: Integer,
    pub is_signed: bool,
}

pub trait Formula {
    fn compute(&self, digits: usize) -> Fixed;
}

pub trait DrmFormula<T>: Formula {
    fn term(&self, k: usize) -> T;
    fn terms_for_digits(&self, digits: usize) -> usize;
    fn drm(&self, n0: usize, n1: usize) -> T;
}

pub trait SquareRootFormula: Formula {
    fn multiplier(&self) -> u64;
    fn inv_sqrt_u64(&self) -> u64;
}

pub trait PiFormula: DrmFormula<DrmTerm> + SquareRootFormula {}

#[derive(Debug, Clone, Copy)]
pub struct Chudnovsky;

impl DrmFormula<DrmTerm> for Chudnovsky {
    fn term(&self, k: usize) -> DrmTerm {
        let k = k as u64;
        let mut x = Integer::from(k * Self::C);
        x *= k * Self::C;
        x *= k * Self::C / 24;
        let mut y = Integer::from(Self::A + Self::B * k);
        let mut z = Integer::from(6 * k - 1);
        z *= 2 * k - 1;
        z *= 6 * k - 5;
        DrmTerm { x, y, z }
    }

    fn terms_for_digits(&self, digits: usize) -> usize {
        (digits as f64 / 14.181_647_462_725_477).ceil() as usize + 1
    }

    fn drm(&self, n0: usize, n1: usize) -> DrmTerm {
        if n0 + 1 >= n1 {
            return self.term(n1);
        }

        let m = (n0 + n1) / 2;
        let mut left = self.drm(n0, m);
        let right = self.drm(m, n1);
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

impl Formula for Chudnovsky {
    fn compute(&self, digits: usize) -> Fixed {
        let terms = self.terms_for_digits(digits);
        let precision_bits = bits_for_decimal_digits(digits + 12);
        let split = crate::drm::split(self, 0, terms);
        let sum = split.t_magnitude_when_positive();

        let reciprocal_sum =
            crate::binary_fixed::divide_biguints_newton(&split.q, &sum, precision_bits);
        let inv_sqrt = Fixed::inv_sqrt_u64(self.sqrt_u64(), precision_bits);
        let sqrt = inv_sqrt.mul_u64(self.sqrt_u64());
        let value = sqrt.mul_u64(self.multiplier()).mul(&reciprocal_sum);

        value
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
