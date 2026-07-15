use crate::{bigint::Integer, binary_fixed::Fixed};

#[derive(Debug, Clone)]
pub struct DrmTerm {
    pub p: Integer,
    pub q: Integer,
    pub t_positive: Integer,
    pub t_negative: Integer,
}

impl DrmTerm {
    pub fn from_signed_t(p: Integer, q: Integer, t: Integer, t_is_negative: bool) -> Self {
        let (t_positive, t_negative) = if t_is_negative {
            (Integer::zero(), t)
        } else {
            (t, Integer::zero())
        };

        Self {
            p,
            q,
            t_positive,
            t_negative,
        }
    }

    pub fn t_magnitude_when_positive(&self) -> Integer {
        assert!(self.t_positive >= self.t_negative);
        &self.t_positive - &self.t_negative
    }
}

pub trait Formula {
    fn compute(&self, digits: usize) -> Fixed;
}

pub trait PiFormula {
    fn term(&self, k: usize) -> DrmTerm;
    fn terms_for_digits(&self, digits: usize) -> usize;
    fn multiplier(&self) -> u64;
    fn sqrt_u64(&self) -> u64;
}

#[derive(Debug, Clone, Copy)]
pub struct Chudnovsky;

impl PiFormula for Chudnovsky {
    fn term(&self, k: usize) -> DrmTerm {
        if k == 0 {
            return DrmTerm::from_signed_t(
                Integer::one(),
                Integer::one(),
                Integer::from(13_591_409),
                false,
            );
        }

        let k_u64 = k as u64;
        let p = &(&Integer::from(6 * k_u64 - 5) * &Integer::from(2 * k_u64 - 1))
            * &Integer::from(6 * k_u64 - 1);
        let k_int = Integer::from(k_u64);
        let q = &(&k_int * &k_int) * &k_int;
        let q = q.mul_u64(10_939_058_860_032_000);
        let t = p.mul_u64(13_591_409 + 545_140_134 * k_u64);

        DrmTerm::from_signed_t(p, q, t, k % 2 != 0)
    }

    fn terms_for_digits(&self, digits: usize) -> usize {
        (digits as f64 / 14.181_647_462_725_477).ceil() as usize + 1
    }

    fn multiplier(&self) -> u64 {
        426_880
    }

    fn sqrt_u64(&self) -> u64 {
        10_005
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

fn bits_for_decimal_digits(digits: usize) -> usize {
    ((digits as f64) * std::f64::consts::LOG2_10).ceil() as usize
}
