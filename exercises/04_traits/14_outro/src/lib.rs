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

use std::ops::{Add, Deref};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16
}

impl From<u16> for SaturatingU16 {
    fn from(s: u16) -> SaturatingU16 {
        SaturatingU16 { value: s }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(s: u8) -> SaturatingU16 {
        SaturatingU16 { value: u16::from(s) }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(s: &u16) -> SaturatingU16 {
        SaturatingU16::from(*s)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(s: &u8) -> SaturatingU16 {
        SaturatingU16::from(u16::from(*s))
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: SaturatingU16) -> Self::Output {
        SaturatingU16::from(self.value.saturating_add(other.value))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, other: u16) -> Self::Output {
        SaturatingU16::from(self.value.saturating_add(other as u16))
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> Self::Output {
        SaturatingU16::from(self.value.saturating_add(other.value))
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl Deref for SaturatingU16 {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.value
    }
}