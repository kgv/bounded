#![feature(const_generics, const_panic, const_fn)]
#![allow(incomplete_features)]

#[cfg(feature = "extra-traits")]
use num::{CheckedAdd, CheckedSub, Saturating, Zero};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Deref, DerefMut,
};
use core::fmt::Display;

/// Bounded.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Bounded<T, const MIN: i128, const MAX: i128>(T);

impl<T: Display, const MIN: i128, const MAX: i128> Display for Bounded<T, MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T, const MIN: i128, const MAX: i128> Deref for Bounded<T, MIN, MAX> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, const MIN: i128, const MAX: i128> DerefMut for Bounded<T, MIN, MAX> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

macro_rules! impl_constructor {
    ($t:ty) => {
        impl<const MIN: i128, const MAX: i128> Bounded<$t, MIN, MAX> {
            pub const INVARIANT: () = {
                assert!(MIN < MAX);
                assert!(<$t>::min_value() as i128 >= MIN);
                assert!(<$t>::max_value() as i128 >= MAX);
            };
        
            pub const fn checked_new(value: $t) -> Option<Self> {
                let _ = Self::INVARIANT;
                if (value as i128) < MIN || (value as i128) > MAX {
                    None
                } else {
                    Some(Self(value))
                }
            }
            
            pub const fn new(value: $t) -> Self {
                match Self::checked_new(value) {
                    None => panic!("value out of bounds"),
                    Some(val) => val,
                }
            }
        }
    };
}


impl_constructor!(u8);
impl_constructor!(u16);
impl_constructor!(u32);
impl_constructor!(u64);
impl_constructor!(u128);
impl_constructor!(usize);

impl_constructor!(i8);
impl_constructor!(i16);
impl_constructor!(i32);
impl_constructor!(i64);
impl_constructor!(i128);
impl_constructor!(isize);


