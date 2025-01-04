use crate::geom::Line;
use crate::geom::Point;
use crate::geom::Rect;
use crate::geom::Size;

use assert_approx_eq::assert_approx_eq;
use pretty_assertions::assert_eq;

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

pub fn assert_approx_points_vec_eq(as_vec: Vec<Point<f32>>, bs_vec: Vec<Point<f32>>) {
    let as_len = as_vec.len();
    let bs_len = bs_vec.len();

    for (a, b) in as_vec.iter().zip(&bs_vec) {
        assert_approx_point_eq(*a, *b);
    }

    if as_len != bs_len {
        assert_eq!(as_vec, bs_vec);
    }
}
