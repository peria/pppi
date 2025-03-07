use std::fmt::{Display, UpperHex};

use super::Real;

impl UpperHex for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = &self.mantissa;
        let point = &self.point;

        let mut n = x.len();
        if x.len() <= *point {
            write!(f, "0.")?;
        } else {
            // TODO: Do not assume the integral part's length is 1.
            write!(f, "{:X}.", x.limbs.last().unwrap())?;
            n -= 1;
        }

        for limb in x.limbs[..(n - 1)].iter().rev() {
            write!(f, "{:016X}", limb)?;
        }

        Ok(())
    }
}

impl Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = &self.mantissa;
        let point = &self.point;

        let mut n = x.len();
        if x.len() <= *point {
            write!(f, "0.")?;
        } else {
            // TODO: Do not assume the integral part's length is 1.
            write!(f, "{}.", x.limbs.last().unwrap())?;
            n -= 1;
        }

        for limb in x.limbs[..(n - 1)].iter().rev() {
            write!(f, "{:019}", limb)?;
        }

        Ok(())
    }
}
