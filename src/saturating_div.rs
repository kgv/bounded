use std::ops::Div;

/// Performs division that saturates at the numeric bounds instead of
/// overflowing
#[cfg(feature = "saturating-div")]
pub trait SaturatingDiv: Sized + Div<Self, Output = Self> {
    #[must_use]
    fn saturating_div(&self, rhs: &Self) -> Self;
}
