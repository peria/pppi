use super::Integer;
use super::Real;

impl From<Integer> for Real {
    fn from(value: Integer) -> Self {
        Real {
            mantissa: value,
            point: 0,
        }
    }
}
