use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

pub type Color = Colour;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    rgba: u32,
}

impl Colour {
    pub const WHITE: Colour = Colour::new_from_rgba(0xffffffff);
    pub const LIGHT_GREY: Colour = Colour::new_from_rgba(0xC0C0C0ff);
    pub const GREY: Colour = Colour::new_from_rgba(0x808080ff);
    pub const DARK_GREY: Colour = Colour::new_from_rgba(0x404040ff);
    pub const BLACK: Colour = Colour::new_from_rgba(0x000000ff);

    pub const RED: Colour = Colour::new_from_rgba(0xff0000ff);
    pub const GREEN: Colour = Colour::new_from_rgba(0x00ff00ff);
    pub const BLUE: Colour = Colour::new_from_rgba(0x0000ffff);

    pub const MAGENTA: Colour = Colour::new_from_rgba(0xff00ffff);
    pub const CYAN: Colour = Colour::new_from_rgba(0x00ffffff);
    pub const YELLOW: Colour = Colour::new_from_rgba(0xffff00ff);

    #[inline(always)]
    fn hex_u32_to_f32(val: u32, shift: u32) -> f32 {
        ((val >> shift) & 0xff) as f32 / 255.0
    }

    #[inline(always)]
    fn hex_u32_to_u32(val: u32, shift: u32) -> u32 {
        (val >> shift) & 0xff
    }

    #[inline(always)]
    fn hex_u32_to_u8(val: u32, shift: u32) -> u8 {
        ((val >> shift) & 0xff) as u8
    }

    #[inline(always)]
    fn f32_to_hex_u32(val: f32, shift: u8) -> u32 {
        ((val * 255.0).round() as u32) << shift
    }

    fn clamp_u8_to_f32(val: u8) -> f32 {
        if val >= 255 {
            1.0
        } else if val == 0 {
            0.0
        } else {
            (val as f32) / 255.0
        }
    }

    fn f32_to_rgba_u8(val: f32) -> u8 {
        if val >= 1.0 {
            255
        } else if val <= 0.0 {
            0
        } else {
            (val * 255.0) as u8
        }
    }

    fn f32_to_rgba_u32(val: f32) -> u32 {
        if val >= 1.0 {
            255
        } else if val <= 0.0 {
            0
        } else {
            (val * 255.0) as u32
        }
    }

    const fn rgba_u32s_to_rgba_hex(red: u32, green: u32, blue: u32, alpha: u32) -> u32 {
        (red << 24) | (green << 16) | (blue << 8) | alpha
    }

    const fn rgba_u8s_to_rgba_hex(red: u8, green: u8, blue: u8, alpha: u8) -> u32 {
        ((red as u32) << 24) | ((green as u32) << 16) | ((blue as u32) << 8) | (alpha as u32)
    }

    pub fn new_from_f32s(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        let red = Colour::f32_to_rgba_u32(red);
        let green = Colour::f32_to_rgba_u32(green);
        let blue = Colour::f32_to_rgba_u32(blue);
        let alpha = Colour::f32_to_rgba_u32(alpha);

        Self {
            rgba: Colour::rgba_u32s_to_rgba_hex(red, green, blue, alpha),
        }
    }

    pub const fn new_from_u32s(red: u32, green: u32, blue: u32, alpha: u32) -> Self {
        Self {
            rgba: Colour::rgba_u32s_to_rgba_hex(red, green, blue, alpha),
        }
    }

    pub const fn new_from_u8s(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            rgba: Colour::rgba_u8s_to_rgba_hex(red, green, blue, alpha),
        }
    }

    pub const fn new_from_rgba(rgba: u32) -> Self {
        Self { rgba }
    }

    pub fn red_f32(&self) -> f32 {
        Colour::hex_u32_to_f32(self.rgba, 24)
    }

    pub fn red_u32(&self) -> u32 {
        Colour::hex_u32_to_u32(self.rgba, 24)
    }

    pub fn red_u8(&self) -> u8 {
        Colour::hex_u32_to_u8(self.rgba, 24)
    }

    pub fn green_f32(&self) -> f32 {
        Colour::hex_u32_to_f32(self.rgba, 16)
    }

    pub fn green_u32(&self) -> u32 {
        Colour::hex_u32_to_u32(self.rgba, 16)
    }

    pub fn green_u8(&self) -> u8 {
        Colour::hex_u32_to_u8(self.rgba, 16)
    }

    pub fn blue_f32(&self) -> f32 {
        Colour::hex_u32_to_f32(self.rgba, 8)
    }

    pub fn blue_u32(&self) -> u32 {
        Colour::hex_u32_to_u32(self.rgba, 8)
    }

    pub fn blue_u8(&self) -> u8 {
        Colour::hex_u32_to_u8(self.rgba, 8)
    }

    pub fn alpha_f32(&self) -> f32 {
        Colour::hex_u32_to_f32(self.rgba, 0)
    }

    pub fn alpha_u32(&self) -> u32 {
        Colour::hex_u32_to_u32(self.rgba, 0)
    }

    pub fn alpha_u8(&self) -> u8 {
        Colour::hex_u32_to_u8(self.rgba, 0)
    }

    pub fn mix(self, other: Self, mut amount: f32) -> Self {
        amount = amount.max(0.0).min(1.0);
        let inverse_amount = 1.0 - amount;

        (self * inverse_amount) + (other * amount)
    }

    pub fn mix_no_alpha(self, other: Self, mut amount: f32) -> Self {
        amount = amount.max(0.0).min(1.0);
        let inverse_amount = 1.0 - amount;

        let mut result = (self * inverse_amount) + (other * amount);
        result.rgba = (result.rgba & 0xffffff00) | self.alpha_u32();
        result
    }

    pub fn replace_alpha_f32(self, alpha: f32) -> Self {
        self.replace_alpha_u32(Colour::f32_to_rgba_u32(alpha))
    }

    pub fn replace_alpha_u32(mut self, alpha: u32) -> Self {
        self.rgba = (self.rgba & 0xffffff00) & alpha;
        self
    }

    #[inline(always)]
    pub fn to_rgba_u32(self) -> u32 {
        self.rgba
    }
}

