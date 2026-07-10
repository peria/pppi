use crate::bigint::BigUint;

#[derive(Debug, Clone)]
pub struct SplitTerm {
    pub p: BigUint,
    pub q: BigUint,
    pub t_positive: BigUint,
    pub t_negative: BigUint,
}

impl SplitTerm {
    pub fn from_signed_t(p: BigUint, q: BigUint, t: BigUint, t_is_negative: bool) -> Self {
        let (t_positive, t_negative) = if t_is_negative {
            (BigUint::zero(), t)
        } else {
            (t, BigUint::zero())
        };

        Self {
            p,
            q,
            t_positive,
            t_negative,
        }
    }

    pub fn t_magnitude_when_positive(&self) -> BigUint {
        assert!(self.t_positive >= self.t_negative);
        self.t_positive.sub(&self.t_negative)
    }
}

pub trait PiFormula {
    fn term(&self, k: usize) -> SplitTerm;
    fn terms_for_digits(&self, digits: usize) -> usize;
    fn multiplier(&self) -> u64;
    fn sqrt_u64(&self) -> u64;
}

#[derive(Debug, Clone, Copy)]
pub struct Chudnovsky;

impl PiFormula for Chudnovsky {
    fn term(&self, k: usize) -> SplitTerm {
        if k == 0 {
            return SplitTerm::from_signed_t(
                BigUint::one(),
                BigUint::one(),
                BigUint::from_u64(13_591_409),
                false,
            );
        }

        let k_u64 = k as u64;
        let p = BigUint::from_u64(6 * k_u64 - 5)
            .mul(&BigUint::from_u64(2 * k_u64 - 1))
            .mul(&BigUint::from_u64(6 * k_u64 - 1));
        let q = BigUint::from_u64(k_u64)
            .mul(&BigUint::from_u64(k_u64))
            .mul(&BigUint::from_u64(k_u64))
            .mul_u64(10_939_058_860_032_000);
        let t = p.mul_u64(13_591_409 + 545_140_134 * k_u64);

        SplitTerm::from_signed_t(p, q, t, k % 2 != 0)
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
