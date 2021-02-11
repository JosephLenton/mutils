use super::FromClamped;
use super::Num;

pub trait ToSignedClamped {
    type Output: Num;
    fn to_signed_clamped(self) -> Self::Output;
}

impl ToSignedClamped for u8 {
    type Output = i16;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self as i16
    }
}

impl ToSignedClamped for u16 {
    type Output = i32;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self as i32
    }
}

impl ToSignedClamped for u32 {
    type Output = i64;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self as i64
    }
}

impl ToSignedClamped for u64 {
    type Output = i64;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        i64::from_clamped(self)
    }
}

impl ToSignedClamped for usize {
    type Output = isize;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        isize::from_clamped(self)
    }
}

impl ToSignedClamped for i8 {
    type Output = Self;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self
    }
}

impl ToSignedClamped for i16 {
    type Output = Self;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self
    }
}

impl ToSignedClamped for i32 {
    type Output = Self;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self
    }
}

impl ToSignedClamped for i64 {
    type Output = Self;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self
    }
}

impl ToSignedClamped for isize {
    type Output = Self;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self
    }
}

impl ToSignedClamped for f32 {
    type Output = Self;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self
    }
}

impl ToSignedClamped for f64 {
    type Output = Self;

    #[inline(always)]
    fn to_signed_clamped(self) -> Self::Output {
        self
    }
}
