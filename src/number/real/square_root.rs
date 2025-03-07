use super::Real;
use crate::number::{BASE_F64, integer::Integer};

impl Real {
    pub fn square_root(a: u64, limbs: usize) -> Self {
        let mut x = Self::inverse_square_root(a, limbs);
        x *= a;
        x
    }

    pub fn inverse_square_root(a: u64, limbs: usize) -> Self {
        let mut bits = limbs * 64;
        let mut lens = Vec::new();
        while bits > 40 {
            let len = (bits + 127) / 64;
            lens.push(len);
            bits = (bits + 1) / 2;
        }

        let mut x = Self::initialize_inverse_square_root(a);
        for n in lens.iter().rev() {
            let m = x.len();
            let mut t = x.square();
            t *= a;
            t.negate();
            t *= &x;
            t >>= 1;
            x.shift_up_limbs(0);
            t.shift_down_limbs(0);
            x += &t;
        }
        Real {
            mantissa: Integer::from(0),
            point: 0,
        }
    }

    fn initialize_inverse_square_root(a: u64) -> Integer {
        let x = BASE_F64 * BASE_F64 / (a as f64).sqrt();
        Integer::from(x)
    }
}
