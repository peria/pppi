use std::ops::ShrAssign;

use super::Integer;

impl ShrAssign<usize> for Integer {
    fn shr_assign(&mut self, rhs: usize) {
        debug_assert!(rhs < 64);

        let mask = (1u64 << rhs) - 1;
        let mut carry = 0;
        for xi in self.limbs.iter_mut().rev() {
            let next = *xi & mask;
            *xi = (*xi >> rhs) + (carry << (64 - rhs));
            carry = next;
        }
    }
}
