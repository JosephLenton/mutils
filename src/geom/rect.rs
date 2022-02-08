use ::std::ops::Shl;
use ::std::ops::ShlAssign;
use ::std::ops::Shr;
use ::std::ops::ShrAssign;
use std::iter::IntoIterator;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::num::INum;
use crate::num::Num;
use crate::num::ToRounded;

use crate::geom::HorizontalPosition;
use crate::geom::Line;
use crate::geom::LinePosition;
use crate::geom::Point;
use crate::geom::PointPosition;
use crate::geom::Size;
use crate::geom::VerticalPosition;

mod rect_iterator;
pub use self::rect_iterator::RectIterator;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect<N: Num = f32>(pub Point<N>, pub Size<N>);

impl<N: Num> Rect<N> {
    pub fn new_zero_value() -> Self {
        Rect(Point::new_zero_value(), Size::new_zero_value())
    }

    pub fn new_from_raw(bottom_left_x: N, bottom_left_y: N, width: N, height: N) -> Self {
        Self::new(Point(bottom_left_x, bottom_left_y), Size(width, height))
    }

    pub fn new(bottom_left: Point<N>, size: Size<N>) -> Self {
        Self(bottom_left, size)
    }

    pub fn new_from_centre(centre: Point<N>, size: Size<N>) -> Self {
        Self(centre - size.half(), size)
    }

    pub fn move_xy(&mut self, xy: Point<N>) {
        self.0 += xy;
    }

    pub fn move_x(&mut self, x: N) {
        self.0.move_x(x);
    }

    pub fn move_y(&mut self, y: N) {
        self.0.move_y(y);
    }

    pub fn width(&self) -> N {
        self.size().width()
    }

    pub fn height(&self) -> N {
        self.size().height()
    }

    pub fn area(&self) -> N {
        self.size().area()
    }

    pub fn size(&self) -> Size<N> {
        self.1.abs()
    }

    pub fn bottom_left(&self) -> Point<N> {
        self.0 + self.1.min(Size(N::zero(), N::zero()))
    }

    pub fn bottom_right(&self) -> Point<N> {
        self.bottom_left() + Size(self.width(), N::zero())
    }

    pub fn top_left(&self) -> Point<N> {
        self.bottom_left() + Size(N::zero(), self.height())
    }

    pub fn top_right(&self) -> Point<N> {
        self.bottom_left() + self.size()
    }

    pub fn top_y(&self) -> N {
        self.bottom_left().y() + self.size().height()
    }

    pub fn bottom_y(&self) -> N {
        self.bottom_left().y()
    }

    pub fn left_x(&self) -> N {
        self.bottom_left().x()
    }

    pub fn right_x(&self) -> N {
        self.bottom_left().x() + self.size().width()
    }

    pub fn centre(&self) -> Point<N> {
        self.bottom_left() + self.size().half()
    }

    pub fn set_bottom_left(&mut self, xy: Point<N>) {
        self.0 = xy;
    }

    pub fn set_centre(&mut self, xy: Point<N>) {
        self.0 = xy - self.size().half();
    }

    pub fn set_size(&mut self, size: Size<N>) {
        self.1 = size;
    }

    pub fn overlaps(&self, other: Self) -> bool {
        let bottom_left_a = self.bottom_left();
        let bottom_left_b = other.bottom_left();

        let top_right_a = self.top_right();
        let top_right_b = other.top_right();

        if top_right_b.x() <= bottom_left_a.x() {
            return false;
        }

        if top_right_b.y() <= bottom_left_a.y() {
            return false;
        }

        if top_right_a.x() <= bottom_left_b.x() {
            return false;
        }

        if top_right_a.y() <= bottom_left_b.y() {
            return false;
        }

        true
    }

    pub fn contains_point(&self, point: Point<N>) -> bool {
        self.point_horizontal_position(point) == HorizontalPosition::Inside
            && self.point_vertical_position(point) == VerticalPosition::Inside
    }

    pub fn line_position(&self, other: Line<N>) -> LinePosition {
        LinePosition(
            self.point_position(other.start()),
            self.point_position(other.end()),
        )
    }

    pub fn point_position(&self, other: Point<N>) -> PointPosition {
        PointPosition(
            self.point_horizontal_position(other),
            self.point_vertical_position(other),
        )
    }

