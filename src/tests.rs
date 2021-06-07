use crate::Bounded;

type Digit = Bounded<i8, 0, 9>;

type N1 = Bounded<i8, 1, { i8::MAX as _ }>;

type Z = Bounded<i8, { i8::MIN as _ }, { i8::MAX as _ }>;

mod new {
    use super::*;

    #[test]
    #[should_panic(expected = "value is out of bounds")]
    fn out_of_max_bound() {
        let _ = Digit::new(10);
    }

    #[test]
    #[should_panic(expected = "value is out of bounds")]
    fn out_of_min_bound() {
        let _ = Digit::new(-1);
    }
}

mod checked_new {
    use super::*;

    #[test]
    fn out_of_max_bound() {
        assert_eq!(None, Digit::checked_new(10));
    }

    #[test]
    fn out_of_min_bound() {
        assert_eq!(None, Digit::checked_new(-1));
    }
}

mod traits {
    use super::*;

    mod add {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to add with out of bounds")]
        fn out_of_bounds() {
            let _ = Digit::new(9) + Digit::new(1);
        }

        #[test]
        #[should_panic(expected = "attempt to add with overflow")]
        fn overflow() {
            let _ = Z::new(i8::MAX) + Z::new(1);
        }
    }

    mod add_assign {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to add with out of bounds")]
        fn out_of_bounds() {
            let mut digit = Digit::new(9);
            digit += Digit::new(1);
        }

        #[test]
        #[should_panic(expected = "attempt to add with overflow")]
        fn overflow() {
            let mut byte = Z::new(i8::MAX);
            byte += Z::new(1);
        }
    }

    mod div {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn divide_by_zero() {
            let _ = Z::new(1) / Z::new(0);
        }

        #[test]
        #[should_panic(expected = "attempt to divide with out of bounds")]
        fn out_of_bounds() {
            let _ = N1::new(1) / N1::new(9);
        }

        #[test]
        #[should_panic(expected = "attempt to divide with overflow")]
        fn overflow() {
            let _ = Z::new(-128) / Z::new(-1);
        }
    }

    mod div_assign {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn divide_by_zero() {
            let mut digit = Z::new(1);
            digit /= Z::new(0);
        }

        #[test]
        #[should_panic(expected = "attempt to divide with out of bounds")]
        fn out_of_bounds() {
            let mut digit = N1::new(1);
            digit /= N1::new(9);
        }

        #[test]
        #[should_panic(expected = "attempt to divide with overflow")]
        fn overflow() {
            let mut byte = Z::new(-128);
            byte /= Z::new(-1);
        }
    }

    mod from {
        use super::*;

        #[test]
        #[should_panic(expected = "value is out of bounds")]
        fn out_of_max_bound() {
            let _ = Digit::from(10);
        }

        #[test]
        #[should_panic(expected = "value is out of bounds")]
        fn out_of_min_bound() {
            let _ = Digit::from(-1);
        }
    }

    mod mul {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to multiply with out of bounds")]
        fn out_of_bounds() {
            let _ = Digit::new(9) * Digit::new(9);
        }

        #[test]
        #[should_panic(expected = "attempt to multiply with overflow")]
        fn overflow() {
            let _ = Z::new(i8::MAX) * Z::new(2);
        }
    }

    mod mul_assign {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to multiply with out of bounds")]
        fn out_of_bounds() {
            let mut digit = Digit::new(9);
            digit *= Digit::new(9);
        }

        #[test]
        #[should_panic(expected = "attempt to multiply with overflow")]
        fn overflow() {
            let mut byte = Z::new(i8::MAX);
            byte *= Z::new(2);
        }
    }

    mod neg {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to negate with out of bounds")]
        fn out_of_bounds() {
            let _ = -Digit::new(9);
        }

        #[test]
        #[should_panic(expected = "attempt to negate with overflow")]
        fn overflow() {
            let _ = -Z::new(i8::MIN);
        }
    }

    mod range_bounds {
        use super::*;

        #[test]
        #[should_panic(expected = "assertion failed: MIN >= <i8>::MIN as i128")]
        fn min_less_type_min() {
            let _ = Bounded::<i8, { i8::MIN as i128 - 1 }, { i8::MAX as u128 }>::new(0);
        }

