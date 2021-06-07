#![feature(const_generics)]
#![feature(const_panic)]

#[cfg(feature = "saturating-div")]
pub use saturating_div::SaturatingDiv;

use core::fmt::{self, Display, Formatter};
use core::ops::{Deref, DerefMut};

/// Integer bounded by closed interval
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Bounded<T, const MIN: i128, const MAX: u128>(T);

r#impl!(Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(Add for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(AddAssign for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(From for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(Div for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(DivAssign for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(Mul for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(MulAssign for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(Neg for Bounded<i8, i16, i32, i64, i128, isize>);

r#impl!(RangeBounds for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(Rem for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(RemAssign for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(Shl for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(ShlAssign for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(Shr for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(ShrAssign for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(Sub for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

r#impl!(SubAssign for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "bounded")]
r#impl!(Bounded for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "checked-add")]
r#impl!(CheckedAdd for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "checked-div")]
r#impl!(CheckedDiv for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "checked-mul")]
r#impl!(CheckedMul for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "checked-neg")]
r#impl!(CheckedNeg for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "checked-rem")]
r#impl!(CheckedRem for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "checked-shl")]
r#impl!(CheckedShl for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "checked-shr")]
r#impl!(CheckedShr for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "checked-sub")]
r#impl!(CheckedSub for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "saturating-add")]
r#impl!(SaturatingAdd for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "saturating-div")]
r#impl!(SaturatingDiv for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "saturating-mul")]
r#impl!(SaturatingMul for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

#[cfg(feature = "saturating-sub")]
r#impl!(SaturatingSub for Bounded<i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize>);

impl<T, const MIN: i128, const MAX: u128> Deref for Bounded<T, MIN, MAX> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, const MIN: i128, const MAX: u128> DerefMut for Bounded<T, MIN, MAX> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Display, const MIN: i128, const MAX: u128> Display for Bounded<T, MIN, MAX> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

mod macros;
#[cfg(feature = "saturating-div")]
mod saturating_div;

#[cfg(test)]
mod tests;
