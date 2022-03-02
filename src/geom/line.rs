use ::std::iter::IntoIterator;
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

use crate::num::FromRounded;
use crate::num::INum;
use crate::num::Num;
use crate::num::NumIdentity;
use crate::num::ToRounded;
use crate::num::ToSignedClamped;

use crate::geom::Circle;
use crate::geom::Point;
use crate::geom::Rect;
use crate::geom::Size;

use crate::geom::PointPosition;
use crate::geom::Transform;

use crate::Random;

mod line_iterator;
pub use self::line_iterator::LineIterator;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line<N: Num = f32>(pub Point<N>, pub Point<N>);

impl<N: Num> Line<N> {
    pub fn new_zero_value() -> Self {
        Line(Point::new_zero_value(), Point::new_zero_value())
    }

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

    /// Where `n` is a valid from 0.0 to 1.0,
    /// this will return a point on the length of the line.
    pub fn interpolation_point(self, n: N) -> Point<N> {
        self.start() + self.diff_as_point() * n
    }

    fn diff_as_point(self) -> Point<N> {
        self.end() - self.start()
    }

    /// This swaps the start and end points.
    pub fn reverse(self) -> Self {
        Self(self.end(), self.start())
    }

    pub fn is_direction_negative_x(self) -> bool {
        self.end().x() < self.start().x()
    }

    pub fn is_direction_positive_x(self) -> bool {
        self.start().x() < self.end().x()
    }

    pub fn is_direction_negative_y(self) -> bool {
        self.end().y() < self.start().y()
    }

    pub fn is_direction_positive_y(self) -> bool {
        self.start().y() < self.end().y()
    }

    pub fn abs(&self) -> Self {
        Self(self.start().abs(), self.end().abs())
    }

    pub fn min(self, other: Self) -> Self {
        Self(self.start().min(other.start()), self.end().min(other.end()))
    }

    pub fn max(self, other: Self) -> Self {
        Self(self.start().max(other.start()), self.end().max(other.end()))
    }

    pub fn min_x(self) -> N {
        self.start().x().min(self.end().x())
    }

    pub fn min_y(self) -> N {
        self.start().y().min(self.end().y())
    }

    pub fn max_x(self) -> N {
        self.start().x().max(self.end().x())
    }

    pub fn max_y(self) -> N {
        self.start().y().max(self.end().y())
    }

    pub fn overlaps_line(self, other: Line<N>) -> bool {
        self.intersect_line(other).is_some()
    }

    pub fn overlaps_circle(self, other: Circle<N>) -> bool {
        other.overlaps_line(self)
    }

    pub fn intersect_line(self, other: Line<N>) -> Option<Point<N>> {
        let self_rounded = self.to_rounded();
        let other_rounded = other.to_rounded();

        let size_1 = self_rounded.diff();
        let size_2 = other_rounded.diff();
        let start_dist = self_rounded.start() - other_rounded.start();

        let s = (-size_1.height() * start_dist.x() + size_1.width() * start_dist.y())
            / (-size_2.width() * size_1.height() + size_1.width() * size_2.height());
        let t = (size_2.width() * start_dist.y() - size_2.height() * start_dist.x())
            / (-size_2.width() * size_1.height() + size_1.width() * size_2.height());

        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            let intersection_x = self_rounded.start().x() + (t * size_1.width());
            let intersection_y = self_rounded.start().y() + (t * size_1.height());
            let intersection = Point(intersection_x, intersection_y).from_f32();
            return Some(intersection);
        }

