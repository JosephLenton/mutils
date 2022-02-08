use ::std::convert::Into;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Div;
use ::std::ops::DivAssign;
use ::std::ops::Mul;
use ::std::ops::MulAssign;
use ::std::ops::Neg;
use ::std::ops::Rem;
use ::std::ops::Shl;
use ::std::ops::ShlAssign;
use ::std::ops::Shr;
use ::std::ops::ShrAssign;
use ::std::ops::Sub;
use ::std::ops::SubAssign;

use ::num_traits::sign::abs;
use ::num_traits::sign::signum;
use ::num_traits::sign::Signed;

use crate::num::FromRounded;
use crate::num::INum;
use crate::num::Num;
use crate::num::NumIdentity;
use crate::num::NumTuple;
use crate::num::NumberExtensions;
use crate::num::ToRounded;
use crate::num::ToSignedClamped;

use crate::geom::Line;
use crate::geom::Rect;
use crate::geom::Size;
use crate::geom::Transform;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<N: Num = f32>(pub N, pub N);

// Point Speicifc functions.
impl<N: Num> Point<N> {
    pub fn new_zero_value() -> Self {
        Point(N::zero(), N::zero())
    }

    pub fn new_from_angle(angle: f32, hypot: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Point(hypot * cos, hypot * sin).from_f32()
    }

    pub fn move_from_angle(self, angle: f32, hypot: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let move_xy = Point(hypot * cos, hypot * sin);

        let new_position = self.to_f32() + move_xy;
        new_position.from_f32()
    }

    pub fn x(&self) -> N {
        self.first()
    }

    pub fn y(&self) -> N {
        self.second()
    }

    pub fn move_x(&mut self, x: N) {
        self.0 += x;
    }

    pub fn move_y(&mut self, y: N) {
        self.1 += y;
    }

    pub fn set_x(&mut self, x: N) {
        self.set_first(x);
    }

    pub fn set_y(&mut self, y: N) {
        self.set_second(y);
    }

    pub fn line_to(self, other: Self) -> Line<N> {
        Line(self, other)
    }

    pub fn rect_to(self, other: Self) -> Rect<N> {
        let xy = self.min(other);
        let other_xy = self.max(other);
        let size = xy.distance_to(other_xy);

        Rect(xy, size)
    }

    pub fn distance_to(self, other: Self) -> Size<N> {
        Size::new(other.first() - self.first(), other.second() - self.second())
    }

    pub fn direction_to(self, other: Self) -> Size<f32> {
        Line(self, other).direction()
    }

    /// This is to allow creating a new Point, which wraps a different number type.
    ///
    /// For example `Point(1u8, 2u8)::to::<u32>()` creates a `Point(1u32, 2u32)`.
    pub fn to<T: Num + From<N>>(&self) -> Point<T> {
        Point(T::from(self.first()), T::from(self.second()))
    }

    pub fn abs(&self) -> Self {
        Self(self.x().abs(), self.y().abs())
    }

    pub fn min(self, other: Self) -> Self {
        Self(self.x().min(other.x()), self.y().min(other.y()))
    }

    pub fn max(self, other: Self) -> Self {
        Self(self.x().max(other.x()), self.y().max(other.y()))
    }

    pub fn hypot_to(self, other: Point<N>) -> N {
        self.distance_to(other).hypot()
    }

    pub fn rotate_around_point(self, angle: f32, other: Self) -> Self {
        (self - other).rotate_around_zero(angle) + other
    }

    pub fn rotate_around_zero(self, rotation: f32) -> Self {
        let Point(x, y): Point<f32> = self.to_f32();

        let hypot = x.hypot(y);
        let angle = self.angle_to_zero() - rotation;
        let cos = angle.cos();
        let sin = angle.sin();

        Point(hypot * cos, hypot * sin).from_f32()
    }

    pub fn angle_to(self, other: Self) -> f32 {
        (self - other).angle_to_zero()
    }

    fn angle_to_zero(self) -> f32 {
        let self_f32 = self.to_f32();
        self_f32.y().atan2(self_f32.x())
    }

    pub fn interpolate_to(self, other: Point<N>, n: N) -> Point<N> {
        let start_f32 = self.to_f32();
        let other_f32 = other.to_f32();
        let n_f32: f32 = n.to_rounded();

        let new_size_f32 = (start_f32 * n_f32.inverse()) + (other_f32 * n_f32);
        new_size_f32.from_f32()
    }

    pub(crate) fn to_f32(self) -> Point<f32> {
        self.to_rounded()
    }
}

impl<N: Num> NumTuple<N> for Point<N> {
    fn new(x: N, y: N) -> Self {
        Point(x, y)
    }

    fn first(&self) -> N {
        self.0
    }

    fn second(&self) -> N {
        self.1
    }

    fn set_first(&mut self, n: N) {
        self.0 = n;
    }

    fn set_second(&mut self, n: N) {
        self.1 = n;
    }

    fn set(&mut self, first: N, second: N) {
        self.0 = first;
        self.1 = second;
    }

    fn get(&mut self) -> (N, N) {
        (self.0, self.1)
    }
}