    pub fn point_horizontal_position(&self, other: Point<N>) -> HorizontalPosition {
        if other.x() < self.left_x() {
            HorizontalPosition::Left
        } else if other.x() > self.right_x() {
            HorizontalPosition::Right
        } else {
            HorizontalPosition::Inside
        }
    }

    pub fn point_vertical_position(&self, other: Point<N>) -> VerticalPosition {
        if other.y() < self.bottom_y() {
            VerticalPosition::Below
        } else if other.y() > self.top_y() {
            VerticalPosition::Above
        } else {
            VerticalPosition::Inside
        }
    }

    pub fn intersect_rect(&self, other: Self) -> Option<Self> {
        if !self.overlaps(other) {
            return None;
        }

        let new_bottom_left = self.bottom_left().max(other.bottom_left());
        let new_top_right = self.top_right().min(other.top_right());
        let new_size = new_bottom_left.distance_to(new_top_right);
        let new_rect = Rect(new_bottom_left, new_size);

        Some(new_rect)
    }

    pub fn combine(&self, other: Self) -> Self {
        let min_xy = self.bottom_left().min(other.bottom_left());
        let max_xy = self.top_right().max(other.top_right());

        min_xy.rect_to(max_xy)
    }

    pub fn get_scale_diff(&self, other: Self) -> Size<N> {
        self.size().get_scale_diff(other.size())
    }

    pub fn to<T: Num + From<N>>(&self) -> Rect<T> {
        Rect(self.bottom_left().to(), self.size().to())
    }

    pub fn min(self, other: Self) -> Self {
        Self(
            self.bottom_left().min(other.bottom_left()),
            self.size().min(other.size()),
        )
    }

    pub fn max(self, other: Self) -> Self {
        Self(
            self.bottom_left().max(other.bottom_left()),
            self.size().max(other.size()),
        )
    }

    pub(crate) fn to_f32(self) -> Rect<f32> {
        self.to_rounded()
    }

    pub fn centre_to(self, other: Rect<N>) -> Line<N> {
        Line(self.centre(), other.centre())
    }

    pub fn interpolate_to(self, other: Rect<N>, n: N) -> Rect<N> {
        let new_centre = self.centre().interpolate_to(other.centre(), n);
        let new_size = self.size().interpolate_to(other.size(), n);

        Rect::new_from_centre(new_centre, new_size)
    }

    pub fn round_to_max_size(self) -> Rect<N> {
        let self_f32 = self.to_f32();
        let Point(bottom_left_x, bottom_left_y) = self_f32.bottom_left();
        let Point(top_right_x, top_right_y) = self_f32.top_right();

        let new_bottom_left_x;
        let new_bottom_left_y;
        let new_top_right_x;
        let new_top_right_y;

        if bottom_left_x < top_right_x {
            new_bottom_left_x = bottom_left_x.floor();
            new_top_right_x = top_right_x.ceil();
        } else {
            new_bottom_left_x = bottom_left_x.floor();
            new_top_right_x = top_right_x.ceil();
        }

        if bottom_left_y < top_right_y {
            new_bottom_left_y = bottom_left_y.floor();
            new_top_right_y = top_right_y.ceil();
        } else {
            new_bottom_left_y = bottom_left_y.floor();
            new_top_right_y = top_right_y.ceil();
        }

        let new_rect_f32 = Point(new_bottom_left_x, new_bottom_left_y).rect_to(Point(new_top_right_x, new_top_right_y));
        new_rect_f32.from_f32()
    }
}

impl Rect<f32> {
    #[allow(dead_code)]
    pub(crate) fn from_f32<N: Num>(self) -> Rect<N> {
        Rect(self.bottom_left().from_f32(), self.size().from_f32())
    }
}

impl<O: Num, N: Num + ToRounded<O>> ToRounded<Rect<O>> for Rect<N> {
    fn to_rounded(self) -> Rect<O> {
        Rect(self.bottom_left().to_rounded(), self.size().to_rounded())
    }
}

impl<N: Num> Add<Point<N>> for Rect<N> {
    type Output = Self;

    fn add(self, other: Point<N>) -> Self {
        Rect(self.bottom_left() + other, self.size())
    }
}

impl<N: Num> AddAssign<Point<N>> for Rect<N> {
    fn add_assign(&mut self, other: Point<N>) {
        self.0 += other;
    }
}

