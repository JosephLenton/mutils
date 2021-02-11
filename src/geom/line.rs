use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Div;
use ::std::ops::DivAssign;
use ::std::ops::Mul;
use ::std::ops::MulAssign;
use ::std::ops::Neg;
use ::std::ops::Rem;
use ::std::ops::Sub;
use ::std::ops::SubAssign;

use ::num_traits::sign::Signed;

use crate::num::Num;
use crate::num::ToSignedClamped;

use crate::geom::Point;
use crate::geom::Size;

use crate::Random;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line<N: Num = f32>(pub Point<N>, pub Point<N>);

impl<N: Num> Line<N> {
    #[inline(always)]
    pub fn start(self) -> Point<N> {
        self.0
    }

    #[inline(always)]
    pub fn end(self) -> Point<N> {
        self.1
    }

    pub fn left_x(self) -> N {
        self.start().x().min(self.end().x())
    }

    pub fn right_x(self) -> N {
        self.start().x().max(self.end().x())
    }

    pub fn bottom_y(self) -> N {
        self.start().y().min(self.end().y())
    }

    pub fn top_y(self) -> N {
        self.start().y().max(self.end().y())
    }

    pub fn is_horizontal(self) -> bool {
        self.start().y() == self.end().y()
    }

    pub fn is_vertical(self) -> bool {
        self.start().x() == self.end().x()
    }

    pub fn is_straight(self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
}

impl<N: Num + ToSignedClamped> Line<N> {
    pub fn diff(self) -> Size<<N as ToSignedClamped>::Output> {
        Size(self.x_diff(), self.y_diff())
    }

    pub fn x_diff(self) -> <N as ToSignedClamped>::Output {
        self.start().x().to_signed_clamped() - self.end().x().to_signed_clamped()
    }

    pub fn y_diff(self) -> <N as ToSignedClamped>::Output {
        self.start().y().to_signed_clamped() - self.end().y().to_signed_clamped()
    }
}

impl Line {
    pub fn midpoint(self) -> Point {
        (self.start() + self.end()) / 2.0
    }

    /**
     * Returns the atan2( y dist, x dist )
     */
    pub fn angle(self) -> f32 {
        self.y_diff().atan2(self.x_diff())
    }

    pub fn hypot(self) -> f32 {
        let Line(Point(x1, y1), Point(x2, y2)) = self;
        let x_diff = x2 - x1;
        let y_diff = y2 - y1;
        let hypot_sqrd = (x_diff * x_diff) + (y_diff * y_diff);

        hypot_sqrd.sqrt()
    }
}

impl Add<Random> for Line {
    type Output = Self;

    fn add(self, rnd: Random) -> Self {
        rnd + self
    }
}

impl<N: Num> Add<Point<N>> for Line<N> {
    type Output = Self;

    fn add(self, other: Point<N>) -> Self {
        Self(self.0 + other, self.1 + other)
    }
}

impl<N: Num> AddAssign<Point<N>> for Line<N> {
    fn add_assign(&mut self, other: Point<N>) {
        self.0 += other;
        self.1 += other;
    }
}

impl<N: Num> Sub<Point<N>> for Line<N> {
    type Output = Self;

    fn sub(self, other: Point<N>) -> Self {
        Self(self.0 - other, self.1 - other)
    }
}

impl<N: Num> SubAssign<Point<N>> for Line<N> {
    fn sub_assign(&mut self, other: Point<N>) {
        self.0 -= other;
        self.1 -= other;
    }
}

impl<N: Num> Mul<Point<N>> for Line<N> {
    type Output = Self;

    fn mul(self, other: Point<N>) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl<N: Num> MulAssign<Point<N>> for Line<N> {
    fn mul_assign(&mut self, other: Point<N>) {
        self.0 *= other;
        self.1 *= other;
    }
}

impl<N: Num> Div<Point<N>> for Line<N> {
    type Output = Self;

    fn div(self, other: Point<N>) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}

impl<N: Num> DivAssign<Point<N>> for Line<N> {
    fn div_assign(&mut self, other: Point<N>) {
        self.0 /= other;
        self.1 /= other;
    }
}

impl<N: Num> Rem<Point<N>> for Line<N> {
    type Output = Self;

    fn rem(self, other: Point<N>) -> Self {
        Self(self.0 % other, self.1 % other)
    }
}

impl<N: Num> Add<Line<N>> for Line<N> {
    type Output = Self;