macro_rules! impl_traits {
    ($t:ty) => {

        impl<const MIN: i128, const MAX: i128> From<$t> for Bounded<$t, MIN, MAX> {
            fn from(num: $t) -> Self {
                Self::new(num)
            }
        }

        impl<const MIN: i128, const MAX: i128> From<Bounded<$t, MIN, MAX>> for $t {
            fn from(num: Bounded<$t, MIN, MAX>) -> Self {
                num.0
            }
        }

        impl<const MIN: i128, const MAX: i128> Add for Bounded<$t, MIN, MAX> {
            type Output = Self;
        
            fn add(self, rhs: Self) -> Self::Output {
                self + rhs.0
            }
        }

        impl<const MIN: i128, const MAX: i128> Add<$t> for Bounded<$t, MIN, MAX> {
            type Output = Self;
        
            fn add(self, rhs: $t) -> Self::Output {
                Self::new(self.0 + rhs)
            }
        }

        impl<const MIN: i128, const MAX: i128> AddAssign for Bounded<$t, MIN, MAX> {
            fn add_assign(&mut self, rhs: Self) {
                *self += rhs.0;
            }
        }
        
        impl<const MIN: i128, const MAX: i128> AddAssign<$t> for Bounded<$t, MIN, MAX> {
            fn add_assign(&mut self, rhs: $t) {
                *self = *self + rhs;
            }
        }

        #[cfg(feature = "extra-traits")]
        impl<const MIN: i128, const MAX: i128> CheckedAdd for Bounded<$t, MIN, MAX> {
            fn checked_add(&self, rhs: &Self) -> Option<Self::Output> {
                self.0.checked_add(rhs.0).and_then(Self::checked_new)
            }
        }

        #[cfg(feature = "extra-traits")]
        impl<const MIN: i128, const MAX: i128> CheckedSub for Bounded<$t, MIN, MAX> {
            fn checked_sub(&self, rhs: &Self) -> Option<Self::Output> {
                self.0.checked_sub(rhs.0).and_then(Self::checked_new)
            }
        }

        impl<const MIN: i128, const MAX: i128> Div for Bounded<$t, MIN, MAX> {
            type Output = Self;
        
            fn div(self, rhs: Self) -> Self::Output {
                self / rhs.0
            }
        }
        
        impl<const MIN: i128, const MAX: i128> Div<$t> for Bounded<$t, MIN, MAX> {
            type Output = Self;
        
            fn div(self, rhs: $t) -> Self::Output {
                // Needn't validate bounds.
                Self(self.0 / rhs)
            }
        }
        
        impl<const MIN: i128, const MAX: i128> DivAssign for Bounded<$t, MIN, MAX> {
            fn div_assign(&mut self, rhs: Self) {
                *self /= rhs.0;
            }
        }
        
        impl<const MIN: i128, const MAX: i128> DivAssign<$t> for Bounded<$t, MIN, MAX> {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }

        impl<const MIN: i128, const MAX: i128> Mul for Bounded<$t, MIN, MAX> {
            type Output = Self;
        
            fn mul(self, rhs: Self) -> Self::Output {
                self * rhs.0
            }
        }
        
        impl<const MIN: i128, const MAX: i128> Mul<$t> for Bounded<$t, MIN, MAX> {
            type Output = Self;
        
            fn mul(self, rhs: $t) -> Self::Output {
                Self::new(self.0 * rhs)
            }
        }
        
        impl<const MIN: i128, const MAX: i128> MulAssign for Bounded<$t, MIN, MAX> {
            fn mul_assign(&mut self, rhs: Self) {
                *self *= rhs.0;
            }
        }
        
        impl<const MIN: i128, const MAX: i128> MulAssign<$t> for Bounded<$t, MIN, MAX> {
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }


        impl<const MIN: i128, const MAX: i128> PartialEq<$t> for Bounded<$t, MIN, MAX> {
            fn eq(&self, other: &$t) -> bool {
                self.0 == *other
            }
        }

        #[cfg(feature = "extra-traits")]
        impl<const MIN: i128, const MAX: i128> Saturating for Bounded<$t, MIN, MAX> {
            fn saturating_add(self, rhs: Self) -> Self {
                Self::checked_new(self.0.saturating_add(rhs.0)).unwrap_or(Self(MAX as $t))
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                Self::checked_new(self.0.saturating_sub(rhs.0)).unwrap_or(Self(MIN as $t))
            }
        }

        impl<const MIN: i128, const MAX: i128> Sub for Bounded<$t, MIN, MAX> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self - rhs.0
            }
        }

        impl<const MIN: i128, const MAX: i128> Sub<$t> for Bounded<$t, MIN, MAX> {
            type Output = Self;

            fn sub(self, rhs: $t) -> Self::Output {
                Self::new(self.0 - rhs)
            }
        }

        impl<const MIN: i128, const MAX: i128> SubAssign for Bounded<$t, MIN, MAX> {
            fn sub_assign(&mut self, rhs: Self) {
                *self -= rhs.0;
            }
        }

        impl<const MIN: i128, const MAX: i128> SubAssign<$t> for Bounded<$t, MIN, MAX> {
            fn sub_assign(&mut self, rhs: $t) {
                *self = *self - rhs;
            }
        }

        #[cfg(feature = "extra-traits")]
        impl<const MIN: i128, const MAX: i128> Zero for Bounded<$t, MIN, MAX> {
            fn zero() -> Self {
                Self(0)
            }

            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
        }

    };
}

impl_traits!(u8);
impl_traits!(u16);
impl_traits!(u32);
impl_traits!(u64);
impl_traits!(u128);
impl_traits!(usize);

impl_traits!(i8);
impl_traits!(i16);
impl_traits!(i32);
impl_traits!(i64);
impl_traits!(i128);
impl_traits!(isize);

#[cfg(test)]
mod tests {
    use super::Bounded;

    type ZeroToTen = Bounded<u8, 0, 9>;

    mod ok {
        use super::ZeroToTen;
        #[cfg(feature = "extra-traits")]
        use num::{CheckedAdd, CheckedSub, Saturating};

        #[test]
        fn add() {
            assert_eq!(ZeroToTen::new(9), ZeroToTen::new(8) + ZeroToTen::new(1));
        }

