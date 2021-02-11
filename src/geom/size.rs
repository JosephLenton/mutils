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

use ::num_traits::sign::{abs, signum, Signed};

use crate::num::FromClamped;
use crate::num::INum;
use crate::num::Num;
use crate::num::NumTuple;

use crate::geom::Point;

mod size_iterator;
pub use self::size_iterator::SizeIterator;

#[derive(Copy, Clone, Debug)]
pub struct Size<N: Num = f32>(pub N, pub N);

// Size Speicifc functions.
impl<N: Num> Size<N> {
    pub fn width(&self) -> N {
        self.first()
    }

    pub fn height(&self) -> N {
        self.second()
    }

    pub fn set_width(&mut self, width: N) {
        self.set_first(width);
    }

    pub fn set_height(&mut self, height: N) {
        self.set_second(height);
    }

    /// The area of the size.
    /// Returns width * height.
    pub fn area(&self) -> N {
        self.width() * self.height()
    }

    pub fn index(&self, pos: Point<N>) -> N {
        self.width() * pos.y() + pos.x()
    }

    pub fn half(&self) -> Self {
        Self(self.0.half(), self.1.half())
    }

    pub fn to_point(self) -> Point<N> {
        Point(self.width(), self.height())
    }

    pub fn get_scale_diff(self, other: Self) -> Self {
        Size(other.width() / self.width(), other.height() / self.height())
    }
}

impl<N: Num + Signed> Size<N> {
    pub fn sign(&self) -> Self {
        let width_sign = if self.0 == N::zero() {
            N::zero()
        } else {
            signum(self.0)
        };
        let height_sign = if self.1 == N::zero() {
            N::zero()
        } else {
            signum(self.1)
        };

        Self(width_sign, height_sign)
    }

    pub fn sign_exclusive_dir(&self) -> Self {
        let w_abs = abs(self.width());
        let h_abs = abs(self.height());

        if w_abs > h_abs {
            Self(signum(self.width()), N::zero())
        } else {
            Self(N::zero(), signum(self.height()))
        }
    }

    pub fn flip_horizontal(self) -> Self {
        Self(-self.width(), self.height())
    }

    pub fn flip_vertical(self) -> Self {
        Self(self.width(), -self.height())
    }
}

impl<N: Num> NumTuple<N> for Size<N> {
    fn new(width: N, height: N) -> Self {
        Size(width, height)
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

impl<N: Num> Size<N> {
    /// Converts to a new type. If the current values don't fit in the new type,
    /// then it'll be clamped between min and max.
    /// i.e. `NumTuple::new(1 as i16, 1 as i16)::to::<u16>()`
    pub fn to_clamped<T: Num + FromClamped<N>>(&self) -> Size<T> {
        NumTuple::new(
            T::from_clamped(self.first()),
            T::from_clamped(self.second()),
        )
    }

    /// This is to allow creating a new Size, with a new type, from the type given.
    /// i.e. `Size::new(1 as u8, 1 as u8)::to::<u32>()`
    pub fn to<T: Num + From<N>>(&self) -> Size<T> {
        NumTuple::new(T::from(self.first()), T::from(self.second()))
    }
}

impl Size<f32> {
    pub fn overlaps(self, Size(w, h): Self, xy1: Self, xy2: Self) -> bool {
        let Size(x_min, y_min) = xy1.min(xy2);
        let Size(x_max, y_max) = xy1.max(xy2);

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

    pub fn hypot(self) -> f32 {
        self.width().hypot(self.height())
    }
}

impl<N: Num> Add<Self> for Size<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        NumTuple::new(self.first() + other.first(), self.second() + other.second())
    }
}

impl<N: Num> AddAssign<Self> for Size<N> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<N: Num> Sub<Self> for Size<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Size::new(self.0 - other.0, self.1 - other.1)
    }
}

impl<N: Num> SubAssign<Self> for Size<N> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<N: Num> Mul<Self> for Size<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Size::new(self.0 * other.0, self.1 * other.1)
    }
}

impl<N: Num> MulAssign<Self> for Size<N> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<N: Num> Rem<Self> for Size<N> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Size::new(self.0 % other.0, self.1 % other.1)
    }
}

impl<N: Num> Div<Self> for Size<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Size::new(self.0 / other.0, self.1 / other.1)
    }
}

impl<N: Num> DivAssign<Self> for Size<N> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl<N: Num> Mul<N> for Size<N> {
    type Output = Self;

    fn mul(self, other: N) -> Self {
        Size::new(self.0 * other, self.1 * other)
    }
}

impl<N: Num> Div<N> for Size<N> {
    type Output = Self;

    fn div(self, other: N) -> Self {
        Size::new(self.0 / other, self.1 / other)
    }
}

impl<N: INum> Shl<N> for Size<N> {
    type Output = Self;

    fn shl(self, other: N) -> Self {
        Self(self.0 << other, self.1 << other)
    }
}

impl<N: INum> ShlAssign<N> for Size<N> {
    fn shl_assign(&mut self, other: N) {
        *self = *self << other;
    }
}

impl<N: INum> Shr<N> for Size<N> {
    type Output = Self;

    fn shr(self, other: N) -> Self {
        Self(self.0 >> other, self.1 >> other)
    }
}

impl<N: INum> ShrAssign<N> for Size<N> {
    fn shr_assign(&mut self, other: N) {
        *self = *self >> other;
    }
}

impl<N: Num + Signed> Neg for Size<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.width(), -self.height())
    }
}

impl<N: Num> From<(N, N)> for Size<N> {
    /// (N, N) -> Size(N, N)
    fn from((x, y): (N, N)) -> Self {
        Self::new(x, y)
    }
}

impl<N: Num> Into<(N, N)> for Size<N> {
    /// Size(N, N) -> (N, N)
    fn into(self) -> (N, N) {
        (self.0, self.1)
    }
}

impl<N: Num> PartialEq for Size<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<N: Num> IntoIterator for Size<N> {
    type Item = Point<N>;
    type IntoIter = SizeIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        SizeIterator::new(self)
    }
}

#[cfg(test)]
mod flip_horizontal {
    use super::*;

    #[test]
    fn it_should_flip_horizontally() {
        let size: Size<i32> = Size(15, 23);
        assert_eq!(size.flip_horizontal(), Size(-15, 23));
    }
}

#[cfg(test)]
mod flip_vertical {
    use super::*;

    #[test]
    fn it_should_flip_vertically() {
        let size: Size<i32> = Size(15, 23);
        assert_eq!(size.flip_vertical(), Size(15, -23));
    }
}
