use crate::num::Num;

use crate::geom::Point;
use crate::geom::Rect;

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
