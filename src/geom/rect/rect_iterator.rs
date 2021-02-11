use crate::num::Num;

use crate::geom::Point;
use crate::geom::Rect;

#[derive(Clone)]
pub struct RectIterator<N: Num = f32> {
    bottom_left: Point<N>,
    top_right: Point<N>,
    current: Point<N>,
}

impl<N: Num> RectIterator<N> {
    pub fn new(rect: Rect<N>) -> Self {
        Self {
            bottom_left: rect.bottom_left(),
            top_right: rect.top_right(),
            current: rect.bottom_left(),
        }
    }
}

impl<N: Num> Iterator for RectIterator<N> {
    type Item = Point<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.top_right.x() <= self.current.x() {
            self.current = Point(self.bottom_left.x(), self.current.y() + N::one());
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
mod iterating {
    use super::*;
    use crate::geom::Size;

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
