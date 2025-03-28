mod add_assign;
mod format;
mod from;
mod mul_assign;
mod negate;
mod shift_limbs;
mod shr_assign;
mod sub_assign;

use super::{BASE_F64, Digit};

// Non-negative integer.
#[derive(Debug, Clone)]
pub struct Integer {
    pub limbs: Vec<Digit>,
}

impl Integer {
    pub fn len(&self) -> usize {
        self.limbs.len()
    }

    pub fn get_leading(&self, n: usize) -> Integer {
        let n = n.min(self.len());
        Integer {
            limbs: self.limbs[(self.len() - n)..].to_vec(),
        }
    }

    pub fn get_leading_as_f64(&self, n: usize) -> f64 {
        let mut iter = self.limbs.iter().rev();

        let mut x = 0f64;
        for _ in 0..n {
            x = x * BASE_F64 + *iter.next().unwrap() as f64;
        }
        x
    }
}
