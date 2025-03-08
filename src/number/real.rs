mod format;
mod mul_assign;
mod square_root;

use super::integer::Integer;

// Multi-precision floating number.
//  Real = mantissa * base^(-point)  where base = 2^64
pub struct Real {
    mantissa: Integer,
    point: usize,
}
