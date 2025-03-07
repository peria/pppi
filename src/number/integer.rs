mod add_assign;
mod div_assign;
mod format;
mod from;
mod mul_assign;
mod negate;
mod shift_limbs;
mod shr_assign;
mod sub_assign;

use super::Digit;

// Non-negative integer.
pub struct Integer {
    limbs: Vec<Digit>,
}

impl Integer {
    pub fn len(&self) -> usize {
        self.limbs.len()
    }
}
