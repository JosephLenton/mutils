use crate::geom::BCurve;
use crate::geom::Line;
use crate::geom::Point;

pub struct CurveLinesIterator<'a, const N: usize> {
    curve: &'a BCurve<N>,
    index_increment: f32,
    i: f32,
    current_point: Point,
}

impl<'a, const N: usize> CurveLinesIterator<'a, N> {
    pub fn new(curve: &'a BCurve<N>, num_lines: u32) -> Self {
        Self {
            curve,
            index_increment: (1.0 / num_lines as f32),
            i: 0.0,
            current_point: curve.start(),
        }
    }
}

impl<'a, const N: usize> Iterator for CurveLinesIterator<'a, N> {
    type Item = Line<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 1.0 {
            self.i += self.index_increment;
            let end = self.curve.interpolation_point(self.i);
            let line = Line(self.current_point, end);
            self.current_point = end;

            return Some(line);
        }

        None
    }
}
