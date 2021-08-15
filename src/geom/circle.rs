use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::num::Num;
use crate::num::ToRounded;

use crate::geom::Point;
use crate::geom::Size;

#[derive(Copy, Clone, Debug)]
pub struct Circle<N: Num = f32>(pub Point<N>, pub N);

impl<N: Num> Circle<N> {
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
}

impl<N> Circle<N>
where
    N: Num + ToRounded<f32>,
    f32: ToRounded<N>,
{
    pub fn overlaps(&self, other: Self) -> bool {
        let self_rounded = self.to_rounded();
        let other_rounded = other.to_rounded();

        let distance = self_rounded
            .centre()
            .line_to(other_rounded.centre())
            .hypot();
        let radius_distance = self_rounded.radius() + other_rounded.radius();

        distance < radius_distance
    }

    pub fn contains_point(&self, point: Point<N>) -> bool {
        self.centre().line_to(point).hypot() <= self.radius()
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
