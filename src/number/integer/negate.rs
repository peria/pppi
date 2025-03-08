use super::Integer;

impl Integer {
    pub fn negate(&mut self) {
        for xi in self.limbs.iter_mut() {
            *xi = !*xi;
        }

        for xi in self.limbs.iter_mut() {
            let (s, carry) = xi.overflowing_add(1);
            *xi = s;
            if !carry {
                break;
            }
        }

        // Remove leading zeros.
        let mut n = self.len();
        while n > 0 && self.limbs[n - 1] == 0 {
            n -= 1;
        }
        if n != self.len() {
            self.limbs.resize(n, 0);
        }
    }
}