        #[test]
        #[should_panic(expected = "assertion failed: MAX <= <i8>::MAX as u128")]
        fn max_greater_type_max() {
            let _ = Bounded::<i8, { i8::MIN as i128 }, { i8::MAX as u128 + 1 }>::new(0);
        }
    }

    mod rem {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
        fn divide_by_zero() {
            let _ = Z::new(1) % Z::new(0);
        }

        #[test]
        #[should_panic(expected = "attempt to calculate the remainder with out of bounds")]
        fn out_of_bounds() {
            let _ = N1::new(1) % N1::new(1);
        }

        #[test]
        #[should_panic(expected = "attempt to calculate the remainder with overflow")]
        fn overflow() {
            let _ = Z::new(i8::MIN) % Z::new(-1);
        }
    }

    mod rem_assign {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
        fn divide_by_zero() {
            let mut digit = Z::new(1);
            digit %= Z::new(0);
        }

        #[test]
        #[should_panic(expected = "attempt to calculate the remainder with out of bounds")]
        fn out_of_bounds() {
            let mut digit = N1::new(1);
            digit %= N1::new(1);
        }

        #[test]
        #[should_panic(expected = "attempt to calculate the remainder with overflow")]
        fn overflow() {
            let mut byte = Z::new(i8::MIN);
            byte %= Z::new(-1);
        }
    }

    mod shl {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to shift left with out of bounds")]
        fn out_of_bounds() {
            let _ = Digit::new(1) << 4;
        }

        #[test]
        #[should_panic(expected = "attempt to shift left with overflow")]
        fn overflow() {
            let _ = Z::new(1) << 8;
        }
    }

    mod shl_assign {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to shift left with out of bounds")]
        fn out_of_bounds() {
            let mut digit = Digit::new(1);
            digit <<= 4;
        }

        #[test]
        #[should_panic(expected = "attempt to shift left with overflow")]
        fn overflow() {
            let mut digit = Z::new(1);
            digit <<= 8;
        }
    }

    mod shr {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to shift right with out of bounds")]
        fn out_of_bounds() {
            let _ = N1::new(1) >> 1;
        }

        #[test]
        #[should_panic(expected = "attempt to shift right with overflow")]
        fn overflow() {
            let _ = Z::new(1) >> 8;
        }
    }

    mod shr_assign {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to shift right with out of bounds")]
        fn out_of_bounds() {
            let mut digit = N1::new(1);
            digit >>= 1;
        }

        #[test]
        #[should_panic(expected = "attempt to shift right with overflow")]
        fn overflow() {
            let mut digit = Z::new(1);
            digit >>= 8;
        }
    }

    mod sub {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to subtract with out of bounds")]
        fn out_of_bounds() {
            let _ = Digit::new(0) - Digit::new(1);
        }

        #[test]
        #[should_panic(expected = "attempt to subtract with overflow")]
        fn overflow() {
            let _ = Z::new(i8::MIN) - Z::new(1);
        }
    }

    mod sub_assign {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to subtract with out of bounds")]
        fn out_of_bounds() {
            let mut digit = Digit::new(0);
            digit -= Digit::new(1);
        }

        #[test]
        #[should_panic(expected = "attempt to subtract with overflow")]
        fn overflow() {
            let mut byte = Z::new(i8::MIN);
            byte -= Z::new(1);
        }
    }
}

#[cfg(any(
    feature = "checked-add",
    feature = "checked-div",
    feature = "checked-mul",
    feature = "checked-neg",
    feature = "checked-rem",
    feature = "checked-shl",
    feature = "checked-shr",
    feature = "checked-sub",
    feature = "saturating-add",
    feature = "saturating-div",
    feature = "saturating-mul",
    feature = "saturating-sub",
))]
mod extra_traits {
    use super::*;

    #[cfg(feature = "checked-add")]
    mod checked_add {
        use super::*;
        use num_traits::CheckedAdd;

        #[test]
        fn out_of_bounds() {
            assert_eq!(None, Digit::new(9).checked_add(&Digit::new(1)));
        }

        #[test]
        fn overflow() {
            assert_eq!(None, Z::new(i8::MAX).checked_add(&Z::new(1)));
        }
    }

    #[cfg(feature = "checked-div")]
    mod checked_div {
        use super::*;
        use num_traits::CheckedDiv;

        #[test]
        fn divide_by_zero() {
            assert_eq!(None, Z::new(1).checked_div(&Z::new(0)));
        }

        #[test]
        fn out_of_bounds() {
            assert_eq!(None, N1::new(1).checked_div(&N1::new(9)));
        }
    }

