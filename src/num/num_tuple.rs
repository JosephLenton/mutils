use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::num::Num;

/// This exists to help add some common functionality to things that
/// are a tuple of two numeric items.
///
/// What are some examples of a 'tuple with two numeric items'?
/// Rect, Point, Size, Line. Things like that.
pub trait NumTuple<N: Num>:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + Copy
    + PartialEq
    + Sized
{
    fn new(first: N, second: N) -> Self;

    fn first(&self) -> N;
    fn second(&self) -> N;

    fn set_first(&mut self, n: N);
    fn set_second(&mut self, n: N);
    fn set(&mut self, first: N, second: N);
    fn get(&mut self) -> (N, N);

    fn min(self, other: Self) -> Self {
        Self::new(
            self.first().min(other.first()),
            self.second().min(other.second()),
        )
    }

    fn max(self, other: Self) -> Self {
        Self::new(
            self.first().max(other.first()),
            self.second().max(other.second()),
        )
    }

    fn max_from_zero(self, other: Self) -> Self {
        Self::new(
            self.first().max(other.first()),
            self.second().max(other.second()),
        )
    }

    fn abs(&self) -> Self {
        Self::new(self.first().abs(), self.second().abs())
    }

    fn is_outside(self, xy1: Self, xy2: Self) -> bool {
        !self.is_inside(xy1, xy2)
    }

    fn is_inside(self, xy1: Self, xy2: Self) -> bool {
        let (x_min, y_min) = xy1.min(xy2).get();
        let (x_max, y_max) = xy1.max(xy2).get();

        let x = self.first();
        let y = self.second();

        x_min <= x && x <= x_max && y_min <= y && y <= y_max
    }
}
