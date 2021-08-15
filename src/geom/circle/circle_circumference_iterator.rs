use crate::geom::Circle;
use crate::geom::Point;
use crate::num::Num;
use crate::num::ToRounded;
use ::std::f32::consts::TAU;

#[derive(Clone, Debug)]
pub struct CircleCircumferenceIterator {
    circle: Circle<f32>,
    index: usize,
    num_points: usize,
}

impl CircleCircumferenceIterator {
    pub fn new<N>(circle: Circle<N>, num_points: usize) -> Self
    where
        N: Num + ToRounded<f32>,
    {
        Self {
            circle: circle.to_rounded(),
            index: 0,
            num_points,
        }
    }
}

impl Iterator for CircleCircumferenceIterator {
    type Item = Point<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.num_points {
            return None;
        }

        let angle_index = self.index as f32 / self.num_points as f32;
        let angle = TAU * angle_index;
        self.index += 1;

        let mut edge_point = self.circle.centre() + Point(0.0, self.circle.radius().to_rounded());
        edge_point = edge_point.rotate_around_point(angle, self.circle.centre());

        Some(edge_point)
    }
}

#[cfg(test)]
mod iterator {
    use super::*;
    use ::assert_approx_eq::assert_approx_eq;

    #[test]
    fn it_should_return_num_of_points_requested() {
        let iterator = CircleCircumferenceIterator::new(Circle(Point(10.0, 20.0), 5.0), 3);

        assert_eq!(iterator.count(), 3);
    }

    #[test]
    fn it_should_return_no_points_if_zero_requested() {
        let mut iterator = CircleCircumferenceIterator::new(Circle(Point(10.0, 20.0), 5.0), 0);

        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn it_should_return_all_points_of_a_circle() {
        let iterator = CircleCircumferenceIterator::new(Circle(Point(10.0, 20.0), 5.0), 8);

        let points: Vec<Point<f32>> = iterator.collect();
        assert_eq!(
            points,
            vec![
                Point(10.0, 25.0),
                Point(13.535534, 23.535534),
                Point(15.0, 20.0),
                Point(13.535534, 16.464466),
                Point(10.0, 15.0),
                Point(6.464466, 16.464466),
                Point(5.0, 20.0),
                Point(6.4644666, 23.535534),
            ]
        );
    }
}
