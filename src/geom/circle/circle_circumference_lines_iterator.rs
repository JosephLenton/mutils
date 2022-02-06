use crate::geom::Circle;
use crate::geom::Line;
use crate::geom::Point;
use crate::num::Num;
use crate::num::ToRounded;
use ::std::f32::consts::TAU;

#[derive(Clone, Debug)]
pub struct CircleCircumferenceLinesIterator {
    circle: Circle<f32>,
    index: usize,
    num_lines: usize,
}

impl CircleCircumferenceLinesIterator {
    pub fn new<N>(circle: Circle<N>, num_lines: usize) -> Self
    where
        N: Num,
    {
        Self {
            circle: circle.to_rounded(),
            index: 0,
            num_lines,
        }
    }
}

impl CircleCircumferenceLinesIterator {
    fn calculate_edge_point(&self, index: usize) -> Point<f32> {
        let angle = self.calculate_angle_index(index);
        let edge_point = self.circle.centre() + Point(0.0, self.circle.radius().to_rounded());
        edge_point.rotate_around_point(angle, self.circle.centre())
    }

    fn calculate_angle_index(&self, index: usize) -> f32 {
        let angle_index = index as f32 / self.num_lines as f32;
        TAU * angle_index
    }
}

impl Iterator for CircleCircumferenceLinesIterator {
    type Item = Line<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.num_lines {
            return None;
        }

        let point_from = self.calculate_edge_point(self.index);
        let point_to = self.calculate_edge_point(self.index + 1);
        let line = Line(point_from, point_to);

        self.index += 1;

        Some(line)
    }
}

#[cfg(test)]
mod iterator {
    use super::*;
    use crate::geom::testing_utils::assert_approx_line_eq;

    #[test]
    fn it_should_return_num_of_points_requested() {
        let iterator = CircleCircumferenceLinesIterator::new(Circle(Point(10.0, 20.0), 5.0), 3);

        assert_eq!(iterator.count(), 3);
    }

    #[test]
    fn it_should_return_no_points_if_zero_requested() {
        let mut iterator = CircleCircumferenceLinesIterator::new(Circle(Point(10.0, 20.0), 5.0), 0);

        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn it_should_return_all_points_of_a_circle() {
        let circle: Circle<f32> = Circle(Point(10.0, 20.0), 5.0);
        let iterator = CircleCircumferenceLinesIterator::new(circle, 8);
        let points: Vec<Line<f32>> = iterator.collect();

        assert_eq!(points.len(), 8);
        assert_approx_line_eq(
            points[0],
            Line(Point(10.0, 25.0), Point(13.535534, 23.535534)),
        );
        assert_approx_line_eq(
            points[1],
            Line(Point(13.535534, 23.535534), Point(15.0, 20.0)),
        );
        assert_approx_line_eq(
            points[2],
            Line(Point(15.0, 20.0), Point(13.535534, 16.464466)),
        );
        assert_approx_line_eq(
            points[3],
            Line(Point(13.535534, 16.464466), Point(10.0, 15.0)),
        );
        assert_approx_line_eq(
            points[4],
            Line(Point(10.0, 15.0), Point(6.464466, 16.464466)),
        );
        assert_approx_line_eq(
            points[5],
            Line(Point(6.464466, 16.464466), Point(5.0, 20.0)),
        );
        assert_approx_line_eq(
            points[6],
            Line(Point(5.0, 20.0), Point(6.4644666, 23.535534)),
        );
        assert_approx_line_eq(
            points[7],
            Line(Point(6.4644666, 23.535534), Point(10.0, 25.0)),
        );
    }
}