    fn add(self, other: Line<N>) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<N: Num> AddAssign<Line<N>> for Line<N> {
    fn add_assign(&mut self, other: Line<N>) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl<N: Num> Sub<Line<N>> for Line<N> {
    type Output = Self;

    fn sub(self, other: Line<N>) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<N: Num> SubAssign<Line<N>> for Line<N> {
    fn sub_assign(&mut self, other: Line<N>) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl<N: Num> Mul<Line<N>> for Line<N> {
    type Output = Self;

    fn mul(self, other: Line<N>) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl<N: Num> MulAssign<Line<N>> for Line<N> {
    fn mul_assign(&mut self, other: Line<N>) {
        self.0 *= other.0;
        self.1 *= other.1;
    }
}

impl<N: Num> Div<Line<N>> for Line<N> {
    type Output = Self;

    fn div(self, other: Line<N>) -> Self {
        Self(self.0 / other.0, self.1 / other.1)
    }
}

impl<N: Num> DivAssign<Line<N>> for Line<N> {
    fn div_assign(&mut self, other: Line<N>) {
        self.0 /= other.0;
        self.1 /= other.1;
    }
}

impl<N: Num> Rem<Line<N>> for Line<N> {
    type Output = Self;

    fn rem(self, other: Line<N>) -> Self {
        Self(self.0 % other.0, self.1 % other.1)
    }
}

impl<N: Num> Mul<Size<N>> for Line<N> {
    type Output = Self;

    fn mul(self, other: Size<N>) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl<N: Num> MulAssign<Size<N>> for Line<N> {
    fn mul_assign(&mut self, other: Size<N>) {
        self.0 *= other;
        self.1 *= other;
    }
}

impl<N: Num> Div<Size<N>> for Line<N> {
    type Output = Self;

    fn div(self, other: Size<N>) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}

impl<N: Num> DivAssign<Size<N>> for Line<N> {
    fn div_assign(&mut self, other: Size<N>) {
        self.0 /= other;
        self.1 /= other;
    }
}

impl<N: Num> Mul<N> for Line<N> {
    type Output = Self;

    fn mul(self, other: N) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl<N: Num> MulAssign<N> for Line<N> {
    fn mul_assign(&mut self, other: N) {
        self.0 *= other;
        self.1 *= other;
    }
}

impl<N: Num> Div<N> for Line<N> {
    type Output = Self;

    fn div(self, other: N) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}

impl<N: Num> DivAssign<N> for Line<N> {
    fn div_assign(&mut self, other: N) {
        self.0 /= other;
        self.1 /= other;
    }
}

impl<N: Num + Signed> Neg for Line<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl<N: Num> From<(Point<N>, Point<N>)> for Line<N> {
    fn from((start, end): (Point<N>, Point<N>)) -> Self {
        Self(start, end)
    }
}

impl<N: Num> Into<(Point<N>, Point<N>)> for Line<N> {
    fn into(self) -> (Point<N>, Point<N>) {
        (self.0, self.1)
    }
}

#[cfg(test)]
mod is_horizontal {
    use super::*;

    #[test]
    fn it_should_be_true_when_horizontal() {
        let line = Line(Point(5, 9), Point(8, 9));
        assert_eq!(line.is_horizontal(), true);
    }

    #[test]
    fn it_should_be_false_when_not_horizontal() {
        let line = Line(Point(5, 9), Point(8, 10));
        assert_eq!(line.is_horizontal(), false);
    }
}

#[cfg(test)]
mod is_vertical {
    use super::*;

    #[test]
    fn it_should_be_true_when_vertical() {
        let line = Line(Point(5, 9), Point(5, 19));
        assert_eq!(line.is_vertical(), true);
    }

    #[test]
    fn it_should_be_false_when_not_vertical() {
        let line = Line(Point(5, 9), Point(8, 10));
        assert_eq!(line.is_vertical(), false);
    }
}

#[cfg(test)]
mod is_straight {
    use super::*;

    #[test]
    fn it_should_be_true_when_vertical() {
        let line = Line(Point(5, 9), Point(5, 19));
        assert_eq!(line.is_vertical(), true);
    }

    #[test]
    fn it_should_be_true_when_horizontal() {
        let line = Line(Point(5, 9), Point(8, 9));
        assert_eq!(line.is_horizontal(), true);
    }

    #[test]
    fn it_should_be_false_when_angled() {
        let line = Line(Point(5, 9), Point(8, 10));
        assert_eq!(line.is_vertical(), false);
    }
}
