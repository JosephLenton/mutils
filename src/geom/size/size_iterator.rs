use crate::geom::Point;
use crate::geom::Size;
use crate::num::Num;

#[derive(Clone)]
pub struct SizeIterator<N: Num = f32> {
    pos: Point<N>,
    size: Size<N>,
}

impl<N: Num> SizeIterator<N> {
    pub fn new(size: Size<N>) -> Self {
        Self {
            pos: Point(N::zero(), N::zero()),
            size,
        }
    }
}

impl<N: Num> Iterator for SizeIterator<N> {
    type Item = Point<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x() >= self.size.width() {
            self.pos = Point(N::zero(), self.pos.y() + N::one());
        }

        if self.pos.y() >= self.size.height() {
            return None;
        }

        let r = self.pos;
        self.pos += Point(N::one(), N::zero());
        Some(r)
    }
}

#[cfg(test)]
mod iterating {
    use super::*;

    #[test]
    fn it_should_iterate_over_size() {
        let size: Size<usize> = Size(6, 9);
        let mut ps: Vec<Point<usize>> = vec![];

        for pos in size {
            ps.push(pos);
        }

        let mut i = 0;
        for y in 0..size.height() {
            for x in 0..size.width() {
                assert_eq!(ps[i], Point(x, y));
                i += 1;
            }
        }
    }
}
