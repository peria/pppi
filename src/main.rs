mod bigint;
mod binary_fixed;
mod drm;
mod formula;

use formula::{Chudnovsky, PiFormula};

fn main() {
    let digits = std::env::args()
        .nth(1)
        .map(|s| s.parse::<usize>())
        .transpose()
        .expect("digits must be a positive integer")
        .unwrap_or(100);

    let formula = Chudnovsky;
    let value = formula.compute(digits);
    println!("{}", &value);
}

fn compute_pi(digits: usize) -> String {
    // let terms = formula.terms_for_digits(digits);
    // let precision_bits = bits_for_decimal_digits(digits + 12);
    // let split = drm::split(&formula, 0, terms);
    // let sum = split.t_magnitude_when_positive();

    // let reciprocal_sum = binary_fixed::divide_biguints_newton(&split.q, &sum, precision_bits);
    // let inv_sqrt = binary_fixed::Fixed::inv_sqrt_u64(formula.sqrt_u64(), precision_bits);
    // let sqrt = inv_sqrt.mul_u64(formula.sqrt_u64());
    // let value = sqrt.mul_u64(formula.multiplier()).mul(&reciprocal_sum);

    value.to_decimal(digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_first_100_digits() {
        assert_eq!(
            compute_pi(100),
            "3.14159265358979323846264338327950288419716939937510\
             58209749445923078164062862089986280348253421170679"
                .replace(' ', "")
        );
    }
}
