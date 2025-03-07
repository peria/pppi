use std::ops::MulAssign;

use super::Digit;
use super::Integer;

impl MulAssign<Digit> for Integer {
    fn mul_assign(&mut self, rhs: Digit) {
        let rhs = rhs as u128;
        let mut carry = 0;
        for xi in self.limbs.iter_mut() {
            let prod = *xi as u128 * rhs + carry;
            *xi = prod as Digit;
            carry = prod >> 64;
        }
        if carry > 0 {
            self.limbs.push(carry as Digit);
        }
    }
}

impl MulAssign<&Self> for Integer {
    fn mul_assign(&mut self, rhs: &Self) {
        let xn = self.len();
        let yn = rhs.len();
        let zn = xn + yn;

        let mut z = vec![0 as Digit; zn];
        for (i, xi) in self.limbs.iter_mut().enumerate() {
            let xi = *xi as u128;
            let mut carry = 0;
            for (j, yj) in rhs.limbs.iter().enumerate() {
                let prod = *yj as u128 * xi + carry + z[i + j] as u128;
                z[i + j] = prod as Digit;
                carry = prod >> 64;
            }
            z[i + yn] = carry as Digit;
        }
        if z[zn - 1] == 0 {
            z.resize(zn - 1, 0);
        }
        std::mem::swap(&mut z, &mut self.limbs);
    }
}

impl Integer {
    pub fn square(&self) -> Self {
        let xn = self.len();
        let zn = 2 * xn;

        let mut z = vec![0 as Digit; zn];
        for (i, xi) in self.limbs.iter().enumerate() {
            let xi = *xi as u128;
            let mut carry = 0;
            for (j, xj) in self.limbs.iter().enumerate() {
                let prod = *xj as u128 * xi + carry + z[i + j] as u128;
                z[i + j] = prod as Digit;
                carry = prod >> 64;
            }
            z[i + xn] = carry as Digit;
        }
        if z[zn - 1] == 0 {
            z.resize(zn - 1, 0);
        }

        Integer { limbs: z }
    }
}
