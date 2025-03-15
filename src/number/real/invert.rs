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
            lens.push(len);
            bits = (bits + 1) / 2;
        }

        let a = &self.mantissa;
        let mut x = initialize_inversion(a);
        eprintln!("{}> x: {:X}", line!(), &x);
        for n in lens.iter().rev() {
            let m = x.len();
            let mut t = a.get_leading(*n);
            eprintln!("{}> t: {:X}", line!(), &t);
            t *= &x;
            eprintln!("{}> t: {:X}", line!(), &t);
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
            eprintln!("{}> t: {:X}", line!(), &t);
            //      <-m -> <----n ---->
            // t: 0.000000 xxxxx xxxxxx
            t *= &x;
            eprintln!("{}> t: {:X}", line!(), &t);
            //      <-m -> <----n ---->
            // t: 0.xxxxxx xxxxx xxxxxx
            x.shift_up_limbs(n - m);
            eprintln!("{}> x: {:X}", line!(), &x);
            t.shift_down_limbs(2 * m);
            eprintln!("{}> t: {:X}", line!(), &t);
            //    <-m -> <-m -> <----n ---->
            // x: xxxxxx
            // t:      0.xxxxxx xxxxx xxxxxx
            //    <-----n---->
            if is_over {
                x -= &t;
            } else {
                x += &t;
            }
            eprintln!("{}> x: {:X}", line!(), &x);
        }

        let point = a.len() + limbs;
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
    use crate::number::Digit;
    use crate::number::integer::Integer;
    use crate::number::real::Real;

    const ALL5: Digit = 0x5555_5555_5555_5555;

    #[test]
    fn test_invert() {
        let limbs = 20;
        let a = Integer {
            limbs: vec![
                0xbee7_8cf5_02ad_8203,
                0x4768_c384_530d_afe3,
                0x1a04_5f01_7850_1e83,
                0x47cb_fe52_3758_1bfa,
                0xab18_793a_efea_cdfa,
                0x1768_3241_0235_67ae,
                0xcfbe_cfbc_bea8_fbac,
                0xf451_0374_6923_fbeb,
                0xfa50_bfed_fbac_cbfe,
                0x1a04_5f01_7850_1e83,
                0x47cb_fe52_3758_1bfa,
                0xab18_793a_efea_cdfa,
                0xcd74_9576_10ef_b412,
                0x3238_567b_caec_afeb,
                0x4587_123a_ceff_eccd,
                0xfb01_1082_3774_8061,
            ],
        };
        let mut x = Real::from(a);
        x.invert(limbs);
        eprintln!("{:X}", &x);
        assert!(false);
    }
}