impl<N: Num> Add<Size<N>> for Rect<N> {
    type Output = Self;

    fn add(self, other: Size<N>) -> Self {
        Rect(self.bottom_left(), self.size() + other)
    }
}

impl<N: Num> AddAssign<Size<N>> for Rect<N> {
    fn add_assign(&mut self, other: Size<N>) {
        self.1 += other;
    }
}

impl<N: Num> Sub<Point<N>> for Rect<N> {
    type Output = Self;

    fn sub(self, other: Point<N>) -> Self {
        Rect(self.bottom_left() - other, self.size())
    }
}

impl<N: Num> SubAssign<Point<N>> for Rect<N> {
    fn sub_assign(&mut self, other: Point<N>) {
        self.0 -= other;
    }
}

impl<N: Num> Sub<Size<N>> for Rect<N> {
    type Output = Self;

    fn sub(self, other: Size<N>) -> Self {
        Rect(self.bottom_left(), self.size() - other)
    }
}

impl<N: Num> SubAssign<Size<N>> for Rect<N> {
    fn sub_assign(&mut self, other: Size<N>) {
        self.1 -= other;
    }
}

impl<N: INum> Shl<N> for Rect<N> {
    type Output = Self;

    fn shl(self, other: N) -> Self {
        Self(self.0 << other, self.1 << other)
    }
}

impl<N: INum> ShlAssign<N> for Rect<N> {
    fn shl_assign(&mut self, other: N) {
        *self = *self << other;
    }
}

impl<N: INum> Shr<N> for Rect<N> {
    type Output = Self;

    fn shr(self, other: N) -> Self {
        Self(self.0 >> other, self.1 >> other)
    }
}

impl<N: INum> ShrAssign<N> for Rect<N> {
    fn shr_assign(&mut self, other: N) {
        *self = *self >> other;
    }
}

impl<N: Num> IntoIterator for Rect<N> {
    type Item = Point<N>;
    type IntoIter = RectIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        RectIterator::new(self)
    }
}

#[cfg(test)]
mod overlaps {
    use super::*;

    #[test]
    fn it_should_not_overlap_with_rectangles_outside_to_the_left() {
        let a = Rect(Point(20, 0), Size(10, 10));
        let b = Rect(Point(0, 0), Size(10, 10));

        assert_eq!(a.overlaps(b), false);
    }

    #[test]
    fn it_should_not_overlap_with_rectangles_outside_to_the_right() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(20, 0), Size(10, 10));

        assert_eq!(a.overlaps(b), false);
    }

    #[test]
    fn it_should_not_overlap_with_rectangles_outside_above() {
        let a = Rect(Point(20, 20), Size(10, 10));
        let b = Rect(Point(20, 0), Size(10, 10));

        assert_eq!(a.overlaps(b), false);
    }

    #[test]
    fn it_should_not_overlap_with_rectangles_outside_below() {
        let a = Rect(Point(20, 20), Size(10, 10));
        let b = Rect(Point(20, 40), Size(10, 10));

        assert_eq!(a.overlaps(b), false);
    }

    #[test]
    fn it_should_overlap_with_rectangles_intersecting_on_the_left() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(-5, 0), Size(10, 10));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn it_should_overlap_with_rectangles_intersecting_left_above() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(-5, 5), Size(10, 10));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn it_should_overlap_with_rectangles_intersecting_right_below() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(5, -5), Size(10, 10));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn it_should_overlap_with_rectangles_intersecting_fully_inside() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(3, 3), Size(6, 6));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn it_should_overlap_identical_rectangles() {
        let a: Rect<i32> = Rect(Point(3, 2), Size(4, 5));
        let b: Rect<i32> = Rect(Point(3, 2), Size(4, 5));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn it_should_not_overlap_rectangles_next_to_each_other() {
        let a: Rect<i32> = Rect(Point(2, 2), Size(2, 2));
        let b: Rect<i32> = Rect(Point(4, 2), Size(2, 2));

        assert_eq!(a.overlaps(b), false);
    }

    #[test]
    fn it_should_overlap_with_negative_sizes() {
        let a: Rect<i32> = Rect(Point(0, 0), Size(10, 10));
        let b: Rect<i32> = Rect(Point(12, 12), Size(-10, -10));

        assert_eq!(a.overlaps(b), true);
        assert_eq!(b.overlaps(a), true);
    }
}

