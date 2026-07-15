use std::cmp::Ordering;
use std::ops::{Add, Mul, Shl, Shr, Sub};

pub type Limb = u64;

const LIMB_BITS: usize = Limb::BITS as usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    limbs: Vec<Limb>,
}

impl Integer {
    pub fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    pub fn one() -> Self {
        Self::from(1)
    }

    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    pub fn bits(&self) -> usize {
        match self.limbs.last() {
            Some(&top) => {
                (self.limbs.len() - 1) * LIMB_BITS + (LIMB_BITS - top.leading_zeros() as usize)
            }
            None => 0,
        }
    }

    pub fn mul_u64(&self, rhs: u64) -> Self {
        self * &Self::from(rhs)
    }

    pub fn div_u64(&self, rhs: u64) -> Self {
        assert!(rhs != 0);

        let mut out = vec![0 as Limb; self.limbs.len()];
        let mut rem = 0u128;

        for i in (0..self.limbs.len()).rev() {
            let cur = (rem << LIMB_BITS) | self.limbs[i] as u128;
            out[i] = (cur / rhs as u128) as Limb;
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
            self >> (bits - count)
        } else {
            self.clone()
        };

        shifted.to_u64()
    }

    pub fn to_u64(&self) -> u64 {
        self.limbs.first().copied().unwrap_or(0)
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

    fn add_ref(&self, rhs: &Self) -> Self {
        let len = self.limbs.len().max(rhs.limbs.len());
        let mut out = Vec::with_capacity(len + 1);
        let mut carry = 0u128;

        for i in 0..len {
            let a = self.limbs.get(i).copied().unwrap_or(0) as u128;
            let b = rhs.limbs.get(i).copied().unwrap_or(0) as u128;
            let s = a + b + carry;
            out.push(s as Limb);
            carry = s >> LIMB_BITS;
        }

        if carry != 0 {
            out.push(carry as Limb);
        }

        Self { limbs: out }
    }

    fn sub_ref(&self, rhs: &Self) -> Self {
        assert!(self >= rhs);

        let mut out = Vec::with_capacity(self.limbs.len());
        let mut borrow = 0 as Limb;

        for i in 0..self.limbs.len() {
            let a = self.limbs[i];
            let b = rhs.limbs.get(i).copied().unwrap_or(0);
            let (d1, b1) = a.overflowing_sub(b);
            let (d2, b2) = d1.overflowing_sub(borrow);
            out.push(d2);
            borrow = (b1 || b2) as Limb;
        }

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    fn mul_ref(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }

        let mut out = vec![0 as Limb; self.limbs.len() + rhs.limbs.len()];

        for (i, &a) in self.limbs.iter().enumerate() {
            let mut carry = 0u128;

            for (j, &b) in rhs.limbs.iter().enumerate() {
                let k = i + j;
                let v = out[k] as u128 + (a as u128) * (b as u128) + carry;
                out[k] = v as Limb;
                carry = v >> LIMB_BITS;
            }

            let mut k = i + rhs.limbs.len();
            while carry != 0 {
                if k == out.len() {
                    out.push(0);
                }

                let v = out[k] as u128 + carry;
                out[k] = v as Limb;
                carry = v >> LIMB_BITS;
                k += 1;
            }
        }

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    fn shl_ref(&self, bits: usize) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        let limb_shift = bits / LIMB_BITS;
        let bit_shift = bits % LIMB_BITS;
        let mut out = vec![0 as Limb; limb_shift + self.limbs.len() + 1];

        if bit_shift == 0 {
            out[limb_shift..limb_shift + self.limbs.len()].copy_from_slice(&self.limbs);
        } else {
            let mut carry = 0u128;

            for (i, &limb) in self.limbs.iter().enumerate() {
                let v = ((limb as u128) << bit_shift) | carry;
                out[i + limb_shift] = v as Limb;
                carry = v >> LIMB_BITS;
            }

            if carry != 0 {
                out[limb_shift + self.limbs.len()] = carry as Limb;
            }
        }

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    fn shr_ref(&self, bits: usize) -> Self {
        let limb_shift = bits / LIMB_BITS;
        let bit_shift = bits % LIMB_BITS;

        if limb_shift >= self.limbs.len() {
            return Self::zero();
        }

        let mut out = Vec::with_capacity(self.limbs.len() - limb_shift);

        if bit_shift == 0 {
            out.extend_from_slice(&self.limbs[limb_shift..]);
        } else {
            let mut carry = 0 as Limb;

            for &limb in self.limbs[limb_shift..].iter().rev() {
                let next = (limb >> bit_shift) | (carry << (LIMB_BITS - bit_shift));
                carry = limb & (((1 as Limb) << bit_shift) - 1);
                out.push(next);
            }

            out.reverse();
        }

        let mut result = Self { limbs: out };
        result.normalize();
        result
    }

    fn div_rem_u32(&self, rhs: u32) -> (Self, u32) {
        assert!(rhs != 0);

        let mut out = vec![0 as Limb; self.limbs.len()];
        let mut rem = 0u128;

        for i in (0..self.limbs.len()).rev() {
            let cur = (rem << LIMB_BITS) | self.limbs[i] as u128;
            out[i] = (cur / rhs as u128) as Limb;
            rem = cur % rhs as u128;
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

impl Add<&Integer> for Integer {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        self.add_ref(rhs)
    }
}

impl Sub<&Integer> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        self.sub_ref(rhs)
    }
}

impl Sub<&Integer> for Integer {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        self.sub_ref(rhs)
    }
}

impl Sub<Integer> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

impl Sub for Integer {
    type Output = Integer;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

impl Mul<&Integer> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        self.mul_ref(rhs)
    }
}

impl Mul<&Integer> for Integer {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        self.mul_ref(rhs)
    }
}

impl Mul<Integer> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: Integer) -> Self::Output {
        self.mul_ref(&rhs)
    }
}

impl Mul for Integer {
    type Output = Integer;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_ref(&rhs)
    }
}

impl Shl<usize> for &Integer {
    type Output = Integer;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shl_ref(rhs)
    }
}

impl Shl<usize> for Integer {
    type Output = Integer;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shl_ref(rhs)
    }
}

impl Shr<usize> for &Integer {
    type Output = Integer;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shr_ref(rhs)
    }
}

impl Shr<usize> for Integer {
    type Output = Integer;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shr_ref(rhs)
    }
}

impl Ord for Integer {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.limbs.len().cmp(&rhs.limbs.len()) {
            Ordering::Equal => self.limbs.iter().rev().cmp(rhs.limbs.iter().rev()),
            other => other,
        }
    }
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl From<u64> for Integer {
    fn from(n: u64) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            Self { limbs: vec![n] }
        }
    }
}
