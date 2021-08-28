use crate::num::Num;
use crate::num::ToRounded;

pub trait FromRounded<N> {
    fn from_rounded(n: N) -> Self;
}

impl<N> FromRounded<f32> for N
where
    N: Num,
    f32: ToRounded<N>,
{
    #[inline(always)]
    fn from_rounded(o: f32) -> N {
        ToRounded::<N>::to_rounded(o)
    }
}
