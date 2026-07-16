use crate::bigint::Integer;
use crate::binary_fixed::Fixed;
use crate::drm::{Drm3Term, DrmFormula, SignedDrm3Formula};

pub trait Formula {
    fn compute(&self, digits: usize) -> Fixed;
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
        let mut term = self.drm(0, n);
        let (x, y) = (term.x, term.y);
        y *= 5;
        // A * x - y
        let mut denominator = Integer::axmy(Self::A, &x, &y);
        let mut numerator = x;
        numerator *= 4270934400;
        (numerator, denominator)
    }
}

impl SignedDrm3Formula for Chudnovsky {}

// TODO: Make this part as a trait.
impl Formula for Chudnovsky {
    fn compute(&self, digits: usize) -> Fixed {
        let terms = self.terms_for_digits(digits);
        let precision_bits = bits_for_decimal_digits(digits + 12);
        let (x, y) = self.run(terms);
        let reciprocal = Fixed::divide(y, x, precision_bits);
        let mut value = Fixed::inv_sqrt_u64(self.inv_sqrt_u64(), precision_bits);
        value *= &reciprocal;
        value
    }
}

pub trait SquareRootFormula: Formula {
    fn inv_sqrt_u64(&self) -> u64;
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
