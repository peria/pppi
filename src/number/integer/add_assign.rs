use std::ops::AddAssign;

use super::Integer;

impl AddAssign<&Self> for Integer {
    fn add_assign(&mut self, rhs: &Self) {
        let xn = self.len();
        let yn = rhs.len();

        if xn < yn {
            self.limbs.resize(yn, 0);
            self.limbs[xn..]
                .iter_mut()
                .zip(rhs.limbs[xn..].iter())
                .for_each(|(xi, yi)| *xi = *yi);
        }

        let mut carry = false;
        for (xi, yi) in self.limbs[..xn].iter_mut().zip(rhs.limbs.iter()) {
            let (s, c1) = xi.overflowing_add(*yi);
            let (s, c2) = s.overflowing_add(if carry { 1 } else { 0 });
            *xi = s;
            carry = c1 || c2;
        }
        if carry {
            for xi in self.limbs[xn..].iter_mut() {
                let (s, c) = xi.overflowing_add(1);
                *xi = s;
                if !c {
                    carry = false;
                    break;
                }
            }
            if carry {
                self.limbs.push(1);
            }
        }
    }
}
