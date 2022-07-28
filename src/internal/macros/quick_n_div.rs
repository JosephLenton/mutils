macro_rules! quick_n_div {
    ( $n:ty, $geom:ty ) => {
        impl ::std::ops::Div<$geom> for $n {
            type Output = $geom;

            #[inline(always)]
            fn div(self, other: $geom) -> $geom {
                other / self
            }
        }
    };
}

pub(crate) use quick_n_div;