        None
    }

    /**
     * Calculates if this overlaps another rectangle.
     * If it does, it will return the part of the line that intersects
     * within that rectangle.
     *
     * `None` is returned when no intersection is found.
     */
    pub fn intersect_rect(self, rect: Rect<N>) -> Option<Line<N>> {
        let mut position = rect.line_position(self);

        // No clippin needed.
        if position.is_entirely_inside() {
            return Some(self);
        }

        if position.is_entirely_outside() {
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
        let mut line_f32 = self.to_f32();
        let rect_f32 = rect.to_f32();
        loop {
            // Case 1:
            // both endpoints are within the clipping region
            if position.is_entirely_inside() {
                return Some(line_f32.from_f32());
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
                    if rect_f32.intersect_rect(line_f32.to_rect()) == None {
                        return None;
                    }
                }
            }
        }
    }

    /**
     * Returns the atan2( y dist, x dist )
     */
    pub fn angle(self) -> N {
        let self_f32 = self.to_f32();
        let y_diff = self_f32.y_diff();
        let x_diff = self_f32.x_diff();
        let angle = y_diff.atan2(x_diff);
        FromRounded::from_rounded(angle)
    }

    pub fn direction(self) -> Size<f32> {
        let angle = self.to_f32().angle();
        Size(angle.cos(), angle.sin())
    }

    pub fn hypot(self) -> N {
        self.start().hypot_to(self.end())
    }

    pub fn hypot_sqrd(self) -> N {
        self.start().distance_to(self.end()).hypot_sqrd()
    }

    pub fn rotate(self, angle: f32) -> Self {
        let self_f32 = self.to_f32();
        let midpoint = self_f32.midpoint();
        let half_len = self_f32.hypot() / 2.0;

        let angle = self_f32.angle() - angle;
        let cos = angle.cos();
        let sin = angle.sin();

        let start = midpoint - Point(half_len * cos, half_len * sin);
        let end = midpoint + Point(half_len * cos, half_len * sin);

        Line(start, end).from_f32()
    }

    pub fn rotate_around_zero(self, angle: f32) -> Self {
        Line(
            self.start().rotate_around_zero(angle),
            self.end().rotate_around_zero(angle),
        )
    }

    pub fn rotate_around_point(self, angle: f32, target: Point<N>) -> Self {
        Line(
            self.start().rotate_around_point(angle, target),
            self.end().rotate_around_point(angle, target),
        )
    }

    pub fn rotate_around_start(self, angle: f32) -> Self {
        Line(
            self.start(),
            self.end().rotate_around_point(angle, self.start()),
        )
    }

    pub fn rotate_around_end(self, angle: f32) -> Self {
        Line(
            self.start().rotate_around_point(angle, self.end()),
            self.end(),
        )
    }

    pub fn midpoint(self) -> Point<N> {
        ((self.start() + self.end()).to_rounded() / 2.0).from_f32()
    }

    pub(crate) fn to_f32(self) -> Line<f32> {
        self.to_rounded()
    }

    /// Returns a line where the start is at the end,
    /// and the end is at the start.
    pub fn flip(self) -> Self {
        Self(self.end(), self.start())
    }

    pub fn into_iter_with_step(self, step: N) -> LineIterator<N> {
        LineIterator::new(self, step, true)
    }

    pub fn into_iter_inclusive(self) -> LineIterator<N> {
        LineIterator::new(self, <N as NumIdentity>::one(), false)
    }

    pub fn into_iter_inclusive_with_step(self, step: N) -> LineIterator<N> {
        LineIterator::new(self, step, false)
    }
}

impl<N: Num + Signed> Line<N> {
    pub fn direction_sign(&self) -> Point<N> {
        self.diff_as_point().sign()
    }
}

impl<N: Num + ToSignedClamped> Line<N>
where
    <N as ToSignedClamped>::Output: Signed,
{
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

    pub fn step_direction(self) -> Point<<N as ToSignedClamped>::Output> {
        self.diff().to_point().step()
    }
}

impl Line<f32> {
    pub(crate) fn from_f32<N: Num>(self) -> Line<N> {
        Line(self.start().from_f32(), self.end().from_f32())
    }

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

impl<N> Add<Transform<N>> for Line<N>
where
    N: Num,
{
    type Output = Line<N>;

    #[inline(always)]
    fn add(self, transform: Transform<N>) -> Self::Output {
        transform + self
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

impl<N: Num> IntoIterator for Line<N> {
    type Item = Point<N>;
    type IntoIter = LineIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator::new(self, <N as NumIdentity>::one(), true)
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
        assert_eq!(line.intersect_rect(rect), None);
    }

    #[test]
    fn it_should_ignore_lines_fully_left() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(5.0, 5.0), Point(5.0, 25.0));
        assert_eq!(line.intersect_rect(rect), None);
    }

