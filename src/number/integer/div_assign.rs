use std::ops::DivAssign;

use super::Integer;

impl DivAssign<&Self> for Integer {
    fn div_assign(&mut self, rhs: &Self) {}
}