impl<O: Num, N: Num + ToRounded<O>> ToRounded<Point<O>> for Point<N> {
    fn to_rounded(self) -> Point<O> {
        Point(self.x().to_rounded(), self.y().to_rounded())
    }
}

impl<N: Num + ToSignedClamped> Point<N> {
    pub fn to_signed_clamped(self) -> Point<<N as ToSignedClamped>::Output> {
        Point(self.x().to_signed_clamped(), self.y().to_signed_clamped())
    }
}

impl<N: Num + Signed> Point<N> {
    pub fn sign(&self) -> Self {
        Self(signum(self.0), signum(self.1))
    }

    pub fn sign_exclusive_dir(&self) -> Self {
        let w_abs = abs(self.first());
        let h_abs = abs(self.second());

        if w_abs > h_abs {
            Self(signum(self.first()), <N as NumIdentity>::zero())
        } else {
            Self(<N as NumIdentity>::zero(), signum(self.second()))
        }
    }

    pub fn flip_x(self) -> Self {
        Self(-self.x(), self.y())
    }

    pub fn flip_y(self) -> Self {
        Self(self.x(), -self.y())
    }
}

impl Point<f32> {
    pub fn overlaps(self, Point(w, h): Self, xy1: Self, xy2: Self) -> bool {
        let Point(x_min, y_min) = xy1.min(xy2);
        let Point(x_max, y_max) = xy1.max(xy2);

        let half_w = w / 2.0;
        let x = self.0 + half_w;
        let other_half_w = (x_max - x_min) / 2.0;
        let other_x = x_min + other_half_w;
        let dist_x = (x - other_x).abs();

        if dist_x > (half_w + other_half_w) {
            return false;
        }

        let half_h = h / 2.0;
        let y = self.1 + half_h;
        let other_half_h = (y_max - y_min) / 2.0;
        let other_y = y_min + other_half_h;
        let dist_y = (y - other_y).abs();

        if dist_y > (half_h + other_half_h) {
            return false;
        }

        true
    }

    pub fn floor(self) -> Self {
        Self::new(self.first().floor(), self.second().floor())
    }

    pub fn ceil(self) -> Self {
        Self::new(self.first().ceil(), self.second().ceil())
    }

    pub fn round(self) -> Self {
        Self::new(self.first().round(), self.second().round())
    }

    pub(crate) fn from_f32<N: Num>(self) -> Point<N> {
        Point(
            FromRounded::from_rounded(self.x()),
            FromRounded::from_rounded(self.y()),
        )
    }
}

impl<N: Num> Add<Self> for Point<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        NumTuple::new(self.first() + other.first(), self.second() + other.second())
    }
}

impl<N: Num> AddAssign<Self> for Point<N> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<N: Num> Sub<Self> for Point<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point::new(self.0 - other.0, self.1 - other.1)
    }
}

impl<N: Num> SubAssign<Self> for Point<N> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<N: Num> Mul<Self> for Point<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Point::new(self.0 * other.0, self.1 * other.1)
    }
}

impl<N: Num> MulAssign<Self> for Point<N> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<N: Num> Div<Self> for Point<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Point::new(self.0 / other.0, self.1 / other.1)
    }
}

impl<N: Num> DivAssign<Self> for Point<N> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl<N: Num> Rem<Self> for Point<N> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Point::new(self.0 % other.0, self.1 % other.1)
    }
}

impl<N: Num> Add<Size<N>> for Point<N> {
    type Output = Self;

    fn add(self, other: Size<N>) -> Self {
        NumTuple::new(self.x() + other.width(), self.y() + other.height())
    }
}

impl<N: Num> AddAssign<Size<N>> for Point<N> {
    fn add_assign(&mut self, other: Size<N>) {
        *self = *self + other;
    }
}

impl<N: Num> Sub<Size<N>> for Point<N> {
    type Output = Self;

    fn sub(self, other: Size<N>) -> Self {
        Point::new(self.x() - other.width(), self.y() - other.height())
    }
}

impl<N: Num> SubAssign<Size<N>> for Point<N> {
    fn sub_assign(&mut self, other: Size<N>) {
        *self = *self - other;
    }
}

impl<N: Num> Mul<Size<N>> for Point<N> {
    type Output = Self;

    fn mul(self, other: Size<N>) -> Self {
        Point::new(self.x() * other.width(), self.y() * other.height())
    }
}

impl<N: Num> MulAssign<Size<N>> for Point<N> {
    fn mul_assign(&mut self, other: Size<N>) {
        *self = *self * other;
    }
}

impl<N: Num> Div<Size<N>> for Point<N> {
    type Output = Self;

    fn div(self, other: Size<N>) -> Self {
        Point::new(self.x() / other.width(), self.y() / other.height())
    }
}

impl<N: Num> DivAssign<Size<N>> for Point<N> {
    fn div_assign(&mut self, other: Size<N>) {
        *self = *self / other;
    }
}

impl<N: Num> Mul<N> for Point<N> {
    type Output = Self;

    fn mul(self, other: N) -> Self {
        Point::new(self.0 * other, self.1 * other)
    }
}

