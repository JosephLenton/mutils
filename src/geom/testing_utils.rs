use crate::geom::Line;
use crate::geom::Point;

use ::assert_approx_eq::assert_approx_eq;

pub fn assert_approx_line_eq(a: Line<f32>, b: Line<f32>) {
    assert_approx_point_eq(a.start(), b.start());
    assert_approx_point_eq(a.end(), b.end());
}

pub fn assert_approx_point_eq(a: Point<f32>, b: Point<f32>) {
    assert_approx_eq!(a.x(), b.x());
    assert_approx_eq!(a.y(), b.y());
}