#[cfg(test)]
mod new_from_centre {
    use super::*;

    #[test]
    fn it_should_create_a_rectangle_around_the_point_given() {
        let rect: Rect<f32> = Rect::new_from_centre(Point(10.0, 10.0), Size(5.0, 8.0));
        assert_eq!(rect, Rect(Point(7.5, 6.0), Size(5.0, 8.0)));
    }
}

#[cfg(test)]
mod bottom_left {
    use super::*;

    #[test]
    fn it_should_return_bottom_left() {
        let rect: Rect<u32> = Rect(Point(3, 4), Size(9, 13));
        assert_eq!(rect.bottom_left(), Point(3, 4));
    }

    #[test]
    fn it_should_return_bottom_left_for_positive_sizes() {
        let rect: Rect<i32> = Rect(Point(10, 12), Size(5, 8));
        assert_eq!(rect.bottom_left(), Point(10, 12));
    }

    #[test]
    fn it_should_account_for_negative_sizes() {
        let rect: Rect<i32> = Rect(Point(10, 12), Size(-5, -8));
        assert_eq!(rect.bottom_left(), Point(5, 4));
    }
}

#[cfg(test)]
mod bottom_right {
    use super::*;

    #[test]
    fn it_should_return_bottom_right() {
        let rect: Rect<u32> = Rect(Point(3, 4), Size(9, 13));
        assert_eq!(rect.bottom_right(), Point(12, 4));
    }
}

#[cfg(test)]
mod top_left {
    use super::*;

    #[test]
    fn it_should_return_top_left() {
        let rect: Rect<u32> = Rect(Point(3, 4), Size(9, 13));
        assert_eq!(rect.top_left(), Point(3, 17));
    }
}

#[cfg(test)]
mod top_right {
    use super::*;

    #[test]
    fn it_should_return_top_right() {
        let rect: Rect<u32> = Rect(Point(3, 4), Size(9, 13));
        assert_eq!(rect.top_right(), Point(12, 17));
    }
}

#[cfg(test)]
mod point_position {
    use super::*;

    #[test]
    fn it_should_return_bottom_left() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(3, 4)),
            PointPosition(HorizontalPosition::Left, VerticalPosition::Below)
        );
    }

    #[test]
    fn it_should_return_middle_left() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(3, 16)),
            PointPosition(HorizontalPosition::Left, VerticalPosition::Inside)
        );
    }

    #[test]
    fn it_should_return_above_left() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(3, 24)),
            PointPosition(HorizontalPosition::Left, VerticalPosition::Above)
        );
    }

    #[test]
    fn it_should_return_bottom_middle() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(13, 4)),
            PointPosition(HorizontalPosition::Inside, VerticalPosition::Below)
        );
    }

    #[test]
    fn it_should_return_middle_middle() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(13, 14)),
            PointPosition(HorizontalPosition::Inside, VerticalPosition::Inside)
        );
    }

    #[test]
    fn it_should_return_above_middle() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(13, 24)),
            PointPosition(HorizontalPosition::Inside, VerticalPosition::Above)
        );
    }

    #[test]
    fn it_should_return_bottom_right() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(30, 4)),
            PointPosition(HorizontalPosition::Right, VerticalPosition::Below)
        );
    }

    #[test]
    fn it_should_return_middle_right() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(30, 14)),
            PointPosition(HorizontalPosition::Right, VerticalPosition::Inside)
        );
    }

    #[test]
    fn it_should_return_above_right() {
        let rect: Rect<u32> = Rect(Point(10, 12), Size(10, 8));
        assert_eq!(
            rect.point_position(Point(30, 34)),
            PointPosition(HorizontalPosition::Right, VerticalPosition::Above)
        );
    }
}

#[cfg(test)]
mod centre_to {
    use super::*;

    #[test]
    fn it_should_return_line_from_first_rect_to_second() {
        let first_rect = Rect::new_from_centre(Point(24_i32, 123_i32), Size(100_i32, 200_i32));
        let second_rect = Rect::new_from_centre(Point(999_i32, -283_i32), Size(100_i32, 200_i32));
        let line = first_rect.centre_to(second_rect);

        assert_eq!(line, Line(Point(24_i32, 123_i32), Point(999_i32, -283_i32)));
    }
}

