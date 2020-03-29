#![feature(bool_to_option)]
#![feature(const_generics)]

use derive_more::{Deref, DerefMut, Display, From, Into};
#[cfg(feature = "extra-traits")]
use num::{CheckedAdd, CheckedSub, Saturating, Zero};
use std::ops::{
    Add, AddAssign, Bound, Div, DivAssign, Mul, MulAssign, RangeBounds, Sub, SubAssign,
};

/// Bounded.
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, Display, Eq, From, Into, PartialEq)]
#[display(fmt = "{}", _0)]
pub struct Bounded<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> Bounded<MIN, MAX> {
    pub fn checked_new(value: usize) -> Option<Self> {
        (MIN..=MAX).contains(&value).then_some(Self(value))
    }

    pub fn new(value: usize) -> Self {
        assert!((MIN..=MAX).contains(&value));
        Self(value)
    }
}

impl<const MIN: usize, const MAX: usize> Add for Bounded<MIN, MAX> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.0
    }
}

impl<const MIN: usize, const MAX: usize> Add<usize> for Bounded<MIN, MAX> {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl<const MIN: usize, const MAX: usize> AddAssign for Bounded<MIN, MAX> {
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.0;
    }
}

impl<const MIN: usize, const MAX: usize> AddAssign<usize> for Bounded<MIN, MAX> {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

#[cfg(feature = "extra-traits")]
impl<const MIN: usize, const MAX: usize> CheckedAdd for Bounded<MIN, MAX> {
    fn checked_add(&self, rhs: &Self) -> Option<Self::Output> {
        self.0.checked_add(rhs.0).and_then(Self::checked_new)
    }
}

#[cfg(feature = "extra-traits")]
impl<const MIN: usize, const MAX: usize> CheckedSub for Bounded<MIN, MAX> {
    fn checked_sub(&self, rhs: &Self) -> Option<Self::Output> {
        self.0.checked_sub(rhs.0).and_then(Self::checked_new)
    }
}

impl<const MIN: usize, const MAX: usize> Div for Bounded<MIN, MAX> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self / rhs.0
    }
}

impl<const MIN: usize, const MAX: usize> Div<usize> for Bounded<MIN, MAX> {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        // Needn't validate bounds.
        Self(self.0 / rhs)
    }
}

impl<const MIN: usize, const MAX: usize> DivAssign for Bounded<MIN, MAX> {
    fn div_assign(&mut self, rhs: Self) {
        *self /= rhs.0;
    }
}

impl<const MIN: usize, const MAX: usize> DivAssign<usize> for Bounded<MIN, MAX> {
    fn div_assign(&mut self, rhs: usize) {
        *self = *self / rhs;
    }
}

impl<const MIN: usize, const MAX: usize> Mul for Bounded<MIN, MAX> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs.0
    }
}

impl<const MIN: usize, const MAX: usize> Mul<usize> for Bounded<MIN, MAX> {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.0 * rhs)
    }
}

impl<const MIN: usize, const MAX: usize> MulAssign for Bounded<MIN, MAX> {
    fn mul_assign(&mut self, rhs: Self) {
        *self *= rhs.0;
    }
}

impl<const MIN: usize, const MAX: usize> MulAssign<usize> for Bounded<MIN, MAX> {
    fn mul_assign(&mut self, rhs: usize) {
        *self = *self * rhs;
    }
}

// impl<const LHS_MIN: usize, const LHS_MAX: usize, const RHS_MIN: usize, const RHS_MAX: usize>
//     PartialEq<Bounded<RHS_MIN, RHS_MAX>> for Bounded<LHS_MIN, LHS_MAX>
// {
//     fn eq(&self, other: &Bounded<RHS_MIN, RHS_MAX>) -> bool {
//         self.0 == other.0
//     }
// }
// impl<const MIN: usize, const MAX: usize> PartialEq for Bounded<MIN, MAX> {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0
//     }
// }

impl<const MIN: usize, const MAX: usize> PartialEq<usize> for Bounded<MIN, MAX> {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[cfg(feature = "extra-traits")]
impl<const MIN: usize, const MAX: usize> Saturating for Bounded<MIN, MAX> {
    fn saturating_add(self, rhs: Self) -> Self {
        Self::checked_new(self.0.saturating_add(rhs.0)).unwrap_or(Self(MAX))
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        Self::checked_new(self.0.saturating_sub(rhs.0)).unwrap_or(Self(MIN))
    }
}

impl<const MIN: usize, const MAX: usize> Sub for Bounded<MIN, MAX> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.0
    }
}

impl<const MIN: usize, const MAX: usize> Sub<usize> for Bounded<MIN, MAX> {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}

impl<const MIN: usize, const MAX: usize> SubAssign for Bounded<MIN, MAX> {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= rhs.0;
    }
}

impl<const MIN: usize, const MAX: usize> SubAssign<usize> for Bounded<MIN, MAX> {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs;
    }
}

