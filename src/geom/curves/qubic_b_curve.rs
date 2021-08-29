use crate::geom::BCurve;
use crate::geom::Line;
use crate::geom::Point;

pub type QubicBCurve = BCurve<4>;

const START_POSITION: usize = 0;
const START_GUIDE: usize = 1;
const END_GUIDE: usize = 2;
const END_POSITION: usize = 3;

impl QubicBCurve {
    pub fn new_from_guide_lines(start: Line, end: Line) -> Self {
        Self::new_from_points([start.start(), start.end(), end.end(), end.start()])
    }

    pub fn start_position(self) -> Point {
        self.points[START_POSITION]
    }

    pub fn end_position(self) -> Point {
        self.points[END_POSITION]
    }

    pub fn start_guide(self) -> Point {
        self.points[START_GUIDE]
    }

    pub fn end_guide(self) -> Point {
        self.points[END_GUIDE]
    }

    pub fn start_line(self) -> Line {
        Line(self.start_position(), self.start_guide())
    }

    pub fn end_line(self) -> Line {
        Line(self.end_position(), self.end_guide())
    }
}

#[cfg(test)]
mod transition_line {
    use super::*;
    use crate::geom::Point;

    #[test]
    fn it_should_return_whole_line_when_from_start_to_end() {
        let curve = QubicBCurve::new_from_guide_lines(
            Line(Point(100.0, 100.0), Point(200.0, 200.0)),
            Line(Point(100.0, 500.0), Point(200.0, 400.0)),
        );

        assert_eq!(
            curve.transition_line(0.0, 1.0),
            Line(Point(100.0, 100.0), Point(100.0, 500.0)),
        );
    }

    #[test]
    fn it_should_return_first_half_on_straight_curve() {
        let curve = QubicBCurve::new_from_guide_lines(
            Line(Point(1.0, 0.0), Point(1.0, 2.0)),
            Line(Point(1.0, 10.0), Point(1.0, 8.0)),
        );

        assert_eq!(
            curve.transition_line(0.0, 0.5),
            Line(Point(1.0, 0.0), Point(1.0, 5.0)),
        );
    }
}

#[cfg(test)]
mod transition_point {
    use super::*;
    use crate::geom::Point;

    #[test]
    fn it_should_return_first_half_on_straight_curve() {
        let curve = QubicBCurve::new_from_guide_lines(
            Line(Point(1.0, 0.0), Point(1.0, 2.0)),
            Line(Point(1.0, 10.0), Point(1.0, 8.0)),
        );

        assert_eq!(curve.transition_point(0.5), Point(1.0, 5.0));
    }
}

#[cfg(test)]
mod iter_transition_lines {
    use super::*;
    use crate::geom::Point;

    #[test]
    fn it_should_return_number_of_lines_asked_for() {
        let curve = QubicBCurve::new_from_guide_lines(
            Line(Point(1.0, 0.0), Point(1.0, 2.0)),
            Line(Point(1.0, 10.0), Point(1.0, 8.0)),
        );

        assert_eq!(13, curve.iter_transition_lines(13).count());
    }
}
