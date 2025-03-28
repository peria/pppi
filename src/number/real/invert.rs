use crate::number::BASE_F64;
use crate::number::Digit;
use crate::number::integer::Integer;
use crate::number::real::Real;

impl Real {
    pub fn invert(&mut self, limbs: usize) {
        let mut bits = limbs * 64;
        let mut lens = Vec::new();
        while bits > 40 {
            let len = (bits + 127) / 64;
            lens.push(len.min(limbs));
            bits = (bits + 1) / 2;
        }

        let a = &self.mantissa;
        let mut x = initialize_inversion(a);
        for n in lens.iter().rev() {
            let m = x.len();
            let mut t = a.get_leading(*n);
            t *= &x;
            //      <-m -> <----n ---->
            // t: 1.000000 xxxxx xxxxxx or
            //    0.FFFFFF xxxxx xxxxxx
            let is_over = t.limbs.last().unwrap() == &1;
            if is_over {
                // Remove the leading 1.00...0.
                let mut k = t.len() - 1;
                while k > 0 && t.limbs[k - 1] == 0 {
                    k -= 1;
                }
                t.limbs.resize(k, 0);
            } else {
                t.negate();
            }
            //      <-m -> <----n ---->
            // t: 0.000000 xxxxxx xxxxx
            t.shift_down_limbs(m);
            //      <---- n ---->
            // t: 0.000000 xxxxxx .....
            t *= &x;
            //      <---- n ---->
            // t: 0.xxxxxx xxxxxx
            x.shift_up_limbs(n - m);
            t.shift_down_limbs(m - 1);
            //    <- m ->
            // x: xxxxxxx .....
            //    <-----n----->
            // t:       0.xxxxxxx xxxx
            //            <----n ---->
            if is_over {
                x -= &t;
            } else {
                x += &t;
            }
        }

        let point = a.len() + limbs - 1;
        self.mantissa = x;
        self.point = point;
    }
}

fn initialize_inversion(a: &Integer) -> Integer {
    let mut x = a.get_leading_as_f64(2);
    x = BASE_F64 * BASE_F64 / x;
    let x0 = x as Digit;
    let x1 = ((x - x0 as f64) * BASE_F64) as Digit;
    Integer {
        limbs: vec![x1, x0],
    }
}

#[cfg(test)]
mod test {
    use crate::number::integer::Integer;
    use crate::number::real::Real;

    #[test]
    fn test_invert() {
        let mut t = 12384348u64;
        for n in [100, 500, 1000] {
            let mut a = vec![0];
            for _ in 0..n {
                a.push(t);
                t = xorshift64_next(t);
            }
            let a = Integer::from(a);
            let mut x = Real::from(a.clone());
            x.invert(n);
            x.mantissa *= &a;
            x.mantissa.shift_down_limbs(x.mantissa.len() - n + 1);
            eprintln!("n = {}", n);
            assert!(x.mantissa.limbs[0] >= (!0u64) - 1);
        }
    }

    fn xorshift64_next(mut x: u64) -> u64 {
        x ^= x << 7;
        x ^= x >> 9;
        x
    }
}
