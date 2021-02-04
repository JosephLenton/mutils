use crate::{Line, Point, Size};
use ::std::convert::From;
use ::std::ops::Add;

#[derive(Copy, Clone, PartialEq)]
pub struct Transform {
    scale: Size,
    position: Point,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Point(0.0, 0.0),
            scale: Size(1.0, 1.0),
        }
    }

    pub fn set_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    pub fn position(&self) -> Point<f32> {
        self.position
    }

    pub fn flip_x(&self) -> bool {
        self.scale.width() < 1.0
    }

    pub fn flip_y(&self) -> bool {
        self.scale.height() < 1.0
    }

    pub fn set_flip_x_from_dir(self, dir: f32) -> Self {
        self.set_flip_x_from_bool(dir < 0.0)
    }

    pub fn set_flip_y_from_dir(self, dir: f32) -> Self {
        self.set_flip_y_from_bool(dir < 0.0)
    }

    pub fn set_flip_x_from_bool(self, is_flipped: bool) -> Self {
        if is_flipped {
            self.set_scale_width(-1.0)
        } else {
            self.set_scale_width(1.0)
        }
    }

    pub fn set_flip_y_from_bool(self, is_flipped: bool) -> Self {
        if is_flipped {
            self.set_scale_height(-1.0)
        } else {
            self.set_scale_height(1.0)
        }
    }

    fn set_scale_width(mut self, scale_width: f32) -> Self {
        self.scale.set_width(scale_width);
        self
    }

    fn set_scale_height(mut self, scale_height: f32) -> Self {
        self.scale.set_height(scale_height);
        self
    }

    pub fn set_scale(mut self, scale: Size) -> Self {
        self.scale = scale;
        self
    }

    pub fn scale(self) -> Size {
        self.scale
    }
}

impl Add<Line> for Transform {
    type Output = Line;

    fn add(self, line: Self::Output) -> Self::Output {
        (line * self.scale()) + self.position()
    }
}

impl From<Point<f32>> for Transform {
    fn from(p: Point<f32>) -> Transform {
        Transform::new().set_position(p)
    }
}

impl From<Size<f32>> for Transform {
    fn from(s: Size<f32>) -> Transform {
        Transform::new().set_scale(s)
    }
}