#[cfg(test)]
mod interpolate_to {
    use super::*;

    #[test]
    fn it_should_interpolate_centre() {
        let first_rect: Rect<f32> = Rect::new_from_centre(Point(100.0, 200.0), Size(100.0, 200.0));
        let second_rect: Rect<f32> =
            Rect::new_from_centre(Point(1100.0, -400.0), Size(100.0, 200.0));
        let interpolated_rect: Rect<f32> = first_rect.interpolate_to(second_rect, 0.5);

        assert_eq!(
            interpolated_rect,
            Rect::new_from_centre(Point(600.0, -100.0), Size(100.0, 200.0))
        );
    }

    #[test]
    fn it_should_interpolate_size() {
        let first_rect: Rect<f32> = Rect::new_from_centre(Point(100.0, 200.0), Size(100.0, 200.0));
        let second_rect: Rect<f32> =
            Rect::new_from_centre(Point(1100.0, -400.0), Size(200.0, 100.0));
        let interpolated_rect: Rect<f32> = first_rect.interpolate_to(second_rect, 0.5);

        assert_eq!(
            interpolated_rect,
            Rect::new_from_centre(Point(600.0, -100.0), Size(150.0, 150.0))
        );
    }
}

#[cfg(test)]
mod round_to_max_size {
    use super::*;
    use ::testcat::*;
    use crate::geom::testing_utils::assert_approx_rect_eq;

    it!("should enlarge positive rectangles to make size larger", test_positive_rect);
    it!("should enlarge positive rectangles with an inverse size", test_positive_rect_with_negative_size);
    it!("should enlarge negative rectangles to make size larger", test_negative_rect);
    it!("should enlarge negative rectangles with an inverse size", test_negative_rect_with_negative_size);
    it!("should enlarge rectangles that cross zero to make size larger", test_cross_zero_rect);
    it!("should enlarge rectangles that cross zero with an inverse size", test_cross_zero_rect_with_negative_size);

    fn test_positive_rect() {
        let rect : Rect<f32> = Point(10.2, 10.8).rect_to(Point(19.2, 19.8));
        let rounded = rect.round_to_max_size();
        let expected : Rect<f32> = Point(10.0, 10.0).rect_to(Point(20.0, 20.0));

        assert_approx_rect_eq(expected, rounded);
    }

    fn test_positive_rect_with_negative_size() {
        let rect : Rect<f32> = Point(19.2, 19.8).rect_to(Point(10.2, 10.8));
        let rounded = rect.round_to_max_size();
        let expected : Rect<f32> = Point(20.0, 20.0).rect_to(Point(10.0, 10.0));

        assert_approx_rect_eq(expected, rounded);
    }

    fn test_negative_rect() {
        let rect : Rect<f32> = Point(-10.2, -10.8).rect_to(Point(-19.2, -19.8));
        let rounded = rect.round_to_max_size();
        let expected : Rect<f32> = Point(-10.0, -10.0).rect_to(Point(-20.0, -20.0));

        assert_approx_rect_eq(expected, rounded);
    }

    fn test_negative_rect_with_negative_size() {
        let rect : Rect<f32> = Point(-19.2, -19.8).rect_to(Point(-10.2, -10.8));
        let rounded = rect.round_to_max_size();
        let expected : Rect<f32> = Point(-20.0, -20.0).rect_to(Point(-10.0, -10.0));

        assert_approx_rect_eq(expected, rounded);
    }

    fn test_cross_zero_rect() {
        let rect : Rect<f32> = Point(-10.2, -10.8).rect_to(Point(19.2, 19.8));
        let rounded = rect.round_to_max_size();
        let expected : Rect<f32> = Point(-11.0, -11.0).rect_to(Point(20.0, 20.0));

        assert_approx_rect_eq(expected, rounded);
    }

    fn test_cross_zero_rect_with_negative_size() {
        let rect : Rect<f32> = Point(19.2, 19.8).rect_to(Point(-10.2, -10.8));
        let rounded = rect.round_to_max_size();
        let expected : Rect<f32> = Point(20.0, 20.0).rect_to(Point(-11.0, -11.0));

        assert_approx_rect_eq(expected, rounded);
    }
}
