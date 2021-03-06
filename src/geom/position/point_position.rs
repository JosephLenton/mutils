use crate::geom::HorizontalPosition;
use crate::geom::VerticalPosition;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointPosition(pub HorizontalPosition, pub VerticalPosition);

impl PointPosition {
    pub fn new(horizontal: HorizontalPosition, vertical: VerticalPosition) -> Self {
        Self(horizontal, vertical)
    }

    #[inline(always)]
    pub fn horizontal(self) -> HorizontalPosition {
        self.0
    }

    #[inline(always)]
    pub fn vertical(self) -> VerticalPosition {
        self.1
    }

    pub fn is_left(self) -> bool {
        self.horizontal() == HorizontalPosition::Left
    }

    pub fn is_right(self) -> bool {
        self.horizontal() == HorizontalPosition::Right
    }

    pub fn is_above(self) -> bool {
        self.vertical() == VerticalPosition::Above
    }

    pub fn is_below(self) -> bool {
        self.vertical() == VerticalPosition::Below
    }

    pub fn is_entirely_inside(self) -> bool {
        self == PointPosition(HorizontalPosition::Inside, VerticalPosition::Inside)
    }

    // It is outside if either the horizontal or the vertical, is not inside.
    pub fn is_outside(self) -> bool {
        self.horizontal() != HorizontalPosition::Inside
            || self.vertical() != VerticalPosition::Inside
    }

    // It is entirely outside if neither the horizontal or the vertical, are inside.
    pub fn is_entirely_outside(self) -> bool {
        self.horizontal() != HorizontalPosition::Inside
            && self.vertical() != VerticalPosition::Inside
    }
}
