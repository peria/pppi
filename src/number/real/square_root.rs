use super::Real;
use crate::number::{BASE_F64, integer::Integer};

impl Real {
    pub fn square_root(a: u64, limbs: usize) -> Self {
        let mut x = Self::inverse_square_root(a, limbs);
        x.mantissa *= a;
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
            //      <--m--> <--m-->
            // t: 0.FFFFFFx xxxxxxx
            t.negate();
            //      <--m--> <--m-->
            // t: 0.000000x xxxxxxx
            t *= &x;
            //      <--m--> <--m--> <--m-->
            // t: 0.000000x xxxxxxx xxxxxxx
            t >>= 1;
            //      <--m--> <--m--> <--m-->
            // t: 0.000000x xxxxxxx xxxxxxx
            // x: 0.xxxxxxx
            //      <-----n------>
            x.shift_up_limbs(n - m);
            t.shift_down_limbs(3 * m - n);
            x += &t;
        }

        let point = x.len();
        Real { mantissa: x, point }
    }

    fn initialize_inverse_square_root(a: u64) -> Integer {
        let x = BASE_F64 * BASE_F64 / (a as f64).sqrt();
        Integer::from(x)
    }
}

#[cfg(test)]
mod test {
    use super::Real;

    #[test]
    fn test_inverse_square_root() {
        let a = 2;
        let x = Real::inverse_square_root(a, 10);
        // 0x0.b504f333f9de6484...
        eprintln!("1/sqrt(2) = {:X}", &x);
        assert_eq!(x.mantissa.len(), 10);
    }
}
