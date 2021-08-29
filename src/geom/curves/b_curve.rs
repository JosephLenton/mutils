use crate::geom::CurveLinesIterator;
use crate::geom::Line;
use crate::geom::Point;

///
/// See: https://youtu.be/aVwxzDHniEw
/// See: https://pomax.github.io/bezierinfo/
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BCurve<const N: usize> {
    points: [Point; N],
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

    pub fn transition_line(self, n1: f32, n2: f32) -> Line {
        Line(self.transition_point(n1), self.transition_point(n2))
    }

    pub fn transition_point(self, n: f32) -> Point {
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
        self.length_segments(LENGTH_SEGMENTS)
    }

    fn length_segments(self, num_segments: u32) -> f32 {
        self.iter_transition_lines(num_segments)
            .fold(0.0, |total, line| total + line.hypot())
    }

    pub fn iter_transition_lines<'a>(&'a self, num_lines: u32) -> CurveLinesIterator<'a, N> {
        CurveLinesIterator::new(self, num_lines)
    }
}

impl<const N: usize> Into<Line> for BCurve<N> {
    fn into(self) -> Line {
        self.as_line()
    }
}

#[cfg(test)]
mod transition_line {
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
            curve.transition_line(0.0, 1.0),
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
            curve.transition_line(0.0, 0.5),
            Line(Point(1.0, 0.0), Point(1.0, 5.0)),
        );
    }
}

#[cfg(test)]
mod transition_point {
    use super::*;

    #[test]
    fn it_should_return_first_half_on_straight_curve() {
        let curve = BCurve::new_from_points([
            Point(1.0, 0.0),
            Point(1.0, 2.0),
            Point(1.0, 8.0),
            Point(1.0, 10.0),
        ]);

        assert_eq!(curve.transition_point(0.5), Point(1.0, 5.0));
    }
}

#[cfg(test)]
mod iter_transition_lines {
    use super::*;

    #[test]
    fn it_should_return_number_of_lines_asked_for() {
        let curve = BCurve::new_from_points([
            Point(1.0, 0.0),
            Point(1.0, 2.0),
            Point(1.0, 8.0),
            Point(1.0, 10.0),
        ]);

        assert_eq!(13, curve.iter_transition_lines(13).count());
    }
}
