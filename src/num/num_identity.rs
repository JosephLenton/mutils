pub trait NumIdentity {
    fn min() -> Self;
    fn max() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! NumIdentityImplInteger {
    ( $type_name:ident ) => {
        impl NumIdentity for $type_name {
            #[inline(always)]
            fn min() -> Self {
                Self::MIN
            }

            #[inline(always)]
            fn max() -> Self {
                Self::MAX
            }

            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn one() -> Self {
                1
            }
        }
    };
}

macro_rules! NumIdentityImplFloat {
    ( $type_name:ident ) => {
        impl NumIdentity for $type_name {
            #[inline(always)]
            fn min() -> Self {
                Self::MIN
            }

            #[inline(always)]
            fn max() -> Self {
                Self::MAX
            }

            #[inline(always)]
            fn zero() -> Self {
                0.0
            }

            #[inline(always)]
            fn one() -> Self {
                1.0
            }
        }
    };
}

NumIdentityImplInteger!(u8);
NumIdentityImplInteger!(u16);
NumIdentityImplInteger!(u32);
NumIdentityImplInteger!(u64);
NumIdentityImplInteger!(u128);
NumIdentityImplInteger!(usize);

NumIdentityImplInteger!(i8);
NumIdentityImplInteger!(i16);
NumIdentityImplInteger!(i32);
NumIdentityImplInteger!(i64);
NumIdentityImplInteger!(i128);
NumIdentityImplInteger!(isize);

NumIdentityImplFloat!(f32);
NumIdentityImplFloat!(f64);
