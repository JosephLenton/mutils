use crate::geom::Point;
use crate::geom::Rect;
use crate::num::Num;

#[derive(Clone, Debug)]
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
        if self.bottom_left.x() == self.top_right.x() {
            return None;
        }

        if self.bottom_left.y() == self.top_right.y() {
            return None;
        }

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
    use ::testcat::*;

    it!("should not iterate over empty rect", test_empty_rect);
    it!(
        "should not iterate over rect with zero width",
        test_rect_with_no_width
    );
    it!(
        "should not iterate over rect with zero height",
        test_rect_with_no_height
    );
    it!(
        "should return one value for a rect with 1x1 size",
        test_rect_with_size_one
    );
    it!(
        "should return one value for a rect with negative 1x1 size",
        test_rect_with_negative_size_one
    );
    it!(
        "should iterate over rect with width and height",
        test_rect_with_area
    );
    it!(
        "should iterate over rect with width and height, for usize",
        test_rect_with_area_usize
    );

    fn test_empty_rect() {
        let rect: Rect<usize> = Rect(Point(2, 3), Size(0, 0));
        let points: Vec<Point<usize>> = rect.into_iter().collect();

        assert_eq!(points, []);
    }

    fn test_rect_with_no_width() {
        let rect: Rect<usize> = Rect(Point(2, 3), Size(0, 4));
        let points: Vec<Point<usize>> = rect.into_iter().collect();

        assert_eq!(points, []);
    }

    fn test_rect_with_no_height() {
        let rect: Rect<usize> = Rect(Point(2, 3), Size(3, 0));
        let points: Vec<Point<usize>> = rect.into_iter().collect();

        assert_eq!(points, []);
    }

    fn test_rect_with_size_one() {
        let rect: Rect<usize> = Rect(Point(2, 3), Size(1, 1));
        let points: Vec<Point<usize>> = rect.into_iter().collect();

        assert_eq!(points, [Point(2, 3)]);
    }

    fn test_rect_with_negative_size_one() {
        let rect: Rect<isize> = Rect(Point(2, 3), Size(-1, -1));
        let points: Vec<Point<isize>> = rect.into_iter().collect();

        assert_eq!(points, [Point(1, 2)]);
    }

    fn test_rect_with_area() {
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

    fn test_rect_with_area_usize() {
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
