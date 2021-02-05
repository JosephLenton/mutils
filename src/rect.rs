use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::iter::IntoIterator;

use super::internal::FromClamped;
use super::internal::Num;
use super::NumTuple;

use super::Point;
use super::Size;

#[derive(Copy, Clone, Debug)]
pub struct Rect<N: Num = f32>(pub Point<N>, pub Size<N>);

impl<N: Num> Rect<N> {
    pub fn new_from_raw(bottom_left_x: N, bottom_left_y: N, width: N, height: N) -> Self {
        Self::new(Point(bottom_left_x, bottom_left_y), Size(width, height))
    }

    pub fn new(bottom_left: Point<N>, size: Size<N>) -> Self {
        Self(bottom_left, size)
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
        self.1
    }

    pub fn bottom_left(&self) -> Point<N> {
        self.0
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

    pub fn top_right(&self) -> Point<N> {
        self.bottom_left() + self.size()
    }

    pub fn centre(&self) -> Point<N> {
        self.bottom_left() + self.size().half()
    }

    pub fn set_bottom_left(&mut self, xy: Point<N>) {
        self.0 = xy
    }

    pub fn set_centre(&mut self, xy: Point<N>) {
        self.0 = xy - self.size().half()
    }

    pub fn overlaps(&self, other: Self) -> bool {
        let bottom_left_a = self.bottom_left();
        let bottom_left_b = other.bottom_left();

        let top_right_a = self.top_right();
        let top_right_b = other.top_right();

        if top_right_b.x() < bottom_left_a.x() {
            return false;
        }

        if top_right_b.y() < bottom_left_a.y() {
            return false;
        }

        if top_right_a.x() < bottom_left_b.x() {
            return false;
        }

        if top_right_a.y() < bottom_left_b.y() {
            return false;
        }

        true
    }

    pub fn contains(&self, point: Point<N>) -> bool {
        let bottom_left = self.bottom_left();
        let top_right = self.top_right();

        if point.x() < bottom_left.x() {
            return false;
        }

        if point.y() < bottom_left.y() {
            return false;
        }

        if top_right.x() < point.x() {
            return false;
        }

        if top_right.y() < point.y() {
            return false;
        }

        true
    }

    pub fn intersect(&self, other: Self) -> Option<Self> {
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
}

impl<N: Num> Rect<N> {
    pub fn to_clamped<T: Num + FromClamped<N>>(&self) -> Rect<T> {
        Rect(self.bottom_left().to_clamped(), self.size().to_clamped())
    }

    pub fn to<T: Num + From<N>>(&self) -> Rect<T> {
        Rect(self.bottom_left().to(), self.size().to())
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

impl<N: Num> PartialEq for Rect<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<N: Num> IntoIterator for Rect<N> {
    type Item = Point<N>;
    type IntoIter = RectIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        RectIter::new(self)
    }
}

pub struct RectIter<N: Num = f32> {
    bottom_left: Point<N>,
    top_right: Point<N>,
    current: Point<N>,
}

impl<N: Num> RectIter<N> {
    fn new(rect: Rect<N>) -> Self {
        Self {
            bottom_left: rect.bottom_left(),
            top_right: rect.top_right(),
            current: rect.bottom_left(),
        }
    }
}

impl<N: Num> Iterator for RectIter<N> {
    type Item = Point<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.top_right.x() <= self.current.x() {
            self.current = Point( self.bottom_left.x(), self.current.y() + N::one() );
        }

        if self.top_right.y() <= self.current.y() {
            return None;
        }

        let r = self.current;
        self.current += Point(N::one(), N::zero());
        Some(r)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rect_overlap_outside_left() {
        let a = Rect(Point(20, 0), Size(10, 10));
        let b = Rect(Point(0, 0), Size(10, 10));

        assert_eq!(a.overlaps(b), false);
    }

    #[test]
    fn rect_overlap_outside_right() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(20, 0), Size(10, 10));

        assert_eq!(a.overlaps(b), false);
    }

    #[test]
    fn rect_overlap_inside_left() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(-5, 0), Size(10, 10));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn rect_overlap_inside_left_above() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(-5, 5), Size(10, 10));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn rect_overlap_inside_right_below() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(5, -5), Size(10, 10));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn rect_overlap_inside() {
        let a = Rect(Point(0, 0), Size(10, 10));
        let b = Rect(Point(3, 3), Size(6, 6));

        assert_eq!(a.overlaps(b), true);
    }

    #[test]
    fn it_should_not_iterate_empty_rect() {
        let mut xs = vec![];
        let mut ys = vec![];

        for Point(x, y) in Rect(Point(2, 3), Size(0, 0)) {
            xs.push(x);
            ys.push(y);
        }

        assert_eq!(xs, []);
        assert_eq!(ys, []);
    }

    #[test]
    fn it_should_iterate_over_rect() {
        let mut xs = vec![];
        let mut ys = vec![];

        for Point(x, y) in Rect(Point(2, 3), Size(3, 4)) {
            xs.push(x);
            ys.push(y);
        }

        #[rustfmt::skip]
        assert_eq!(xs, [
            2, 3, 4,
            2, 3, 4,
            2, 3, 4,
            2, 3, 4,
        ]);

        #[rustfmt::skip]
        assert_eq!(ys, [
            3, 3, 3,
            4, 4, 4,
            5, 5, 5,
            6, 6, 6,
        ]);
    }

    #[test]
    fn it_should_iterate_over_rect_usize() {
        let mut xs = vec![];
        let mut ys = vec![];

        for Point(x, y) in Rect(Point(0, 0), Size(2, 2)) {
            xs.push(x);
            ys.push(y);
        }

        #[rustfmt::skip]
        assert_eq!(xs, [
            0, 1,
            0, 1,
        ]);

        #[rustfmt::skip]
        assert_eq!(ys, [
            0, 0,
            1, 1,
        ]);
    }
}
