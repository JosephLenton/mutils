use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Div;
use ::std::ops::DivAssign;
use ::std::ops::Mul;
use ::std::ops::MulAssign;
use ::std::ops::Neg;
use ::std::ops::Rem;
use ::std::ops::Shl;
use ::std::ops::ShlAssign;
use ::std::ops::Shr;
use ::std::ops::ShrAssign;
use ::std::ops::Sub;
use ::std::ops::SubAssign;

use ::num_traits::sign::Signed;

use crate::num::INum;
use crate::num::Num;
use crate::num::ToRounded;
use crate::num::ToSignedClamped;

use crate::geom::Point;
use crate::geom::Rect;
use crate::geom::Size;

use crate::geom::HorizontalPosition;
use crate::geom::PointPosition;
use crate::geom::VerticalPosition;

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

    /// This is to allow creating a new Point, with a new type, from the type given.
    /// i.e. `Point::new(1 as u8, 1 as u8)::to::<u32>()`
    pub fn to<T: Num + From<N>>(&self) -> Line<T> {
        Line(self.start().to::<T>(), self.end().to::<T>())
    }

    pub fn to_rect(self) -> Rect<N> {
        self.start().rect_to(self.end())
    }
}

impl<N: Num + ToSignedClamped> Line<N> {
    pub fn to_signed_clamped(self) -> Line<<N as ToSignedClamped>::Output> {
        Line(
            self.start().to_signed_clamped(),
            self.end().to_signed_clamped(),
        )
    }

    pub fn diff(self) -> Size<<N as ToSignedClamped>::Output> {
        Size(self.x_diff(), self.y_diff())
    }

    pub fn x_diff(self) -> <N as ToSignedClamped>::Output {
        self.end().x().to_signed_clamped() - self.start().x().to_signed_clamped()
    }

    pub fn y_diff(self) -> <N as ToSignedClamped>::Output {
        self.end().y().to_signed_clamped() - self.start().y().to_signed_clamped()
    }

    pub fn slope(self) -> <N as ToSignedClamped>::Output {
        let diff = self.diff();
        diff.height() / diff.width()
    }

    pub fn inverse_slope(self) -> <N as ToSignedClamped>::Output {
        let diff = self.diff();
        diff.width() / diff.height()
    }
}

impl<N: Num + ToRounded<f32>> Line<N>
where
    f32: ToRounded<N>,
{
    pub fn intersect(self, rect: Rect<N>) -> Option<Line<N>> {
        let mut position = rect.line_position(self);

        // No clippin needed.
        if position.is_entirely_inside() {
            return Some(self);
        }

        // Is either fully above, or fully below.
        if position.is_on_same_horizontal()
            && position.start().horizontal() != HorizontalPosition::Inside
        {
            return None;
        }

        // Is either fully on the left, or fully on the right.
        if position.is_on_same_vertical() && position.start().vertical() != VerticalPosition::Inside
        {
            return None;
        }

        // Lines left either ...
        //  - Need Clipping
        //    - Start outside, and end inside.
        //    - Start inside, and end outside.
        //    - Start and end outside, and cross the inside.
        //  - Are also excluded
        //    - Start and end outside, but don't quite cross the inside. They came close.

        // Should only iterate twice, at most.
        let mut line_f32: Line<f32> = self.to_rounded();
        let rect_f32: Rect<f32> = rect.to_rounded();
        loop {
            // Case 1:
            // both endpoints are within the clipping region
            if position.is_entirely_inside() {
                return Some(line_f32.to_rounded());
            }

            // Starts and ends in the same space, and this is outside.
            if position.is_within_same_space() && position.start().is_entirely_outside() {
                return None;
            }

            // Case 3:
            // The endpoints are in different regions, and the segment is partially within the clipping rectangle

            // Select one of the endpoints outside the clipping rectangle
            let point_position = if position.start().is_outside() {
                position.start()
            } else {
                position.end()
            };

            // Calculate the intersection of the line with the clipping rectangle.
            match line_f32.calculate_intersection(rect_f32, point_position) {
                None => return None,
                Some(new_point) => {
                    // Update the point after clipping and recalculate outcode.
                    if point_position == position.start() {
                        line_f32 = Line(new_point, line_f32.end());
                        position = rect_f32.line_position(line_f32);
                    } else {
                        line_f32 = Line(line_f32.start(), new_point);
                        position = rect_f32.line_position(line_f32);
                    }

                    // This case happens when there are lines that go close to the rectangle,
                    // but don't quite clip it.
                    //
                    // i.e.
                    //
                    //         /
                    //        /
                    //       /
                    //      / -------
                    //     /  |
                    //    /   |
                    //   /    |
                    //        |
                    //
                    if rect_f32.intersect(line_f32.to_rect()) == None {
                        return None;
                    }
                }
            }
        }
    }
}