#[cfg(feature = "extra-traits")]
impl<const MIN: usize, const MAX: usize> Zero for Bounded<MIN, MAX> {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

#[test]
fn temp() {
    fn temp<const MIN: Bound<usize>, const MAX: Bound<usize>>() {
        let a = 9usize;
        let b = (Bound::Unbounded, MAX).contains(&a);
        println!("b: {}", b);
    }

    temp::<{ Bound::Included(0) }, { Bound::Included(9) }>();

    #[derive(Clone, Copy, Debug, Default, Deref, DerefMut, Display, From, Into)]
    #[display(fmt = "{}", _0)]
    pub struct Temp<const MIN: Bound<usize>, const MAX: Bound<usize>>(usize);

    impl<const MIN: Bound<usize>, const MAX: Bound<usize>> Temp<MIN, MAX> {
        pub fn new(value: usize) -> Temp<MIN, MAX> {
            assert!((MIN, MAX).contains(&value));
            Temp(value)
        }
    }

    Temp::<{ Bound::Included(0) }, { Bound::Included(9) }>::new(9);
    // println!("a: {:?}", a);
}

#[cfg(test)]
mod tests {
    use super::Bounded;

    mod ok {
        use super::Bounded;
        #[cfg(feature = "extra-traits")]
        use num::{CheckedAdd, CheckedSub, Saturating};

        #[test]
        fn add() {
            assert_eq!(Bounded::<0, 9>::new(9), Bounded::new(8) + Bounded::new(1));
        }

        #[test]
        fn add_assign() {
            let mut a = Bounded::<0, 9>::new(8);
            a += Bounded::new(1);
            assert_eq!(Bounded::new(9), a);
        }

        #[test]
        fn div() {
            assert_eq!(Bounded::<0, 9>::new(0), Bounded::new(0) / Bounded::new(1));
        }

        #[test]
        fn div_assign() {
            let mut a = Bounded::<0, 9>::new(0);
            a /= Bounded::new(1);
            assert_eq!(Bounded::new(0), a);
        }

        #[cfg(feature = "extra-traits")]
        #[test]
        fn checked_add() {
            assert_eq!(
                Some(Bounded::<0, 9>::new(9)),
                Bounded::new(8).checked_add(&Bounded::new(1))
            );
            assert_eq!(None, Bounded::<0, 9>::new(9).checked_add(&Bounded::new(1)));
        }

        #[cfg(feature = "extra-traits")]
        #[test]
        fn checked_sub() {
            assert_eq!(
                Some(Bounded::<0, 9>::new(0)),
                Bounded::new(1).checked_sub(&Bounded::new(1))
            );
            assert_eq!(None, Bounded::<0, 9>::new(0).checked_sub(&Bounded::new(1)));
        }

        #[test]
        fn mul() {
            assert_eq!(Bounded::<0, 9>::new(9), Bounded::new(9) * Bounded::new(1));
        }

        #[test]
        fn mul_assign() {
            let mut a = Bounded::<0, 9>::new(9);
            a *= Bounded::new(1);
            assert_eq!(Bounded::new(9), a);
        }

        #[test]
        fn new() {
            assert_eq!(1, *Bounded::<0, 2>::new(1));
        }

        #[cfg(feature = "extra-traits")]
        #[test]
        fn saturating_add() {
            assert_eq!(
                Bounded::<0, 9>::new(9),
                Bounded::new(9).saturating_add(Bounded::new(1))
            );
        }

        #[cfg(feature = "extra-traits")]
        #[test]
        fn saturating_sub() {
            assert_eq!(
                Bounded::<0, 9>::new(0),
                Bounded::new(0).saturating_sub(Bounded::new(1))
            );
        }

        #[test]
        fn sub() {
            assert_eq!(Bounded::<0, 9>::new(0), Bounded::new(1) - Bounded::new(1));
        }

        #[test]
        fn sub_assign() {
            let mut a = Bounded::<0, 9>::new(1);
            a -= Bounded::new(1);
            assert_eq!(Bounded::new(0), a);
        }
    }

    mod panic {
        use super::Bounded;

        #[test]
        #[should_panic]
        fn add() {
            let _ = Bounded::<0, 9>::new(9) + Bounded::new(1);
        }

        #[test]
        #[should_panic]
        fn add_assign() {
            let mut a = Bounded::<0, 9>::new(9);
            a += Bounded::new(1);
        }

        #[test]
        #[should_panic]
        fn div() {
            let _ = Bounded::<0, 9>::new(9) / Bounded::new(0);
        }

        #[test]
        #[should_panic]
        fn div_assign() {
            let mut a = Bounded::<0, 9>::new(9);
            a /= Bounded::new(0);
        }

        #[test]
        #[should_panic]
        fn mul() {
            let _ = Bounded::<0, 9>::new(5) * Bounded::new(2);
        }

        #[test]
        #[should_panic]
        fn mul_assign() {
            let mut a = Bounded::<0, 9>::new(5);
            a *= Bounded::new(2);
        }

        #[test]
        #[should_panic]
        fn new() {
            let _ = Bounded::<0, 9>::new(10);
        }

        #[test]
        #[should_panic]
        fn sub() {
            let _ = Bounded::<0, 9>::new(0) - Bounded::new(1);
        }

        #[test]
        #[should_panic]
        fn sub_assign() {
            let mut a = Bounded::<0, 9>::new(0);
            a -= Bounded::new(1);
        }
    }
}