    #[cfg(feature = "checked-mul")]
    mod checked_mul {
        use super::*;
        use num_traits::CheckedMul;

        #[test]
        fn out_of_bounds() {
            assert_eq!(None, Digit::new(9).checked_mul(&Digit::new(2)));
        }

        #[test]
        fn overflow() {
            assert_eq!(None, Z::new(i8::MAX).checked_mul(&Z::new(2)));
        }
    }

    #[cfg(feature = "checked-neg")]
    mod checked_neg {
        use super::*;
        use num_traits::CheckedNeg;

        #[test]
        fn out_of_bounds() {
            assert_eq!(None, Digit::new(9).checked_neg());
        }

        #[test]
        fn overflow() {
            assert_eq!(None, Z::new(i8::MIN).checked_neg());
        }
    }

    #[cfg(feature = "checked-rem")]
    mod checked_rem {
        use super::*;
        use num_traits::CheckedRem;

        #[test]
        fn divide_by_zero() {
            assert_eq!(None, Z::new(1).checked_rem(&Z::new(0)));
        }

        #[test]
        fn out_of_bounds() {
            assert_eq!(None, N1::new(1).checked_rem(&N1::new(1)));
        }

        #[test]
        fn overflow() {
            assert_eq!(None, Z::new(i8::MIN).checked_rem(&Z::new(-1)));
        }
    }

    #[cfg(feature = "checked-shl")]
    mod checked_shl {
        use super::*;
        use num_traits::CheckedShl;

        #[test]
        fn out_of_bounds() {
            assert_eq!(None, Digit::new(1).checked_shl(4));
        }

        #[test]
        fn overflow() {
            assert_eq!(None, Z::new(1).checked_shl(8));
        }
    }

    #[cfg(feature = "checked-shr")]
    mod checked_shr {
        use super::*;
        use num_traits::CheckedShr;

        #[test]
        fn out_of_bounds() {
            assert_eq!(None, N1::new(1).checked_shr(1));
        }

        #[test]
        fn overflow() {
            assert_eq!(None, Z::new(1).checked_shr(8));
        }
    }

    #[cfg(feature = "checked-sub")]
    mod checked_sub {
        use super::*;
        use num_traits::CheckedSub;

        #[test]
        fn out_of_bounds() {
            assert_eq!(None, N1::new(1).checked_sub(&N1::new(1)));
        }

        #[test]
        fn overflow() {
            assert_eq!(None, Z::new(i8::MIN).checked_sub(&Z::new(1)));
        }
    }

    #[cfg(feature = "saturating-add")]
    mod saturating_add {
        use super::*;
        use num_traits::SaturatingAdd;

        #[test]
        fn out_of_bounds() {
            assert_eq!(Digit::new(9), Digit::new(9).saturating_add(&Digit::new(1)));
        }

        #[test]
        fn overflow() {
            assert_eq!(Z::new(i8::MAX), Z::new(i8::MAX).saturating_add(&Z::new(1)));
        }
    }

    #[cfg(feature = "saturating-div")]
    mod saturating_div {
        use super::*;
        use crate::SaturatingDiv;

        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn divide_by_zero() {
            let _ = Z::new(1).saturating_div(&Z::new(0));
        }

        #[test]
        fn out_of_bounds() {
            assert_eq!(N1::new(1), N1::new(1).saturating_div(&N1::new(9)));
        }
    }

    #[cfg(feature = "saturating-mul")]
    mod saturating_mul {
        use super::*;
        use num_traits::SaturatingMul;

        #[test]
        fn out_of_bounds() {
            assert_eq!(Digit::new(9), Digit::new(9).saturating_mul(&Digit::new(2)));
        }

        #[test]
        fn overflow() {
            assert_eq!(Z::new(i8::MAX), Z::new(i8::MAX).saturating_mul(&Z::new(2)));
        }
    }

    #[cfg(feature = "saturating-sub")]
    mod saturating_sub {
        use super::*;
        use num_traits::SaturatingSub;

        #[test]
        fn out_of_bounds() {
            assert_eq!(N1::new(1), N1::new(1).saturating_sub(&N1::new(1)));
        }

        #[test]
        fn overflow() {
            assert_eq!(Z::new(i8::MIN), Z::new(i8::MIN).saturating_sub(&Z::new(1)));
        }
    }
}
