use crate::geom::Line;
use crate::geom::Point;
use crate::geom::Size;
use crate::geom::Rect;

use ::assert_approx_eq::assert_approx_eq;

pub fn assert_approx_rect_eq(a: Rect<f32>, b: Rect<f32>) {
    assert_approx_point_eq(a.bottom_left(), b.bottom_left());
    assert_approx_size_eq(a.size(), b.size());
}

pub fn assert_approx_size_eq(a: Size<f32>, b: Size<f32>) {
    assert_approx_eq!(a.width(), b.width());
    assert_approx_eq!(a.height(), b.height());
}

pub fn assert_approx_line_eq(a: Line<f32>, b: Line<f32>) {
    assert_approx_point_eq(a.start(), b.start());
    assert_approx_point_eq(a.end(), b.end());
}

pub fn assert_approx_point_eq(a: Point<f32>, b: Point<f32>) {
    assert_approx_eq!(a.x(), b.x());
    assert_approx_eq!(a.y(), b.y());
}