impl Line<f32> {
    fn calculate_intersection(self, rect: Rect<f32>, clip_to: PointPosition) -> Option<Point<f32>> {
        let p1 = self.start();
        let slope = self.slope();
        let inverse_slope = self.inverse_slope();

        if clip_to.is_above() {
            let new_x = p1.x() + inverse_slope * (rect.top_y() - p1.y());
            let new_y = rect.top_y();

            return Some(Point(new_x, new_y));
        }

        if clip_to.is_below() {
            let new_x = p1.x() + inverse_slope * (rect.bottom_y() - p1.y());
            let new_y = rect.bottom_y();

            return Some(Point(new_x, new_y));
        }

        if clip_to.is_right() {
            let new_x = rect.right_x();
            let new_y = p1.y() + slope * (rect.right_x() - p1.x());

            return Some(Point(new_x, new_y));
        }

        if clip_to.is_left() {
            let new_x = rect.left_x();
            let new_y = p1.y() + slope * (rect.left_x() - p1.x());

            return Some(Point(new_x, new_y));
        }

        None
    }
}

impl<O: Num, N: Num + ToRounded<O>> ToRounded<Line<O>> for Line<N> {
    fn to_rounded(self) -> Line<O> {
        Line(self.start().to_rounded(), self.end().to_rounded())
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

impl<N: INum> Shl<N> for Line<N> {
    type Output = Self;

    fn shl(self, other: N) -> Self {
        Self(self.0 << other, self.1 << other)
    }
}

impl<N: INum> ShlAssign<N> for Line<N> {
    fn shl_assign(&mut self, other: N) {
        *self = *self << other;
    }
}

impl<N: INum> Shr<N> for Line<N> {
    type Output = Self;

    fn shr(self, other: N) -> Self {
        Self(self.0 >> other, self.1 >> other)
    }
}

impl<N: INum> ShrAssign<N> for Line<N> {
    fn shr_assign(&mut self, other: N) {
        *self = *self >> other;
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

#[cfg(test)]
mod diff {
    use super::*;

    #[test]
    fn it_should_be_end_minus_start() {
        let diff: Line<i32> = Line(Point(5, 9), Point(24, -3));
        assert_eq!(diff.diff(), Size(19, -12));
    }
}

#[cfg(test)]
mod intersect {
    use super::*;

    #[test]
    fn it_should_ignore_lines_fully_above() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(5.0, 5.0), Point(25.0, 5.0));
        assert_eq!(line.intersect(rect), None);
    }

    #[test]
    fn it_should_ignore_lines_fully_left() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(5.0, 5.0), Point(5.0, 25.0));
        assert_eq!(line.intersect(rect), None);
    }

    #[test]
    fn it_should_ignore_lines_fully_right() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(25.0, 5.0), Point(25.0, 25.0));
        assert_eq!(line.intersect(rect), None);
    }

    #[test]
    fn it_should_ignore_lines_fully_below() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(5.0, 25.0), Point(25.0, 25.0));
        assert_eq!(line.intersect(rect), None);
    }

    /// This tests for when lines go very close to a corner,
    /// but don't quite clip it.
    ///
    /// i.e.
    ///
    ///         /
    ///        /
    ///       /
    ///      / -------
    ///     /  |
    ///    /   |
    ///   /    |
    ///        |
    ///
    #[test]
    fn it_should_ignore_lines_crossing_a_corner() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(5.0, 12.0), Point(12.0, 5.0));
        assert_eq!(line.intersect(rect), None);
    }

    #[test]
    fn it_should_not_clip_lines_fully_inside() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));

        let l1 = Line(Point(12.0, 12.0), Point(18.0, 18.0));
        let l2 = Line(Point(12.0, 18.0), Point(18.0, 12.0));
        let l3 = Line(Point(18.0, 18.0), Point(12.0, 12.0));

        assert_eq!(l1.intersect(rect), Some(l1));
        assert_eq!(l2.intersect(rect), Some(l2));
        assert_eq!(l3.intersect(rect), Some(l3));
    }

    #[test]
    fn it_should_clip_lines_from_the_left() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(7.0, 11.0), Point(16.0, 17.0));
        assert_eq!(
            line.intersect(rect),
            Some(Line(Point(10.0, 13.0), Point(16.0, 17.0)))
        );
    }

    #[test]
    fn it_should_clip_lines_from_the_bottom_right() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(26.0, 29.0), Point(18.0, 17.0));

        assert_eq!(
            line.intersect(rect),
            Some(Line(Point(20.0, 20.0), Point(18.0, 17.0))),
        );
    }

    #[test]
    fn it_should_clip_lines_fully_crossing() {
        let rect = Rect(Point(10.0, 10.0), Size(8.0, 9.0));
        let line = Line(Point(12.0, 7.0), Point(24.0, 25.0));

        assert_eq!(
            line.intersect(rect),
            Some(Line(Point(14.0, 10.0), Point(18.0, 16.0)))
        );
    }

    // #[test]
    // fn it_should_clip_u32_lines() {
    //     let rect : Rect<u32> = Rect(Point(10, 10), Size(10, 10));
    //     let line : Rect<u32> = Line(Point(7, 11), Point(16, 17));
    //     assert_eq!(
    //         line.intersect(rect),
    //         Some(Line(Point(10, 13), Point(16, 17)))
    //     );
    // }
}
