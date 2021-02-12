use crate::geom::PointPosition;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LinePosition(pub PointPosition, pub PointPosition);
