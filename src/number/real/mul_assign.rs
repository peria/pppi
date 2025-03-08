use std::ops::MulAssign;

use super::Real;

impl MulAssign<u64> for Real {
    fn mul_assign(&mut self, rhs: u64) {
        self.mantissa *= rhs;
    }
}