impl Add<Self> for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new_from_f32s(
            self.red_f32() + other.red_f32(),
            self.green_f32() + other.green_f32(),
            self.blue_f32() + other.blue_f32(),
            self.alpha_f32() + other.alpha_f32(),
        )
    }
}

impl AddAssign<Self> for Colour {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub<Self> for Colour {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new_from_f32s(
            self.red_f32() - other.red_f32(),
            self.green_f32() - other.green_f32(),
            self.blue_f32() - other.blue_f32(),
            self.alpha_f32() - other.alpha_f32(),
        )
    }
}

impl SubAssign<Self> for Colour {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<Self> for Colour {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new_from_f32s(
            self.red_f32() * other.red_f32(),
            self.green_f32() * other.green_f32(),
            self.blue_f32() * other.blue_f32(),
            self.alpha_f32() * other.alpha_f32(),
        )
    }
}

impl MulAssign<Self> for Colour {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Div<Self> for Colour {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new_from_f32s(
            self.red_f32() / other.red_f32(),
            self.green_f32() / other.green_f32(),
            self.blue_f32() / other.blue_f32(),
            self.alpha_f32() / other.alpha_f32(),
        )
    }
}

impl DivAssign<Self> for Colour {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl Rem<Self> for Colour {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self::new_from_f32s(
            self.red_f32() % other.red_f32(),
            self.green_f32() % other.green_f32(),
            self.blue_f32() % other.blue_f32(),
            self.alpha_f32() % other.alpha_f32(),
        )
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, val: f32) -> Self {
        Self::new_from_f32s(
            self.red_f32() * val,
            self.green_f32() * val,
            self.blue_f32() * val,
            self.alpha_f32() * val,
        )
    }
}

impl MulAssign<f32> for Colour {
    fn mul_assign(&mut self, val: f32) {
        *self = *self * val;
    }
}

impl Div<f32> for Colour {
    type Output = Self;

    fn div(self, val: f32) -> Self {
        Self::new_from_f32s(
            self.red_f32() / val,
            self.green_f32() / val,
            self.blue_f32() / val,
            self.alpha_f32() / val,
        )
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, val: f32) {
        *self = *self / val;
    }
}

#[cfg(test)]
mod red_xx {
    use super::*;

    #[test]
    pub fn it_should_return_red_given() {
        let rgba_hex = 0xffa89321;
        let colour = Colour::new_from_rgba(rgba_hex);

        assert_eq!(colour.red_f32(), 1.0);
        assert_eq!(colour.red_u32(), 255);
        assert_eq!(colour.red_u8(), 255);
    }
}

#[cfg(test)]
mod green_xx {
    use super::*;

    #[test]
    pub fn it_should_return_green_given() {
        let rgba_hex = 0xffa89321;
        let colour = Colour::new_from_rgba(rgba_hex);

        assert_eq!(colour.green_f32(), 168.0 / 255.0);
        assert_eq!(colour.green_u32(), 168);
        assert_eq!(colour.green_u8(), 168);
    }
}

#[cfg(test)]
mod blue_xx {
    use super::*;

    #[test]
    pub fn it_should_return_blue_given() {
        let rgba_hex = 0xffa89321;
        let colour = Colour::new_from_rgba(rgba_hex);

        assert_eq!(colour.blue_f32(), 147.0 / 255.0);
        assert_eq!(colour.blue_u32(), 147);
        assert_eq!(colour.blue_u8(), 147);
    }
}

#[cfg(test)]
mod alpha_xx {
    use super::*;

    #[test]
    pub fn it_should_return_alpha_given() {
        let rgba_hex = 0xffa89321;
        let colour = Colour::new_from_rgba(rgba_hex);

        assert_eq!(colour.alpha_f32(), 33.0 / 255.0);
        assert_eq!(colour.alpha_u32(), 33);
        assert_eq!(colour.alpha_u8(), 33);
    }
}

#[cfg(test)]
mod new_rgba_hex {
    use super::*;

    #[test]
    pub fn it_should_have_components_match_those_given() {
        let rgba_hex = 0xffa89321;
        let colour = Colour::new_from_rgba(rgba_hex);

        assert_eq!(colour.red_u32(), 255);
        assert_eq!(colour.green_u32(), 168);
        assert_eq!(colour.blue_u32(), 147);
        assert_eq!(colour.alpha_u32(), 33);
        assert_eq!(colour.to_rgba_u32(), rgba_hex);
    }
}
