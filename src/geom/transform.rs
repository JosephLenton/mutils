use std::convert::From;
use std::ops::Add;

use num_traits::sign::Signed;

use crate::geom::Line;
use crate::geom::Point;
use crate::geom::Size;
use crate::num::Num;
use crate::num::NumIdentity;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform<N: Num = f32> {
    scale: Size<N>,
    position: Point<N>,
    rotation: f32,
}

impl<N: Num> Transform<N> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            position: Point::new_zero_value(),
            scale: Size::new_one_value(),
            rotation: 0.0,
        }
    }

    #[must_use]
    pub fn set_position(mut self, position: Point<N>) -> Self {
        self.position = position;
        self
    }

    pub fn position(&self) -> Point<N> {
        self.position
    }

    #[must_use]
    pub fn set_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    #[must_use]
    fn set_scale_width(mut self, scale_width: N) -> Self {
        self.scale.set_width(scale_width);
        self
    }

    #[must_use]
    fn set_scale_height(mut self, scale_height: N) -> Self {
        self.scale.set_height(scale_height);
        self
    }

    #[must_use]
    pub fn set_scale(mut self, scale: Size<N>) -> Self {
        self.scale = scale;
        self
    }

    pub fn scale(self) -> Size<N> {
        self.scale
    }
}

impl<N> Transform<N>
where
    N: Num + Signed,
{
    pub fn set_flip_x_from_dir_num(self, dir: f32) -> Self {
        self.set_flip_x_from_bool(dir < 0.0)
    }

    pub fn set_flip_y_from_dir_num(self, dir: f32) -> Self {
        self.set_flip_y_from_bool(dir < 0.0)
    }

    pub fn set_flip_x_from_bool(self, is_flipped: bool) -> Self {
        if is_flipped {
            self.set_scale_width(-<N as NumIdentity>::one())
        } else {
            self.set_scale_width(<N as NumIdentity>::one())
        }
    }

    pub fn set_flip_y_from_bool(self, is_flipped: bool) -> Self {
        if is_flipped {
            self.set_scale_height(-<N as NumIdentity>::one())
        } else {
            self.set_scale_height(<N as NumIdentity>::one())
        }
    }
}

impl<N> Add<Line<N>> for Transform<N>
where
    N: Num,
{
    type Output = Line<N>;

    fn add(self, line: Self::Output) -> Self::Output {
        line.rotate_around_zero(self.rotation()) * self.scale() + self.position()
    }
}

impl<N> Add<Point<N>> for Transform<N>
where
    N: Num,
{
    type Output = Point<N>;

    fn add(self, point: Self::Output) -> Self::Output {
        point.rotate_around_zero(self.rotation()) * self.scale() + self.position()
    }
}

impl<N> From<Point<N>> for Transform<N>
where
    N: Num,
{
    fn from(p: Point<N>) -> Self {
        Self::new().set_position(p)
    }
}

impl<N> From<Size<N>> for Transform<N>
where
    N: Num,
{
    fn from(s: Size<N>) -> Self {
        Self::new().set_scale(s)
    }
}
