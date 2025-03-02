use std::fmt::{Display, UpperHex};

use super::Integer;

impl UpperHex for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let leading = self.limbs.last().unwrap_or(&0);
        write!(f, "{:X}", leading)?;

        let n = self.len().max(1);
        for limb in self.limbs[..(n - 1)].iter().rev() {
            write!(f, "{:016X}", limb)?;
        }

        Ok(())
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let leading = self.limbs.last().unwrap_or(&0);
        write!(f, "{}", leading)?;

        let n = self.len().max(1);
        for limb in self.limbs[..(n - 1)].iter().rev() {
            write!(f, "{:019}", limb)?;
        }

        Ok(())
    }
}
