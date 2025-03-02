use std::ops::MulAssign;

use super::Digit;
use super::Integer;

impl MulAssign<Digit> for Integer {
    fn mul_assign(&mut self, rhs: Digit) {
        for xi in self.limbs.iter_mut() {
            let (p, c) = xi.overflowing_mul(rhs);
        }
    }
}

impl MulAssign<&Self> for Integer {
    fn mul_assign(&mut self, rhs: &Self) {}
}
