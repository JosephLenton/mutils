use ::std::marker::PhantomData;

use crate::geom::Line;
use crate::geom::Point;
use crate::num::Num;

#[derive(Clone, Debug)]
pub struct LineIterator<N: Num = f32> {
    n_type_marker: PhantomData<N>,
    current: Point<f32>,
    end: Point<f32>,
    step: Point<f32>,
    is_exclusive: bool,
}

impl<N: Num> LineIterator<N> {
    pub fn new(line: Line<N>, step: N, is_exclusive: bool) -> Self {
        let line_f32 = line.to_f32();
        let current = line_f32.start();
        let end = line_f32.end();
        let step_f32 = step.to_rounded();
        let step = line.direction().to_point() * step_f32;

        if step_f32 == 0.0 {
            panic!("Zero step given");
        }

        if step_f32 < 0.0 {
            panic!("Negative step given");
        }

        Self {
            n_type_marker: PhantomData,
            current,
            end,
            step,
            is_exclusive,
        }
    }
}

impl<N: Num> Iterator for LineIterator<N> {
    type Item = Point<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if has_moved_to_final_iteration(self.current, self.end, self.step) {
            if self.is_exclusive {
                return None;
            }

            if has_moved_to_final_iteration(self.current - self.step, self.end, self.step) {
                return None;
            }

            self.current += self.step * 2.0;
            return Some(self.end.from_f32());
        }

        let current = self.current;
        self.current = current + self.step;
        Some(current.from_f32())
    }
}

fn has_moved_to_final_iteration(current: Point<f32>, end: Point<f32>, step: Point<f32>) -> bool {
    if current == end {
        return true;
    }

    if step.x() < 0.0 && current.x() < end.x() {
        return true;
    }

    if 0.0 < step.x() && end.x() < current.x() {
        return true;
    }

    if step.y() < 0.0 && current.y() < end.y() {
        return true;
    }

    if 0.0 < step.y() && end.y() < current.y() {
        return true;
    }

    false
}

#[cfg(test)]
mod iterating {
    use super::*;
    use crate::geom::testing_utils::assert_approx_points_vec_eq;
    use ::testcat::*;

    describe!("exclusive", {
        it!(
            "should return no points when start and finish are same",
            test_exclusive_same_point
        );
        it!(
            "should iterate all points from start to finish",
            test_exclusive_iterate_positive
        );
        it!(
            "should iterate all points from start to finish, in reverse",
            test_exclusive_iterate_negative
        );
    });

    describe!("inclusive", {
        it!(
            "should return one point when start and finish are same",
            test_inclusive_same_point
        );
        it!(
            "should iterate all points from start to finish",
            test_inclusive_iterate_positive
        );
        it!(
            "should iterate all points from start to finish, in reverse",
            test_inclusive_iterate_negative
        );
    });

    fn test_exclusive_same_point() {
        let line: Line<f32> = Line(Point(10.0, 20.0), Point(10.0, 20.0));
        let points: Vec<Point<f32>> = line.into_iter().collect();

        assert_eq!(points, vec![]);
    }

    fn test_exclusive_iterate_positive() {
        let line: Line<f32> = Line(Point(10.0, 20.0), Point(15.0, 24.0));
        let points: Vec<Point<f32>> = line.into_iter().collect();

        #[rustfmt::skip]
        assert_approx_points_vec_eq(points, vec![
            Point(10.0, 20.0),
            Point(10.780869, 20.624695),
            Point(11.561737, 21.24939),
            Point(12.342606, 21.874084),
            Point(13.123474, 22.49878),
            Point(13.904343, 23.123474),
            Point(14.685211, 23.748169),
        ]);
    }

    fn test_exclusive_iterate_negative() {
        let line: Line<f32> = Line(Point(10.0, 20.0), Point(15.0, 24.0)).reverse();
        let points: Vec<Point<f32>> = line.into_iter().collect();

        #[rustfmt::skip]
        assert_approx_points_vec_eq(points, vec![
            Point(15.0, 24.0),
            Point(14.219131, 23.375305),
            Point(13.438263, 22.75061),
            Point(12.657394, 22.125916),
            Point(11.876526, 21.50122),
            Point(11.095657, 20.876526),
            Point(10.314789, 20.251831),
        ]);
    }

    fn test_inclusive_same_point() {
        let line: Line<f32> = Line(Point(10.0, 20.0), Point(10.0, 20.0));
        let points: Vec<Point<f32>> = line.into_iter_inclusive().collect();

        #[rustfmt::skip]
        assert_approx_points_vec_eq(points, vec![
            Point(10.0, 20.0),
        ]);
    }

    fn test_inclusive_iterate_positive() {
        let line: Line<f32> = Line(Point(10.0, 20.0), Point(15.0, 24.0));
        let points: Vec<Point<f32>> = line.into_iter_inclusive().collect();

        #[rustfmt::skip]
        assert_approx_points_vec_eq(points, vec![
            Point(10.0, 20.0),
            Point(10.780869, 20.624695),
            Point(11.561737, 21.24939),
            Point(12.342606, 21.874084),
            Point(13.123474, 22.49878),
            Point(13.904343, 23.123474),
            Point(14.685211, 23.748169),
            Point(15.0, 24.0),
        ]);
    }

    fn test_inclusive_iterate_negative() {
        let line: Line<f32> = Line(Point(10.0, 20.0), Point(15.0, 24.0)).reverse();
        let points: Vec<Point<f32>> = line.into_iter_inclusive().collect();

        #[rustfmt::skip]
        assert_approx_points_vec_eq(points, vec![
            Point(15.0, 24.0),
            Point(14.219131, 23.375305),
            Point(13.438263, 22.75061),
            Point(12.657394, 22.125916),
            Point(11.876526, 21.50122),
            Point(11.095657, 20.876526),
            Point(10.314789, 20.251831),
            Point(10.0, 20.0),
        ]);
    }
}
