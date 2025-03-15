use super::Integer;

impl Integer {
    pub fn shift_up_limbs(&mut self, n: usize) {
        let xn = self.len();
        self.limbs.resize(xn + n, 0);
        for i in (0..xn).rev() {
            self.limbs[i + n] = self.limbs[i];
        }
        for xi in self.limbs[..n].iter_mut() {
            *xi = 0;
        }
    }

    pub fn shift_down_limbs(&mut self, n: usize) {
        if n >= self.len() {
            self.limbs.clear();
            return;
        }

        let xn = self.len();
        for i in n..xn {
            self.limbs[i - n] = self.limbs[i];
        }
        self.limbs.resize(xn - n, 0);
    }
}
