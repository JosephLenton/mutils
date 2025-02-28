use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::num::FromRounded;
use crate::num::Num;
use crate::num::ToRounded;

use crate::geom::Line;
use crate::geom::Point;
use crate::geom::Size;

mod circle_circumference_points_iterator;
pub use self::circle_circumference_points_iterator::*;

mod circle_circumference_lines_iterator;
pub use self::circle_circumference_lines_iterator::*;

#[derive(Copy, Clone, Debug)]
pub struct Circle<N: Num = f32>(pub Point<N>, pub N);

impl<N: Num> Circle<N> {
    pub fn new_zero_value() -> Self {
        Circle(Point::new_zero_value(), N::zero())
    }

    pub fn with_radius(centre_point: Point<N>, radius: N) -> Self {
        Self(centre_point, radius)
    }

    pub fn with_diameter(centre_point: Point<N>, diameter: N) -> Self {
        Self::with_radius(centre_point, diameter / (N::one() + N::one()))
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
        self.radius()
    }

    pub fn height(&self) -> N {
        self.radius()
    }

    pub fn radius(&self) -> N {
        self.1
    }

    pub fn radius_sqrd(&self) -> N {
        self.radius() * self.radius()
    }

    pub fn to_size(&self) -> Size<N> {
        Size(self.width(), self.height())
    }

    pub fn centre(&self) -> Point<N> {
        self.0
    }

    pub fn mult_radius(&mut self, n: N) {
        self.1 *= n;
    }

    pub fn div_radius(&mut self, n: N) {
        self.1 /= n;
    }

    pub fn add_radius(&mut self, n: N) {
        self.1 += n;
    }

    pub fn sub_radius(&mut self, n: N) {
        self.1 -= n;
    }

    pub fn abs(&self) -> Self {
        Self(self.centre().abs(), self.radius().abs())
    }

    pub fn min(self, other: Self) -> Self {
        Self(
            self.centre().min(other.centre()),
            self.radius().min(other.radius()),
        )
    }

    pub fn max(self, other: Self) -> Self {
        Self(
            self.centre().max(other.centre()),
            self.radius().max(other.radius()),
        )
    }

    pub fn overlaps(&self, other: Self) -> bool {
        let self_rounded = self.to_rounded();
        let other_rounded = other.to_rounded();

        let distance = self_rounded.centre().hypot_to(other_rounded.centre());
        let radius_distance = self_rounded.radius().abs() + other_rounded.radius().abs();

        distance < radius_distance
    }

    pub fn contains_point(&self, point: Point<N>) -> bool {
        self.centre().hypot_to(point) <= self.radius()
    }

    pub fn iter_circumference_points(self, num_points: usize) -> CircleCircumferencePointsIterator {
        CircleCircumferencePointsIterator::new(self, num_points)
    }

    pub fn iter_circumference_lines(self, num_lines: usize) -> CircleCircumferenceLinesIterator {
        CircleCircumferenceLinesIterator::new(self, num_lines)
    }

    pub fn rotate_around_zero(self, rotation: f32) -> Self {
        Circle(self.centre().rotate_around_zero(rotation), self.radius())
    }

    pub fn rotate_around_point(self, rotation: f32, point: Point<N>) -> Self {
        Circle(
            self.centre().rotate_around_point(rotation, point),
            self.radius(),
        )
    }

    pub fn overlaps_line(self, other: Line<N>) -> bool {
        self.distance_to_line(other) <= self.to_f32().radius()
    }

    fn distance_to_line(self, line: Line<N>) -> f32 {
        let self_f32 = self.to_f32();
        let line_f32 = line.to_f32();

        let v1 = line_f32.diff().to_point();
        let mut v2 = line_f32.start().distance_to(self_f32.centre()).to_point();
        let u = (v2.x() * v1.x() + v2.y() * v1.y()) / (v1.y() * v1.y() + v1.x() * v1.x());

        if u >= 0.0 && u <= 1.0 {
            let mut x = (v1.x() * u + line_f32.start().x()) - self_f32.centre().x();
            let mut y = (v1.y() * u + line_f32.start().y()) - self_f32.centre().y();
            x *= x;
            y *= y;

            return (y + x).sqrt(); // return distance from line
        }

        // get distance from end points
        let mut x = self_f32.centre().x() - line_f32.end().x();
        let mut y = self_f32.centre().y() - line_f32.end().y();
        x *= x;
        y *= y;

        v2 *= v2;

        let first = (v2.y() + v2.x()).sqrt();
        let second = (y + x).sqrt();
        return first.min(second); // return smaller of two distances as the result
    }

