use crate::geom::PointPosition;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LinePosition(pub PointPosition, pub PointPosition);

impl LinePosition {
    pub fn new(start: PointPosition, end: PointPosition) -> Self {
        Self(start, end)
    }

    pub fn start(self) -> PointPosition {
        self.0
    }

    pub fn end(self) -> PointPosition {
        self.1
    }

    pub fn is_entirely_left(self) -> bool {
        self.start().is_left() && self.end().is_left()
    }

    pub fn is_entirely_right(self) -> bool {
        self.start().is_right() && self.end().is_right()
    }

    pub fn is_entirely_above(self) -> bool {
        self.start().is_above() && self.end().is_above()
    }

    pub fn is_entirely_below(self) -> bool {
        self.start().is_below() && self.end().is_below()
    }

    pub fn is_on_same_horizontal(self) -> bool {
        self.start().horizontal() == self.end().horizontal()
    }

    pub fn is_on_same_vertical(self) -> bool {
        self.start().vertical() == self.end().vertical()
    }

    pub fn is_within_same_space(self) -> bool {
        self.start() == self.end()
    }

    pub fn is_entirely_inside(self) -> bool {
        self.start().is_entirely_inside() && self.end().is_entirely_inside()
    }
}
