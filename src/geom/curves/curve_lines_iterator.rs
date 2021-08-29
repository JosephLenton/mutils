use crate::geom::BCurve;
use crate::geom::Line;

pub struct CurveLinesIterator<'a, const N: usize> {
    curve: &'a BCurve<N>,
    num_lines: u32,
    i: u32,
}

impl<'a, const N: usize> CurveLinesIterator<'a, N> {
    pub fn new(curve: &'a BCurve<N>, num_lines: u32) -> Self {
        Self {
            curve,
            num_lines,
            i: 0,
        }
    }
}

impl<'a, const N: usize> Iterator for CurveLinesIterator<'a, N> {
    type Item = Line<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.num_lines {
            let start = (1.0 / (self.num_lines as f32)) * (self.i as f32);
            let end = (1.0 / (self.num_lines as f32)) * ((self.i + 1) as f32);
            let line = self.curve.transition_line(start, end);

            self.i += 1;
            return Some(line);
        }

        None
    }
}
