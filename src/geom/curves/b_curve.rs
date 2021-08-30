use crate::geom::CurveLinesIterator;
use crate::geom::Line;
use crate::geom::Point;

///
/// See: https://youtu.be/aVwxzDHniEw
/// See: https://pomax.github.io/bezierinfo/
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BCurve<const N: usize> {
    pub(crate) points: [Point; N],
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct CurvePoint(pub Line);

/// The number of times to chop up a curve when calculating it's length.
/// Number picked is entirely arbituary. I have no idea if it's a good / bad number.
const LENGTH_SEGMENTS: u32 = 12;

impl<const N: usize> BCurve<N> {
    pub fn new_from_points(points: [Point; N]) -> Self {
        Self { points }
    }

    pub fn as_line(&self) -> Line {
        Line(self.start(), self.end())
    }

    pub fn start(&self) -> Point {
        self.points[0]
    }

    pub fn end(&self) -> Point {
        self.points[N - 1]
    }

    pub fn interpolation_line(self, start_n: f32, end_n: f32) -> Line {
        Line(
            self.interpolation_point(start_n),
            self.interpolation_point(end_n),
        )
    }

    pub fn interpolation_point(self, n: f32) -> Point {
        let mut ps: [Point; N] = self.points.clone();

        let mut count = N - 1;
        while count > 0 {
            for i in 0..count {
                ps[i] = Line(ps[i], ps[i + 1]).transition_point(n);
            }

            count -= 1;
        }

        ps[0]
    }

    /// An approximate total length for the curve.
    pub fn length(self) -> f32 {
        self.length_by_segments(LENGTH_SEGMENTS)
    }

    /// Calculates an approximate length of the curve,
    /// using the number of segments you provide.
    ///
    /// The lower the number of segments, the faster this will run.
    /// However it will be less accurate. A higher number will be slower,
    /// but more accurate.
    fn length_by_segments(self, num_segments: u32) -> f32 {
        self.iter_interpolation_lines(num_segments)
            .fold(0.0, |total, line| total + line.hypot())
    }

    pub fn iter_interpolation_lines<'a>(&'a self, num_lines: u32) -> CurveLinesIterator<'a, N> {
        CurveLinesIterator::new(self, num_lines)
    }
}

impl<const N: usize> Into<Line> for BCurve<N> {
    fn into(self) -> Line {
        self.as_line()
    }
}

#[cfg(test)]
mod interpolation_line {
    use super::*;

    #[test]
    fn it_should_return_whole_line_when_from_start_to_end() {
        let curve = BCurve::new_from_points([
            Point(100.0, 100.0),
            Point(200.0, 200.0),
            Point(200.0, 400.0),
            Point(100.0, 500.0),
        ]);

        assert_eq!(
            curve.interpolation_line(0.0, 1.0),
            Line(Point(100.0, 100.0), Point(100.0, 500.0)),
        );
    }

    #[test]
    fn it_should_return_first_half_on_straight_curve() {
        let curve = BCurve::new_from_points([
            Point(1.0, 0.0),
            Point(1.0, 2.0),
            Point(1.0, 8.0),
            Point(1.0, 10.0),
        ]);

        assert_eq!(
            curve.interpolation_line(0.0, 0.5),
            Line(Point(1.0, 0.0), Point(1.0, 5.0)),
        );
    }
}

#[cfg(test)]
mod interpolation_point {
    use super::*;

    #[test]
    fn it_should_return_first_half_on_straight_curve() {
        let curve = BCurve::new_from_points([
            Point(1.0, 0.0),
            Point(1.0, 2.0),
            Point(1.0, 8.0),
            Point(1.0, 10.0),
        ]);

        assert_eq!(curve.interpolation_point(0.5), Point(1.0, 5.0));
    }
}

#[cfg(test)]
mod iter_interpolation_lines {
    use super::*;
    use crate::geom::Point;

    #[test]
    fn it_should_return_number_of_lines_asked_for() {
        let curve = BCurve::new_from_points([
            Point(1.0, 0.0),
            Point(1.0, 2.0),
            Point(1.0, 8.0),
            Point(1.0, 10.0),
        ]);

        assert_eq!(13, curve.iter_interpolation_lines(13).count());
    }

    #[test]
    fn it_should_return_the_lines_we_expect() {
        let curve = BCurve::new_from_points([
            Point(0.0, 0.0),
            Point(0.0, 0.0),
            Point(10.0, 10.0),
            Point(10.0, 10.0),
        ]);

        let lines: Vec<Line> = curve.iter_interpolation_lines(4).collect();
        assert_eq!(
            lines,
            &[
                Line(Point(0.0, 0.0), Point(1.5625, 1.5625)),
                Line(Point(1.5625, 1.5625), Point(5.0, 5.0)),
                Line(Point(5.0, 5.0), Point(8.4375, 8.4375)),
                Line(Point(8.4375, 8.4375), Point(10.0, 10.0))
            ]
        );
    }
}