        #[test]
        fn add_assign() {
            let mut a = ZeroToTen::new(8);
            a += ZeroToTen::new(1);
            assert_eq!(ZeroToTen::new(9), a);
        }

        #[test]
        fn div() {
            assert_eq!(ZeroToTen::new(0), ZeroToTen::new(0) / ZeroToTen::new(1));
        }

        #[test]
        fn div_assign() {
            let mut a = ZeroToTen::new(0);
            a /= ZeroToTen::new(1);
            assert_eq!(ZeroToTen::new(0), a);
        }

        #[cfg(feature = "extra-traits")]
        #[test]
        fn checked_add() {
            assert_eq!(
                Some(ZeroToTen::new(9)),
                ZeroToTen::new(8).checked_add(&ZeroToTen::new(1))
            );
            assert_eq!(None, ZeroToTen::new(9).checked_add(&ZeroToTen::new(1)));
        }

        #[cfg(feature = "extra-traits")]
        #[test]
        fn checked_sub() {
            assert_eq!(
                Some(ZeroToTen::new(0)),
                ZeroToTen::new(1).checked_sub(&ZeroToTen::new(1))
            );
            assert_eq!(None, ZeroToTen::new(0).checked_sub(&ZeroToTen::new(1)));
        }

        #[test]
        fn mul() {
            assert_eq!(ZeroToTen::new(9), ZeroToTen::new(9) * ZeroToTen::new(1));
        }

        #[test]
        fn mul_assign() {
            let mut a = ZeroToTen::new(9);
            a *= ZeroToTen::new(1);
            assert_eq!(ZeroToTen::new(9), a);
        }

        #[test]
        fn new() {
            assert_eq!(1, *crate::Bounded::<u8, 0, 2>::new(1));
        }

        #[cfg(feature = "extra-traits")]
        #[test]
        fn saturating_add() {
            assert_eq!(
                ZeroToTen::new(9),
                ZeroToTen::new(9).saturating_add(ZeroToTen::new(1))
            );
        }

        #[cfg(feature = "extra-traits")]
        #[test]
        fn saturating_sub() {
            assert_eq!(
                ZeroToTen::new(0),
                ZeroToTen::new(0).saturating_sub(ZeroToTen::new(1))
            );
        }

        #[test]
        fn sub() {
            assert_eq!(ZeroToTen::new(0), ZeroToTen::new(1) - ZeroToTen::new(1));
        }

        #[test]
        fn sub_assign() {
            let mut a = ZeroToTen::new(1);
            a -= ZeroToTen::new(1);
            assert_eq!(ZeroToTen::new(0), a);
        }
    }

    mod panic {
        use super::ZeroToTen;

        #[test]
        #[should_panic]
        fn add() {
            let _ = ZeroToTen::new(9) + ZeroToTen::new(1);
        }

        #[test]
        #[should_panic]
        fn add_assign() {
            let mut a = ZeroToTen::new(9);
            a += ZeroToTen::new(1);
        }

        #[test]
        #[should_panic]
        fn div() {
            let _ = ZeroToTen::new(9) / ZeroToTen::new(0);
        }

        #[test]
        #[should_panic]
        fn div_assign() {
            let mut a = ZeroToTen::new(9);
            a /= ZeroToTen::new(0);
        }

        #[test]
        #[should_panic]
        fn mul() {
            let _ = ZeroToTen::new(5) * ZeroToTen::new(2);
        }

        #[test]
        #[should_panic]
        fn mul_assign() {
            let mut a = ZeroToTen::new(5);
            a *= ZeroToTen::new(2);
        }

        #[test]
        #[should_panic]
        fn new() {
            let _ = ZeroToTen::new(10);
        }

        #[test]
        #[should_panic]
        fn sub() {
            let _ = ZeroToTen::new(0) - ZeroToTen::new(1);
        }

        #[test]
        #[should_panic]
        fn sub_assign() {
            let mut a = ZeroToTen::new(0);
            a -= ZeroToTen::new(1);
        }
    }
}
