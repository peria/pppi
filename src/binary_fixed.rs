use crate::bigint::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixed {
    value: Integer,
    precision_bits: usize,
}

impl Fixed {
    pub fn inv_sqrt_u64(n: u64, precision_bits: usize) -> Self {
        assert!(n > 0);
        let mut x = Self::from_f64(1.0 / (n as f64).sqrt(), precision_bits);
        let three = Self::from_u64(3, precision_bits);

        for _ in 0..newton_iterations(precision_bits) {
            let nx2 = x.mul(&x).mul_u64(n);
            x = x.mul(&three.sub(&nx2)).shr1();
        }

        x
    }

    pub fn divide(mut numerator: Integer, denominator: Integer, precision_bits: usize) -> Fixed {
        let m = precision_bits + denominator.bits() + 8;
        let reciprocal = reciprocal_newton(denominator, m);
        numerator *= &reciprocal;
        Fixed {
            value: numerator,
            precision_bits,
        }
    }

    pub fn to_decimal(&self, digits: usize) -> String {
        let scale10 = pow10(digits);
        let truncated = (&self.value * &scale10) >> self.precision_bits;
        let mut s = truncated.to_decimal_string();

        if digits == 0 {
            return s;
        }

        if s.len() <= digits {
            s = format!("{}{}", "0".repeat(digits + 1 - s.len()), s);
        }

        let dot = s.len() - digits;
        format!("{}.{}", &s[..dot], &s[dot..])
    }
}

fn reciprocal_newton(d: Integer, scale_bits: usize) -> Integer {
    let mut x = reciprocal_seed(&d, scale_bits);

    let mut scale_limbs = Vec::new();

    for _ in 0..newton_iterations(scale_bits) {
        let dx = d * &x;
        let correction = &two_b - &dx;
        x = (&x * &correction) >> scale_bits;
        if x.is_zero() {
            x = Integer::one();
        }
    }

    x
}

fn reciprocal_seed(d: &Integer, scale_bits: usize) -> Integer {
    let bits = d.bits();
    let top_bits = bits.min(53);
    let top = d.top_bits_u64(top_bits);
    let numerator_shift = scale_bits + top_bits - bits;

    if numerator_shift < 63 {
        Integer::from(((1u128 << numerator_shift) / top as u128) as u64)
    } else {
        (Integer::one() << numerator_shift).div_u64(top)
    }
}

fn pow10(digits: usize) -> Integer {
    let mut value = Integer::one();
    for _ in 0..digits {
        value = value.mul_u64(10);
    }
    value
}

fn newton_iterations(bits: usize) -> usize {
    let mut precision = 48usize;
    let mut iterations = 2usize;
    while precision < bits + 8 {
        precision *= 2;
        iterations += 1;
    }
    iterations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_with_newton() {
        let q = divide_biguints_newton(&Integer::one(), &Integer::from(7), 256);
        assert!(q
            .to_decimal(40)
            .starts_with("0.1428571428571428571428571428571428571428"));
    }

    #[test]
    fn inverse_square_root_is_accurate() {
        let inv = Fixed::inv_sqrt_u64(10_005, 256);
        assert!(inv.to_decimal(50).starts_with("0.009997500937109545"));
    }
}
