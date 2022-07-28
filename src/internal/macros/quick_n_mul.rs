macro_rules! quick_n_mul {
    ( $n:ty, $geom:ty ) => {
        impl ::std::ops::Mul<$geom> for $n {
            type Output = $geom;

            #[inline(always)]
            fn mul(self, other: $geom) -> $geom {
                other * self
            }
        }
    };
}

pub(crate) use quick_n_mul;
