use crate::number::BASE_F64;

use super::Digit;
use super::Integer;

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        let mut limbs = Vec::new();
        limbs.push(value);

        Integer { limbs }
    }
}

impl From<f64> for Integer {
    fn from(mut value: f64) -> Self {
        let mut len = 0;
        while value >= 1.0 {
            value /= BASE_F64;
            len += 1;
        }

        let mut limbs = vec![0; len];
        for xi in limbs.iter_mut().rev() {
            value *= BASE_F64;
            let limb = value as Digit;
            value -= limb as f64;
            *xi = limb;
        }
        Integer { limbs }
    }
}
