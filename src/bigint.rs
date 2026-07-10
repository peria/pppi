use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUint {
    limbs: Vec<u32>,
}

impl BigUint {
    pub fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    pub fn one() -> Self {
        Self::from_u64(1)
    }

    pub fn from_u64(mut n: u64) -> Self {
        let mut limbs = Vec::new();
        while n != 0 {
            limbs.push(n as u32);
            n >>= 32;
        }
        Self { limbs }
    }

    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    pub fn bits(&self) -> usize {
        match self.limbs.last() {
            Some(&top) => (self.limbs.len() - 1) * 32 + (32 - top.leading_zeros() as usize),
            None => 0,
        }
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let len = self.limbs.len().max(rhs.limbs.len());
        let mut out = Vec::with_capacity(len + 1);
        let mut carry = 0u64;

        for i in 0..len {
            let a = self.limbs.get(i).copied().unwrap_or(0) as u64;
            let b = rhs.limbs.get(i).copied().unwrap_or(0) as u64;
            let s = a + b + carry;
            out.push(s as u32);
            carry = s >> 32;
        }

        if carry != 0 {
            out.push(carry as u32);
        }

        Self { limbs: out }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        assert!(self >= rhs);

        let mut out = Vec::with_capacity(self.limbs.len());
        let mut borrow = 0i64;

        for i in 0..self.limbs.len() {
            let a = self.limbs[i] as i64;
            let b = rhs.limbs.get(i).copied().unwrap_or(0) as i64;
            let mut d = a - b - borrow;

            if d < 0 {
                d += 1i64 << 32;
                borrow = 1;
            } else {
                borrow = 0;
            }

            out.push(d as u32);
        }

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }

        let mut out = vec![0u32; self.limbs.len() + rhs.limbs.len()];

        for (i, &a) in self.limbs.iter().enumerate() {
            let mut carry = 0u64;

            for (j, &b) in rhs.limbs.iter().enumerate() {
                let k = i + j;
                let v = out[k] as u64 + (a as u64) * (b as u64) + carry;
                out[k] = v as u32;
                carry = v >> 32;
            }

            let mut k = i + rhs.limbs.len();
            while carry != 0 {
                if k == out.len() {
                    out.push(0);
                }

                let v = out[k] as u64 + carry;
                out[k] = v as u32;
                carry = v >> 32;
                k += 1;
            }
        }

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    pub fn mul_u64(&self, rhs: u64) -> Self {
        self.mul(&Self::from_u64(rhs))
    }

    pub fn shl_bits(&self, bits: usize) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        let limb_shift = bits / 32;
        let bit_shift = bits % 32;
        let mut out = vec![0u32; limb_shift + self.limbs.len() + 1];
        let mut carry = 0u64;

        for (i, &limb) in self.limbs.iter().enumerate() {
            let v = ((limb as u64) << bit_shift) | carry;
            out[i + limb_shift] = v as u32;
            carry = v >> 32;
        }

        if carry != 0 {
            out[limb_shift + self.limbs.len()] = carry as u32;
        }

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    pub fn shr_bits(&self, bits: usize) -> Self {
        let limb_shift = bits / 32;
        let bit_shift = bits % 32;

        if limb_shift >= self.limbs.len() {
            return Self::zero();
        }

        let mut out = Vec::with_capacity(self.limbs.len() - limb_shift);
        let mut carry = 0u32;

        for &limb in self.limbs[limb_shift..].iter().rev() {
            let next = if bit_shift == 0 {
                limb
            } else {
                (limb >> bit_shift) | (carry << (32 - bit_shift))
            };

            carry = if bit_shift == 0 {
                0
            } else {
                limb & ((1u32 << bit_shift) - 1)
            };

            out.push(next);
        }

        out.reverse();

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    pub fn div_u64(&self, rhs: u64) -> Self {
        assert!(rhs != 0);

        let mut out = vec![0u32; self.limbs.len()];
        let mut rem = 0u128;

        for i in (0..self.limbs.len()).rev() {
            let cur = (rem << 32) | self.limbs[i] as u128;
            out[i] = (cur / rhs as u128) as u32;
            rem = cur % rhs as u128;
        }

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    pub fn top_bits_u64(&self, count: usize) -> u64 {
        assert!(count <= 64);

        let bits = self.bits();
        let shifted = if bits > count {
            self.shr_bits(bits - count)
        } else {
            self.clone()
        };

        shifted.to_u64()
    }

    pub fn to_u64(&self) -> u64 {
        let mut out = 0u64;

        for (i, &limb) in self.limbs.iter().take(2).enumerate() {
            out |= (limb as u64) << (i * 32);
        }

        out
    }

    pub fn to_decimal_string(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let mut n = self.clone();
        let base = 1_000_000_000u32;
        let mut parts = Vec::new();

        while !n.is_zero() {
            let (q, r) = n.div_rem_u32(base);
            parts.push(r);
            n = q;
        }

        let mut s = parts.pop().unwrap().to_string();

        for part in parts.iter().rev() {
            s.push_str(&format!("{part:09}"));
        }

        s
    }

    fn div_rem_u32(&self, rhs: u32) -> (Self, u32) {
        assert!(rhs != 0);

        let mut out = vec![0u32; self.limbs.len()];
        let mut rem = 0u64;

        for i in (0..self.limbs.len()).rev() {
            let cur = (rem << 32) | self.limbs[i] as u64;
            out[i] = (cur / rhs as u64) as u32;
            rem = cur % rhs as u64;
        }

        let mut q = Self { limbs: out };
        q.normalize();

        (q, rem as u32)
    }

    fn normalize(&mut self) {
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }
}

impl Ord for BigUint {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.limbs.len().cmp(&rhs.limbs.len()) {
            Ordering::Equal => self.limbs.iter().rev().cmp(rhs.limbs.iter().rev()),
            other => other,
        }
    }
}

impl PartialOrd for BigUint {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
