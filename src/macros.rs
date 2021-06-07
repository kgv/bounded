#[macro_export]
macro_rules! r#impl {
    (Bounded<$type:ty>) => {
        $crate::new!($type);
    };
    // Traits
    (Add for Bounded<$type:ty>) => {
        $crate::add!($type);
    };
    (AddAssign for Bounded<$type:ty>) => {
        $crate::add_assign!($type);
    };
    (From for Bounded<$type:ty>) => {
        $crate::from!($type);
    };
    (Div for Bounded<$type:ty>) => {
        $crate::div!($type);
    };
    (DivAssign for Bounded<$type:ty>) => {
        $crate::div_assign!($type);
    };
    (Mul for Bounded<$type:ty>) => {
        $crate::mul!($type);
    };
    (MulAssign for Bounded<$type:ty>) => {
        $crate::mul_assign!($type);
    };
    (Neg for Bounded<$type:ty>) => {
        $crate::neg!($type);
    };
    (RangeBounds for Bounded<$type:ty>) => {
        $crate::range_bounds!($type);
    };
    (Rem for Bounded<$type:ty>) => {
        $crate::rem!($type);
    };
    (RemAssign for Bounded<$type:ty>) => {
        $crate::rem_assign!($type);
    };
    (Shl for Bounded<$type:ty>) => {
        $crate::shl!($type, i8);
        $crate::shl!($type, i16);
        $crate::shl!($type, i32);
        $crate::shl!($type, i64);
        $crate::shl!($type, i128);
        $crate::shl!($type, isize);
        $crate::shl!($type, u8);
        $crate::shl!($type, u16);
        $crate::shl!($type, u32);
        $crate::shl!($type, u64);
        $crate::shl!($type, u128);
        $crate::shl!($type, usize);
    };
    (ShlAssign for Bounded<$type:ty>) => {
        $crate::shl_assign!($type, i8);
        $crate::shl_assign!($type, i16);
        $crate::shl_assign!($type, i32);
        $crate::shl_assign!($type, i64);
        $crate::shl_assign!($type, i128);
        $crate::shl_assign!($type, isize);
        $crate::shl_assign!($type, u8);
        $crate::shl_assign!($type, u16);
        $crate::shl_assign!($type, u32);
        $crate::shl_assign!($type, u64);
        $crate::shl_assign!($type, u128);
        $crate::shl_assign!($type, usize);
    };
    (Shr for Bounded<$type:ty>) => {
        $crate::shr!($type, i8);
        $crate::shr!($type, i16);
        $crate::shr!($type, i32);
        $crate::shr!($type, i64);
        $crate::shr!($type, i128);
        $crate::shr!($type, isize);
        $crate::shr!($type, u8);
        $crate::shr!($type, u16);
        $crate::shr!($type, u32);
        $crate::shr!($type, u64);
        $crate::shr!($type, u128);
        $crate::shr!($type, usize);
    };
    (ShrAssign for Bounded<$type:ty>) => {
        $crate::shr_assign!($type, i8);
        $crate::shr_assign!($type, i16);
        $crate::shr_assign!($type, i32);
        $crate::shr_assign!($type, i64);
        $crate::shr_assign!($type, i128);
        $crate::shr_assign!($type, isize);
        $crate::shr_assign!($type, u8);
        $crate::shr_assign!($type, u16);
        $crate::shr_assign!($type, u32);
        $crate::shr_assign!($type, u64);
        $crate::shr_assign!($type, u128);
        $crate::shr_assign!($type, usize);
    };
    (Sub for Bounded<$type:ty>) => {
        $crate::sub!($type);
    };
    (SubAssign for Bounded<$type:ty>) => {
        $crate::sub_assign!($type);
    };
    // Extra traits
    (Bounded for Bounded<$type:ty>) => {
        $crate::bounded!($type);
    };
    (CheckedAdd for Bounded<$type:ty>) => {
        $crate::checked_add!($type);
    };
    (CheckedDiv for Bounded<$type:ty>) => {
        $crate::checked_div!($type);
    };
    (CheckedMul for Bounded<$type:ty>) => {
        $crate::checked_mul!($type);
    };
    (CheckedNeg for Bounded<$type:ty>) => {
        $crate::checked_neg!($type);
    };
    (CheckedRem for Bounded<$type:ty>) => {
        $crate::checked_rem!($type);
    };
    (CheckedShl for Bounded<$type:ty>) => {
        $crate::checked_shl!($type);
    };
    (CheckedShr for Bounded<$type:ty>) => {
        $crate::checked_shr!($type);
    };
    (CheckedSub for Bounded<$type:ty>) => {
        $crate::checked_sub!($type);
    };
    (SaturatingAdd for Bounded<$type:ty>) => {
        $crate::saturating_add!($type);
    };
    (SaturatingDiv for Bounded<$type:ty>) => {
        $crate::saturating_div!($type);
    };
    (SaturatingMul for Bounded<$type:ty>) => {
        $crate::saturating_mul!($type);
    };
    (SaturatingNeg for Bounded<$type:ty>) => {
        $crate::saturating_neg!($type);
    };
    (SaturatingRem for Bounded<$type:ty>) => {
        $crate::saturating_rem!($type);
    };
    (SaturatingSub for Bounded<$type:ty>) => {
        $crate::saturating_sub!($type);
    };
    (Bounded<$($type:ty),+>) => {
        $(r#impl!(Bounded<$type>);)+
    };
    ($trait:ident for Bounded<$($type:ty),+>) => {
        $(r#impl!($trait for Bounded<$type>);)+
    };
}

#[macro_export]
macro_rules! new {
    ($type:ty) => {
        impl<const MIN: i128, const MAX: u128> $crate::Bounded<$type, MIN, MAX> {
            pub const fn checked_new(value: $type) -> Option<Self> {
                if MIN >= 0 {
                    assert!(MAX > MIN as u128);
                }
                assert!(MIN >= <$type>::MIN as i128);
                assert!(MAX <= <$type>::MAX as u128);
                if MIN as $type <= value && value <= MAX as $type {
                    Some(Self(value))
                } else {
                    None
                }
            }

            pub const fn new(value: $type) -> Self {
                match Self::checked_new(value) {
                    Some(val) => val,
                    None => panic!("value is out of bounds"),
                }
            }
        }
    };
}

mod traits {
    #[macro_export]
    macro_rules! add {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::Add
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    self + rhs.0
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::Add<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn add(self, rhs: $type) -> Self::Output {
                    Self::checked_new(self.0 + rhs).expect("attempt to add with out of bounds")
                }
            }
        };
    }

    #[macro_export]
    macro_rules! add_assign {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::AddAssign
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn add_assign(&mut self, rhs: Self) {
                    *self += rhs.0;
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::AddAssign<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn add_assign(&mut self, rhs: $type) {
                    *self = *self + rhs;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! div {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::Div
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    self / rhs.0
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::Div<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn div(self, rhs: $type) -> Self::Output {
                    Self::checked_new(self.0 / rhs).expect("attempt to divide with out of bounds")
                }
            }
        };
    }

    #[macro_export]
    macro_rules! div_assign {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::DivAssign
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn div_assign(&mut self, rhs: Self) {
                    *self /= rhs.0;
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::DivAssign<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn div_assign(&mut self, rhs: $type) {
                    *self = *self / rhs;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! from {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> From<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn from(from: $type) -> Self {
                    Self::new(from)
                }
            }

            impl<const MIN: i128, const MAX: u128> From<$crate::Bounded<$type, MIN, MAX>>
                for $type
            {
                fn from(from: $crate::Bounded<$type, MIN, MAX>) -> Self {
                    from.0
                }
            }
        };
    }

    #[macro_export]
    macro_rules! mul {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::Mul
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    self * rhs.0
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::Mul<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn mul(self, rhs: $type) -> Self::Output {
                    Self::checked_new(self.0 * rhs).expect("attempt to multiply with out of bounds")
                }
            }
        };
    }

    #[macro_export]
    macro_rules! mul_assign {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::MulAssign
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn mul_assign(&mut self, rhs: Self) {
                    *self *= rhs.0;
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::MulAssign<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn mul_assign(&mut self, rhs: $type) {
                    *self = *self * rhs;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! neg {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::Neg
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self::checked_new(-self.0).expect("attempt to negate with out of bounds")
                }
            }
        };
    }

    #[macro_export]
    macro_rules! range_bounds {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::RangeBounds<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn start_bound(&self) -> core::ops::Bound<&$type> {
                    core::ops::Bound::Included(&(MIN as $type))
                }

                fn end_bound(&self) -> core::ops::Bound<&$type> {
                    core::ops::Bound::Included(&(MAX as $type))
                }
            }
        };
    }

    #[macro_export]
    macro_rules! rem {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::Rem
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn rem(self, rhs: Self) -> Self::Output {
                    self % rhs.0
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::Rem<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn rem(self, rhs: $type) -> Self::Output {
                    Self::checked_new(self.0 % rhs)
                        .expect("attempt to calculate the remainder with out of bounds")
                }
            }
        };
    }

    #[macro_export]
    macro_rules! rem_assign {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::RemAssign
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn rem_assign(&mut self, rhs: Self) {
                    *self %= rhs.0;
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::RemAssign<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn rem_assign(&mut self, rhs: $type) {
                    *self = *self % rhs;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! shl {
        ($lhs:ty, $rhs:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::Shl<$rhs>
                for $crate::Bounded<$lhs, MIN, MAX>
            {
                type Output = Self;

                fn shl(self, rhs: $rhs) -> Self::Output {
                    Self::checked_new(self.0 << rhs)
                        .expect("attempt to shift left with out of bounds")
                }
            }
        };
    }

    #[macro_export]
    macro_rules! shl_assign {
        ($lhs:ty, $rhs:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::ShlAssign<$rhs>
                for $crate::Bounded<$lhs, MIN, MAX>
            {
                fn shl_assign(&mut self, rhs: $rhs) {
                    *self = *self << rhs;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! shr {
        ($lhs:ty, $rhs:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::Shr<$rhs>
                for $crate::Bounded<$lhs, MIN, MAX>
            {
                type Output = Self;

                fn shr(self, rhs: $rhs) -> Self::Output {
                    Self::checked_new(self.0 >> rhs)
                        .expect("attempt to shift right with out of bounds")
                }
            }
        };
    }

    #[macro_export]
    macro_rules! shr_assign {
        ($lhs:ty, $rhs:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::ShrAssign<$rhs>
                for $crate::Bounded<$lhs, MIN, MAX>
            {
                fn shr_assign(&mut self, rhs: $rhs) {
                    *self = *self >> rhs;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! sub {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::Sub
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    self - rhs.0
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::Sub<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                type Output = Self;

                fn sub(self, rhs: $type) -> Self::Output {
                    Self::checked_new(self.0 - rhs).expect("attempt to subtract with out of bounds")
                }
            }
        };
    }

    #[macro_export]
    macro_rules! sub_assign {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> core::ops::SubAssign
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn sub_assign(&mut self, rhs: Self) {
                    *self -= rhs.0;
                }
            }

            impl<const MIN: i128, const MAX: u128> core::ops::SubAssign<$type>
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn sub_assign(&mut self, rhs: $type) {
                    *self = *self - rhs;
                }
            }
        };
    }
}

#[cfg(feature = "extra-traits")]
mod extra_traits {
    #[macro_export]
    macro_rules! bounded {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::Bounded
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn min_value() -> Self {
                    Self(MIN as $type)
                }

                fn max_value() -> Self {
                    Self(MAX as $type)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_add {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::CheckedAdd
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_new(self.0.checked_add(rhs.0)?)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_div {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::CheckedDiv
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn checked_div(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_new(self.0.checked_div(rhs.0)?)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_mul {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::CheckedMul
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_new(self.0.checked_mul(rhs.0)?)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_neg {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::CheckedNeg
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn checked_neg(&self) -> Option<Self> {
                    Self::checked_new(self.0.checked_neg()?)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_rem {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::CheckedRem
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn checked_rem(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_new(self.0.checked_rem(rhs.0)?)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_shl {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::CheckedShl
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn checked_shl(&self, rhs: u32) -> Option<Self> {
                    Self::checked_new(self.0.checked_shl(rhs)?)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_shr {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::CheckedShr
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn checked_shr(&self, rhs: u32) -> Option<Self> {
                    Self::checked_new(self.0.checked_shr(rhs)?)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! checked_sub {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::CheckedSub
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_new(self.0.checked_sub(rhs.0)?)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! saturating_add {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::SaturatingAdd
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn saturating_add(&self, rhs: &Self) -> Self {
                    Self::checked_new(self.0.saturating_add(rhs.0)).unwrap_or(Self(MAX as $type))
                }
            }
        };
    }

    #[macro_export]
    macro_rules! saturating_div {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> $crate::SaturatingDiv
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn saturating_div(&self, rhs: &Self) -> Self {
                    Self::checked_new(self.0 / rhs.0).unwrap_or(Self(MIN as $type))
                }
            }
        };
    }

    #[macro_export]
    macro_rules! saturating_mul {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::SaturatingMul
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn saturating_mul(&self, rhs: &Self) -> Self {
                    Self::checked_new(self.0.saturating_mul(rhs.0)).unwrap_or(Self(MAX as $type))
                }
            }
        };
    }

    #[macro_export]
    macro_rules! saturating_sub {
        ($type:ty) => {
            impl<const MIN: i128, const MAX: u128> num_traits::SaturatingSub
                for $crate::Bounded<$type, MIN, MAX>
            {
                fn saturating_sub(&self, rhs: &Self) -> Self {
                    Self::checked_new(self.0.saturating_sub(rhs.0)).unwrap_or(Self(MIN as $type))
                }
            }
        };
    }
}
