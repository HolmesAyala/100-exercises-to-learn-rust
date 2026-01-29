// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    fn add_saturating(&self, saturating_u16: &SaturatingU16) -> SaturatingU16 {
        if saturating_u16.value > u16::MAX - self.value {
            return SaturatingU16 { value: u16::MAX };
        }

        SaturatingU16 {
            value: &self.value + saturating_u16.value,
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, &other: &u16) -> bool {
        self.value == other
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> SaturatingU16 {
        SaturatingU16 { value }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(&value: &u16) -> SaturatingU16 {
        SaturatingU16 { value: value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> SaturatingU16 {
        SaturatingU16 {
            value: value.into(),
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(&value: &u8) -> SaturatingU16 {
        SaturatingU16 {
            value: value.into(),
        }
    }
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self {
        self.add_saturating(&rhs)
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self {
        self.add_saturating(rhs)
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self {
        let value: SaturatingU16 = rhs.into();

        self.add_saturating(&value)
    }
}