    #[test]
    fn it_should_ignore_lines_fully_right() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(25.0, 5.0), Point(25.0, 25.0));
        assert_eq!(line.intersect_rect(rect), None);
    }

    #[test]
    fn it_should_ignore_lines_fully_below() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(5.0, 25.0), Point(25.0, 25.0));
        assert_eq!(line.intersect_rect(rect), None);
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
        assert_eq!(line.intersect_rect(rect), None);
    }

    #[test]
    fn it_should_not_clip_lines_fully_inside() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));

        let l1 = Line(Point(12.0, 12.0), Point(18.0, 18.0));
        let l2 = Line(Point(12.0, 18.0), Point(18.0, 12.0));
        let l3 = Line(Point(18.0, 18.0), Point(12.0, 12.0));

        assert_eq!(l1.intersect_rect(rect), Some(l1));
        assert_eq!(l2.intersect_rect(rect), Some(l2));
        assert_eq!(l3.intersect_rect(rect), Some(l3));
    }

    #[test]
    fn it_should_clip_lines_from_the_left() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(7.0, 11.0), Point(16.0, 17.0));

        assert_eq!(
            line.intersect_rect(rect),
            Some(Line(Point(10.0, 13.0), Point(16.0, 17.0)))
        );
    }

    #[test]
    fn it_should_clip_lines_from_the_bottom_right() {
        let rect = Rect(Point(10.0, 10.0), Size(10.0, 10.0));
        let line = Line(Point(26.0, 29.0), Point(18.0, 17.0));

        assert_eq!(
            line.intersect_rect(rect),
            Some(Line(Point(20.0, 20.0), Point(18.0, 17.0))),
        );
    }

    #[test]
    fn it_should_clip_lines_fully_crossing() {
        let rect = Rect(Point(10.0, 10.0), Size(8.0, 9.0));
        let line = Line(Point(12.0, 7.0), Point(24.0, 25.0));

        assert_eq!(
            line.intersect_rect(rect),
            Some(Line(Point(14.0, 10.0), Point(18.0, 16.0)))
        );
    }

    #[test]
    fn it_should_clip_u32_lines() {
        let rect: Rect<u32> = Rect(Point(10, 10), Size(10, 10));
        let line: Line<u32> = Line(Point(7, 11), Point(16, 17));

        assert_eq!(
            line.intersect_rect(rect),
            Some(Line(Point(10, 13), Point(16, 17)))
        );

        let rect: Rect<u32> = Rect(Point(10, 10), Size(8, 9));
        let line: Line<u32> = Line(Point(12, 7), Point(24, 25));

        assert_eq!(
            line.intersect_rect(rect),
            Some(Line(Point(14, 10), Point(18, 16)))
        );
    }
}

#[cfg(test)]
mod interpolation_point {
    use super::*;

    #[test]
    fn it_should_return_start_when_zero() {
        let line = Line(Point(3.0, 4.0), Point(14.0, 19.0));
        assert_eq!(line.interpolation_point(0.0), Point(3.0, 4.0));
    }

    #[test]
    fn it_should_return_end_when_one() {
        let line = Line(Point(3.0, 4.0), Point(14.0, 19.0));
        assert_eq!(line.interpolation_point(1.0), Point(14.0, 19.0));
    }

    #[test]
    fn it_should_return_middle_when_half() {
        let line = Line(Point(3.0, 4.0), Point(14.0, 19.0));
        assert_eq!(line.interpolation_point(0.5), Point(8.5, 11.5));
    }
}

#[cfg(test)]
mod rotate {
    use super::*;
    use ::std::f32::consts::TAU;

    #[test]
    fn it_should_be_level_after_45_rotation() {
        let line = Line(Point(0.0, 0.0), Point(10.0, 10.0));
        let rotation: Line<i32> = line.rotate(TAU / 8.0).to_rounded();
        assert_eq!(rotation, Line(Point(-2, 5), Point(12, 5)));
    }

    #[test]
    fn it_should_be_flipped_with_180_rotation() {
        let line = Line(Point(1.0, 5.0), Point(15.0, 20.0));
        let rotation: Line<i32> = line.rotate(TAU / 2.0).to_rounded();
        assert_eq!(rotation, line.reverse().to_rounded());
    }

    #[test]
    fn it_should_be_the_same_with_360_rotation() {
        let line = Line(Point(0.0, 5.0), Point(15.0, 20.0));
        let rotation: Line<i32> = line.rotate(TAU).to_rounded();
        assert_eq!(rotation, line.to_rounded());
    }
}

#[cfg(test)]
mod rotate_around_point {
    use super::*;
    use ::std::f32::consts::TAU;

    #[test]
    fn it_should_be_rotated_with_90_rotation() {
        let line = Line(Point(0.0, 5.0), Point(5.0, 0.0));
        let rotation: Line<i32> = line
            .rotate_around_point(TAU * 0.25, Point(0.0, 0.0))
            .to_rounded();
        assert_eq!(rotation, Line(Point(5, 0), Point(0, -5)));
    }

