use num_traits::sign::signum;
use num_traits::sign::Signed;
use std::convert::Into;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::num::FromRounded;
use crate::num::INum;
use crate::num::Num;
use crate::num::NumIdentity;
use crate::num::NumTuple;
use crate::num::NumberExtensions;
use crate::num::ToRounded;

use crate::geom::Point;
use crate::geom::Rect;

use crate::internal::macros::quick_n_div;
use crate::internal::macros::quick_n_mul;

mod size_iterator;
pub use self::size_iterator::SizeIterator;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size<N: Num = f32>(pub N, pub N);

// Size Speicifc functions.
impl<N: Num> Size<N> {
    pub fn new_zero_value() -> Self {
        Size(N::zero(), N::zero())
    }

    pub fn new_one_value() -> Self {
        Size(N::one(), N::one())
    }

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

    /// Returns the longest side.
    ///
    /// Be aware it uses an absolute comparison. This means -10 is
    /// considered larger than 5.
    ///
    /// For example ... `Size(-10, 5).longest_size()` will return -10.
    pub fn longest_side(self) -> N {
        if self.width().abs() < self.height().abs() {
            self.height()
        } else {
            self.width()
        }
    }

    /// Note that for negative sizes, the
    pub fn shortest_side(self) -> N {
        if self.width().abs() < self.height().abs() {
            self.width()
        } else {
            self.height()
        }
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

    pub fn to_rect(self) -> Rect<N> {
        Rect(Point::new_zero_value(), self)
    }

    pub fn get_scale_diff(self, other: Self) -> Self {
        Size(other.width() / self.width(), other.height() / self.height())
    }

    /// This is to allow creating a new Size, with a new type, from the type given.
    /// i.e. `Size::new(1 as u8, 1 as u8)::to::<u32>()`
    pub fn to<T: Num + From<N>>(&self) -> Size<T> {
        Size(T::from(self.first()), T::from(self.second()))
    }

    pub fn abs(&self) -> Self {
        Self(self.width().abs(), self.height().abs())
    }

    pub fn min(self, other: Self) -> Self {
        Self(
            self.width().min(other.width()),
            self.height().min(other.height()),
        )
    }

    pub fn max(self, other: Self) -> Self {
        Self(
            self.width().max(other.width()),
            self.height().max(other.height()),
        )
    }

    pub fn hypot(self) -> N {
        let width: f32 = self.width().to_rounded();
        let height: f32 = self.height().to_rounded();
        let hypot = width.hypot(height);
        FromRounded::from_rounded(hypot)
    }

    pub fn hypot_sqrd(self) -> N {
        (self.width() * self.width()) + (self.height() * self.height())
    }

    pub fn interpolate_to(self, other: Size<N>, n: N) -> Size<N> {
        let start_f32 = self.to_f32();
        let other_f32 = other.to_f32();
        let n_f32: f32 = n.to_rounded();

        let new_size_f32 = (start_f32 * n_f32.inverse()) + (other_f32 * n_f32);
        new_size_f32.from_f32()
    }

    pub(crate) fn to_f32(self) -> Size<f32> {
        self.to_rounded()
    }

    pub fn swizzle_wh(self) -> Self {
        Self(self.width(), self.height())
    }

    pub fn swizzle_hw(self) -> Self {
        Self(self.height(), self.width())
    }

    pub fn swizzle_ww(self) -> Self {
        Self(self.width(), self.width())
    }

    pub fn swizzle_hh(self) -> Self {
        Self(self.height(), self.height())
    }

    pub fn swizzle_0w(self) -> Self {
        Self(N::zero(), self.width())
    }

    pub fn swizzle_0h(self) -> Self {
        Self(N::zero(), self.height())
    }

    pub fn swizzle_w0(self) -> Self {
        Self(self.width(), N::zero())
    }

    pub fn swizzle_h0(self) -> Self {
        Self(self.height(), N::zero())
    }
}

impl<N: Num + Signed> Size<N> {
    pub fn sign(&self) -> Self {
        Self(self.sign_width(), self.sign_height())
    }

    pub fn sign_width(&self) -> N {
        if self.width() == <N as NumIdentity>::zero() {
            return <N as NumIdentity>::zero();
        }

        signum(self.width())
    }

    pub fn sign_height(&self) -> N {
        if self.height() == <N as NumIdentity>::zero() {
            return <N as NumIdentity>::zero();
        }

        signum(self.height())
    }

