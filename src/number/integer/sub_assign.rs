use std::ops::SubAssign;

use super::Integer;

impl SubAssign<&Self> for Integer {
    fn sub_assign(&mut self, rhs: &Self) {
        debug_assert!(self.len() >= rhs.len());
        let yn = rhs.len();

        let mut carry = false;
        for (xi, yi) in self.limbs.iter_mut().zip(rhs.limbs.iter()) {
            let (s, c1) = xi.overflowing_sub(*yi);
            let (s, c2) = s.overflowing_sub(if carry { 1 } else { 0 });
            *xi = s;
            carry = c1 || c2;
        }
        if carry {
            for xi in self.limbs[yn..].iter_mut() {
                let (s, c) = xi.overflowing_sub(1);
                *xi = s;
                if !c {
                    break;
                }
            }
        }

        let mut n = self.limbs.len();
        while self.limbs[n - 1] == 0 {
            n -= 1;
        }
        if n != self.limbs.len() {
            self.limbs.resize(n, 0);
        }
    }
}