    #[allow(dead_code)]
    pub(crate) fn to_f32(self) -> Circle<f32> {
        self.to_rounded()
    }
}

impl Circle<f32> {
    #[allow(dead_code)]
    pub(crate) fn from_f32<N: Num>(self) -> Circle<N> {
        Circle(
            self.centre().from_f32(),
            FromRounded::from_rounded(self.radius()),
        )
    }
}

impl<O: Num, N: Num + ToRounded<O>> ToRounded<Circle<O>> for Circle<N> {
    fn to_rounded(self) -> Circle<O> {
        Circle(self.centre().to_rounded(), self.radius().to_rounded())
    }
}

impl<N: Num> Add<Point<N>> for Circle<N> {
    type Output = Self;

    fn add(self, other: Point<N>) -> Self {
        Circle(self.centre() + other, self.radius())
    }
}

impl<N: Num> AddAssign<Point<N>> for Circle<N> {
    fn add_assign(&mut self, other: Point<N>) {
        self.0 += other;
    }
}

impl<N: Num> Sub<Point<N>> for Circle<N> {
    type Output = Self;

    fn sub(self, other: Point<N>) -> Self {
        Circle(self.centre() - other, self.radius())
    }
}

impl<N: Num> SubAssign<Point<N>> for Circle<N> {
    fn sub_assign(&mut self, other: Point<N>) {
        self.0 -= other;
    }
}

impl<N: Num> PartialEq for Circle<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
mod overlaps {
    use super::*;

    #[test]
    fn it_should_overlap_with_another_circle_when_negative_radius_used() {
        let a = Circle(Point(288.0, 179.0), 44.0);
        let b = Circle(Point(341.0, 196.0), -148.0);

        assert_eq!(a.overlaps(b), true);
        assert_eq!(b.overlaps(a), true);
    }

    #[test]
    fn it_should_overlap_with_another_circle_when_within_its_radius() {
        let a = Circle(Point(10, 10), 5);
        let b = Circle(Point(14, 14), 2);

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn it_should_overlap_with_another_circle_when_within_their_radius() {
        let a = Circle(Point(10, 10), 2);
        let b = Circle(Point(14, 14), 5);

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn it_should_not_overlap_with_another_circle_when_combined_radius_too_small() {
        let a = Circle(Point(10, 10), 2);
        let b = Circle(Point(15, 15), 2);

        assert_eq!(a.overlaps(b), false);
    }
}

#[cfg(test)]
mod overlaps_line {
    use super::*;

    #[test]
    fn it_should_not_overlap_line_that_doesnt_cut() {
        let circle: Circle<i32> = Circle(Point(10, 10), 4);
        let line = Line(Point(0, 10), Point(10, 15));
        assert_eq!(circle.overlaps_line(line), false);
    }

    #[test]
    fn it_should_overlap_line_cutting_through() {
        let circle: Circle<i32> = Circle(Point(10, 10), 5);
        let line = Line(Point(0, 10), Point(20, 10));
        assert!(circle.overlaps_line(line));
    }

    #[test]
    fn it_should_overlap_line_that_starts_in_circle() {
        let circle: Circle<i32> = Circle(Point(10, 10), 5);
        let line = Line(Point(9, 10), Point(20, 10));
        assert!(circle.overlaps_line(line));
    }

    #[test]
    fn it_should_overlap_line_that_end_in_circle() {
        let circle: Circle<i32> = Circle(Point(10, 10), 5);
        let line = Line(Point(0, 10), Point(11, 10));
        assert!(circle.overlaps_line(line));
    }

    #[test]
    fn it_should_overlap_line_that_end_and_starts_in_circle() {
        let circle: Circle<i32> = Circle(Point(10, 10), 5);
        let line = Line(Point(9, 10), Point(11, 10));
        assert!(circle.overlaps_line(line));
    }

    #[test]
    fn it_should_overlap_line_that_end_and_starts_in_circle_in_opposite_direction() {
        let circle: Circle<i32> = Circle(Point(10, 10), 5);
        let line = Line(Point(11, 10), Point(9, 10));
        assert!(circle.overlaps_line(line));
    }

    #[test]
    fn it_should_not_overlap_an_infinite_line() {
        let circle: Circle<f32> = Circle(Point(10.0, 10.0), 5.0);
        let line: Line<f32> = Line(Point(10.0, 20.0), Point(10.0, 30.0));
        assert_eq!(false, circle.overlaps_line(line));
    }
}