    pub fn flip(self) -> Self {
        Self(-self.width(), -self.height())
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

impl Size<f32> {
    pub(crate) fn from_f32<N: Num>(self) -> Size<N> {
        Size(
            FromRounded::from_rounded(self.width()),
            FromRounded::from_rounded(self.height()),
        )
    }

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

    pub fn cossin(self, angle: f32) -> Self {
        Self::new(self.width() * angle.cos(), self.height() * angle.sin())
    }
}

impl<O: Num, N: Num + ToRounded<O>> ToRounded<Size<O>> for Size<N> {
    fn to_rounded(self) -> Size<O> {
        Size(self.width().to_rounded(), self.height().to_rounded())
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

quick_n_div!(f32, Size<f32>);
quick_n_div!(f64, Size<f64>);

quick_n_div!(usize, Size<usize>);
quick_n_div!(u8, Size<u8>);
quick_n_div!(u16, Size<u16>);
quick_n_div!(u32, Size<u32>);
quick_n_div!(u64, Size<u64>);
quick_n_div!(u128, Size<u128>);

quick_n_div!(isize, Size<isize>);
quick_n_div!(i8, Size<i8>);
quick_n_div!(i16, Size<i16>);
quick_n_div!(i32, Size<i32>);
quick_n_div!(i64, Size<i64>);
quick_n_div!(i128, Size<i128>);

quick_n_mul!(f32, Size<f32>);
quick_n_mul!(f64, Size<f64>);

quick_n_mul!(usize, Size<usize>);
quick_n_mul!(u8, Size<u8>);
quick_n_mul!(u16, Size<u16>);
quick_n_mul!(u32, Size<u32>);
quick_n_mul!(u64, Size<u64>);
quick_n_mul!(u128, Size<u128>);

quick_n_mul!(isize, Size<isize>);
quick_n_mul!(i8, Size<i8>);
quick_n_mul!(i16, Size<i16>);
quick_n_mul!(i32, Size<i32>);
quick_n_mul!(i64, Size<i64>);
quick_n_mul!(i128, Size<i128>);

impl<N: Num> Mul<N> for Size<N> {
    type Output = Self;

    fn mul(self, other: N) -> Self {
        Size::new(self.0 * other, self.1 * other)
    }
}

impl<N: Num> MulAssign<N> for Size<N> {
    fn mul_assign(&mut self, other: N) {
        *self = *self * other;
    }
}

impl<N: Num> Div<N> for Size<N> {
    type Output = Self;

    fn div(self, other: N) -> Self {
        Size::new(self.0 / other, self.1 / other)
    }
}

impl<N: Num> DivAssign<N> for Size<N> {
    fn div_assign(&mut self, other: N) {
        *self = *self / other;
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

impl<N: Num> IntoIterator for Size<N> {
    type Item = Point<N>;
    type IntoIter = SizeIterator<N>;

    fn into_iter(self) -> Self::IntoIter {
        SizeIterator::new(self)
    }
}

impl<N: Num> Display for Size<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Point({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod to_rect {
    use super::*;

    #[test]
    fn it_should_return_rect_with_this_size() {
        let size: Size<i32> = Size(123, 456);
        assert_eq!(size.to_rect(), Rect(Point(0, 0), Size(123, 456)));
    }
}

#[cfg(test)]
mod longest_side {
    use super::*;

    #[test]
    fn it_should_return_larger_width_over_smaller_height() {
        let size = Size(10, 5);

        assert_eq!(size.longest_side(), 10);
    }

    #[test]
    fn it_should_return_larger_height_over_smaller_width() {
        let size = Size(5, 10);

        assert_eq!(size.longest_side(), 10);
    }

    #[test]
    fn it_should_return_larger_negative_over_smaller_positive() {
        let size = Size(-10, 5);

        assert_eq!(size.longest_side(), -10);
    }
}

#[cfg(test)]
mod shortest_side {
    use super::*;

    #[test]
    fn it_should_return_smaller_height_over_larger_width() {
        let size = Size(10, 5);

        assert_eq!(size.shortest_side(), 5);
    }

    #[test]
    fn it_should_return_smaller_width_over_larger_height() {
        let size = Size(5, 10);

        assert_eq!(size.shortest_side(), 5);
    }

    #[test]
    fn it_should_return_smaller_positive_numbers_over_larger_negative_numbers() {
        let size = Size(-10, 5);

        assert_eq!(size.shortest_side(), 5);
    }
}

#[cfg(test)]
mod hypot {
    use super::*;

    #[test]
    fn it_should_be_correct_for_positive_values() {
        let size = Size(3.0, 4.0);

        assert_eq!(5.0, size.hypot());
    }
}

#[cfg(test)]
mod hypot_sqrd {
    use super::*;

    #[test]
    fn it_should_be_correct_for_positive() {
        let size = Size(3.0, 4.0);

        assert_eq!(25.0, size.hypot_sqrd());
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