impl<N: Num> MulAssign<N> for Point<N> {
    fn mul_assign(&mut self, other: N) {
        *self = *self * other;
    }
}

impl<N: Num> Div<N> for Point<N> {
    type Output = Self;

    fn div(self, other: N) -> Self {
        Point::new(self.0 / other, self.1 / other)
    }
}

impl<N: Num> DivAssign<N> for Point<N> {
    fn div_assign(&mut self, other: N) {
        *self = *self / other;
    }
}

impl<N: INum> Shl<N> for Point<N> {
    type Output = Self;

    fn shl(self, other: N) -> Self {
        Self(self.0 << other, self.1 << other)
    }
}

impl<N: INum> ShlAssign<N> for Point<N> {
    fn shl_assign(&mut self, other: N) {
        *self = *self << other;
    }
}

impl<N: INum> Shr<N> for Point<N> {
    type Output = Self;

    fn shr(self, other: N) -> Self {
        Self(self.0 >> other, self.1 >> other)
    }
}

impl<N: INum> ShrAssign<N> for Point<N> {
    fn shr_assign(&mut self, other: N) {
        *self = *self >> other;
    }
}

impl<N: Num + Signed> Neg for Point<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y())
    }
}

impl<N> Add<Transform<N>> for Point<N>
where
    N: Num,
{
    type Output = Point<N>;

    #[inline(always)]
    fn add(self, transform: Transform<N>) -> Self::Output {
        transform + self
    }
}

impl<N: Num> From<(N, N)> for Point<N> {
    /// (N, N) -> Point(N, N)
    fn from((x, y): (N, N)) -> Self {
        Self::new(x, y)
    }
}

impl<N: Num> Into<(N, N)> for Point<N> {
    /// Point(N, N) -> (N, N)
    fn into(self) -> (N, N) {
        (self.0, self.1)
    }
}

#[cfg(test)]
mod flip_x {
    use super::*;

    #[test]
    fn it_should_flip_x() {
        let point: Point<i32> = Point(15, 23);
        assert_eq!(point.flip_x(), Point(-15, 23));
    }
}

#[cfg(test)]
mod flip_y {
    use super::*;

    #[test]
    fn it_should_flip_y() {
        let point: Point<i32> = Point(15, 23);
        assert_eq!(point.flip_y(), Point(15, -23));
    }
}

#[cfg(test)]
mod rotate_around_point {
    use super::*;
    use crate::geom::testing_utils::assert_approx_point_eq;
    use ::std::f32::consts::TAU;

    #[test]
    fn it_should_rotate_90_degrees() {
        let point = Point(0.0, 10.0);
        let rotate = point.rotate_around_point(TAU * 0.25, Point(0.0, 0.0));

        assert_approx_point_eq(rotate, Point(10.0, 0.0));
    }

    #[test]
    fn it_should_rotate_90_degrees_around_point() {
        let point = Point(25.0, 35.0);
        let rotate = point.rotate_around_point(TAU * 0.25, Point(25.0, 25.0));

        assert_approx_point_eq(rotate, Point(35.0, 25.0));
    }
}

#[cfg(test)]
mod angle_to {
    use super::*;
    use ::assert_approx_eq::assert_approx_eq;
    use ::std::f32::consts::TAU;

    #[test]
    fn it_should_angle_to_zero_from_right() {
        let point = Point(10.0, 0.0);
        assert_approx_eq!(point.angle_to(Point(0.0, 0.0)), 0.0);
    }

    #[test]
    fn it_should_angle_to_zero_from_above() {
        let point = Point(0.0, 10.0);
        assert_approx_eq!(point.angle_to(Point(0.0, 0.0)), TAU * 0.25);
    }

    #[test]
    fn it_should_angle_to_zero_from_left() {
        let point = Point(-10.0, 0.0);
        assert_approx_eq!(point.angle_to(Point(0.0, 0.0)), TAU * 0.5);
    }

    #[test]
    fn it_should_angle_to_zero_from_below() {
        let point = Point(0.0, -10.0);
        assert_approx_eq!(point.angle_to(Point(0.0, 0.0)), -TAU * 0.25);
    }

    #[test]
    fn it_should_angle_to_point_from_right() {
        let origin = Point(5.0, 8.0);
        let point = Point(10.0, 0.0) + origin;
        assert_approx_eq!(point.angle_to(origin), 0.0);
    }

    #[test]
    fn it_should_angle_to_point_from_above() {
        let origin = Point(5.0, 8.0);
        let point = Point(0.0, 10.0) + origin;
        assert_approx_eq!(point.angle_to(origin), TAU * 0.25);
    }

    #[test]
    fn it_should_angle_to_point_from_left() {
        let origin = Point(5.0, 8.0);
        let point = Point(-10.0, 0.0) + origin;
        assert_approx_eq!(point.angle_to(origin), TAU * 0.5);
    }

    #[test]
    fn it_should_angle_to_point_from_below() {
        let origin = Point(5.0, 8.0);
        let point = Point(0.0, -10.0) + origin;
        assert_approx_eq!(point.angle_to(origin), -TAU * 0.25);
    }
}