    #[test]
    fn it_should_be_flipped_with_180_rotation() {
        let line = Line(Point(0.0, 5.0), Point(5.0, 0.0));
        let rotation: Line<i32> = line
            .rotate_around_point(TAU * 0.5, Point(0.0, 0.0))
            .to_rounded();
        assert_eq!(rotation, Line(Point(0, -5), Point(-5, -0)));
    }

    #[test]
    fn it_should_be_rotated_with_90_rotation_around_point() {
        let line = Line(Point(8.0, 15.0), Point(13.0, 10.0));
        let rotation: Line<i32> = line
            .rotate_around_point(TAU * 0.25, Point(8.0, 10.0))
            .to_rounded();
        assert_eq!(rotation, Line(Point(13, 10), Point(8, 5)));
    }

    #[test]
    fn it_should_be_flipped_with_180_rotation_around_point() {
        let line = Line(Point(8.0, 15.0), Point(13.0, 10.0));
        let rotation: Line<i32> = line
            .rotate_around_point(TAU * 0.5, Point(8.0, 10.0))
            .to_rounded();
        assert_eq!(rotation, Line(Point(8, 5), Point(3, 10)));
    }
}

#[cfg(test)]
mod direction_sign {
    use super::*;

    #[test]
    fn it_should_be_positive_when_end_is_larger() {
        let line = Line(Point(0.0, 0.0), Point(100.0, 100.0));

        assert_eq!(line.direction_sign(), Point(1.0, 1.0));
    }

    #[test]
    fn it_should_be_positive_when_end_is_the_same() {
        let line = Line(Point(0.0, 0.0), Point(0.0, 0.0));

        assert_eq!(line.direction_sign(), Point(1.0, 1.0));
    }

    #[test]
    fn it_should_be_negative_when_end_is_smaller() {
        let line = Line(Point(0.0, 0.0), Point(-100.0, -100.0));

        assert_eq!(line.direction_sign(), Point(-1.0, -1.0));
    }

    #[test]
    fn it_should_be_mix_when_end_is_smaller_and_larger() {
        let line = Line(Point(0.0, 0.0), Point(100.0, -100.0));

        assert_eq!(line.direction_sign(), Point(1.0, -1.0));
    }
}

#[cfg(test)]
mod hypot {
    use super::*;

    #[test]
    fn it_should_be_correct_for_positive() {
        let line = Line(Point(2.0, 2.0), Point(5.0, 6.0));

        assert_eq!(5.0, line.hypot());
    }
}

#[cfg(test)]
mod hypot_sqrd {
    use super::*;

    #[test]
    fn it_should_be_correct_for_positive() {
        let line = Line(Point(2.0, 2.0), Point(5.0, 6.0));

        assert_eq!(25.0, line.hypot_sqrd());
    }
}

#[cfg(test)]
mod intersect_line {
    use super::*;

    #[test]
    fn it_should_intersect_lines() {
        let line_1 = Line(Point(0.0, 0.0), Point(10.0, 10.0));
        let line_2 = Line(Point(0.0, 10.0), Point(10.0, 0.0));

        let intersection = line_1.intersect_line(line_2);
        assert_eq!(intersection, Some(Point(5.0, 5.0)));
    }

    #[test]
    fn it_should_have_intersections_working_in_both_directions() {
        let line_1 = Line(Point(0.0, 0.0), Point(10.0, 10.0));
        let line_2 = Line(Point(0.0, 10.0), Point(10.0, 0.0));

        let intersection_1 = line_1.intersect_line(line_2);
        let intersection_2 = line_1.intersect_line(line_2);
        assert_eq!(intersection_1, intersection_2);
    }

    #[test]
    fn it_should_not_intersect_parallel_lines() {
        let line_1 = Line(Point(0.0, 0.0), Point(10.0, 2.0));
        let line_2 = Line(Point(0.0, 2.0), Point(10.0, 4.0));

        let intersection = line_1.intersect_line(line_2);
        assert_eq!(intersection, None);
    }
}

#[cfg(test)]
mod flip {
    use super::*;

    #[test]
    fn it_should_swap_start_and_end() {
        let line: Line<i32> = Line(Point(123, 456), Point(987, 654));
        assert_eq!(line.flip(), Line(Point(987, 654), Point(123, 456)))
    }
}

#[cfg(test)]
mod step_direction {
    use super::*;

    #[test]
    fn it_should_return_zero_for_no_direction() {
        let point: Point<f32> = Point(0.0, 0.0);
        let line = Line(point, point);

        let step_direction = line.step_direction();
        assert_eq!(Point(0.0_f32, 0.0_f32), step_direction);
    }
}
