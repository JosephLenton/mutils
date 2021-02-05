use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    // @TODO: This uses FOUR f32 values for a colour. It can be replaced by ONE u32. Rewrite it!
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Colour {
    pub const WHITE: Colour = Colour::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Colour = Colour::new(0.0, 0.0, 0.0, 1.0);

    pub const MAGENTA: Colour = Colour::new(1.0, 0.0, 1.0, 1.0);
    pub const CYAN: Colour = Colour::new(0.0, 1.0, 1.0, 1.0);
    pub const YELLOW: Colour = Colour::new(1.0, 1.0, 0.0, 1.0);

    #[inline(always)]
    fn rgb_hex_to_red_f32(hex: u32) -> f32 {
        Colour::hex_u32_to_comp_f32(hex, 16)
    }

    #[inline(always)]
    fn rgb_hex_to_green_f32(hex: u32) -> f32 {
        Colour::hex_u32_to_comp_f32(hex, 8)
    }

    #[inline(always)]
    fn rgb_hex_to_blue_f32(hex: u32) -> f32 {
        Colour::hex_u32_to_comp_f32(hex, 0)
    }

    #[inline(always)]
    fn rgba_hex_to_red_f32(hex: u32) -> f32 {
        Colour::hex_u32_to_comp_f32(hex, 24)
    }

    #[inline(always)]
    fn rgba_hex_to_green_f32(hex: u32) -> f32 {
        Colour::hex_u32_to_comp_f32(hex, 16)
    }

    #[inline(always)]
    fn rgba_hex_to_blue_f32(hex: u32) -> f32 {
        Colour::hex_u32_to_comp_f32(hex, 8)
    }

    #[inline(always)]
    fn rgba_hex_to_alpha_f32(hex: u32) -> f32 {
        Colour::hex_u32_to_comp_f32(hex, 0)
    }

    #[inline(always)]
    fn hex_u32_to_comp_f32(val:u32, shift: u8) -> f32 {
        ((val >> shift) & 0xff) as f32 / 255.0
    }

    #[inline(always)]
    fn f32_to_hex_u32(val:f32, shift: u8) -> u32 {
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

    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    fn new_capped_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self::new(
            red.max(0.0).min(1.0),
            green.max(0.0).min(1.0),
            blue.max(0.0).min(1.0),
            alpha.max(0.0).min(1.0),
        )
    }

    pub fn new_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red: Colour::clamp_u8_to_f32(red),
            green: Colour::clamp_u8_to_f32(green),
            blue: Colour::clamp_u8_to_f32(blue),
            alpha: Colour::clamp_u8_to_f32(alpha),
        }
    }

    pub fn new_rgb_hex(rgb_hex: u32) -> Self {
        Self {
            red: Colour::rgb_hex_to_red_f32(rgb_hex),
            green: Colour::rgb_hex_to_green_f32(rgb_hex),
            blue: Colour::rgb_hex_to_blue_f32(rgb_hex),
            alpha: 1.0,
        }
    }

    pub fn new_rgba_hex(rgb_hex: u32) -> Self {
        Self {
            red: Colour::rgba_hex_to_red_f32(rgb_hex),
            green: Colour::rgba_hex_to_green_f32(rgb_hex),
            blue: Colour::rgba_hex_to_blue_f32(rgb_hex),
            alpha: Colour::rgba_hex_to_alpha_f32(rgb_hex),
        }
    }

    pub fn red(&self) -> f32 {
        self.red
    }

    pub fn red_u8(&self) -> u8 {
        Colour::f32_to_rgba_u8(self.red)
    }

    pub fn red_u32(&self) -> u32 {
        Colour::f32_to_rgba_u8(self.red) as u32
    }

    pub fn green(&self) -> f32 {
        self.green
    }

    pub fn green_u8(&self) -> u8 {
        Colour::f32_to_rgba_u8(self.green)
    }

    pub fn green_u32(&self) -> u32 {
        Colour::f32_to_rgba_u8(self.green) as u32
    }

    pub fn blue(&self) -> f32 {
        self.blue
    }

    pub fn blue_u8(&self) -> u8 {
        Colour::f32_to_rgba_u8(self.blue)
    }

    pub fn blue_u32(&self) -> u32 {
        Colour::f32_to_rgba_u8(self.blue) as u32
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn alpha_u8(&self) -> u8 {
        Colour::f32_to_rgba_u8(self.alpha)
    }

    pub fn alpha_u32(&self) -> u32 {
        Colour::f32_to_rgba_u8(self.alpha) as u32
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
        result.alpha = self.alpha;
        result
    }

    #[inline(always)]
    pub fn to_rgba_u32(self) -> u32 {
        Colour::f32_to_hex_u32(self.red(), 24) | Colour::f32_to_hex_u32(self.green(), 16) | Colour::f32_to_hex_u32(self.blue(), 8) | Colour::f32_to_hex_u32(self.alpha(), 0)
    }
}

impl Add<Self> for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new_capped_rgba(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
            self.alpha + other.alpha,
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
        Self::new_capped_rgba(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
            self.alpha - other.alpha,
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
        Self::new_capped_rgba(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
            self.alpha * other.alpha,
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
        Self::new_capped_rgba(
            self.red / other.red,
            self.green / other.green,
            self.blue / other.blue,
            self.alpha / other.alpha,
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
        Self::new_capped_rgba(
            self.red % other.red,
            self.green % other.green,
            self.blue % other.blue,
            self.alpha % other.alpha,
        )
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, val: f32) -> Self {
        Self::new_capped_rgba(
            self.red * val,
            self.green * val,
            self.blue * val,
            self.alpha * val,
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
        Self::new_capped_rgba(
            self.red / val,
            self.green / val,
            self.blue / val,
            self.alpha / val,
        )
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, val: f32) {
        *self = *self / val;
    }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn colour_red_u32() {
      let rgba_hex = 0xffa89321;
      let colour = Colour::new_rgba_hex(rgba_hex);

      assert_eq!(colour.red(), 1.0);
      assert_eq!(colour.red_u32(), 255);
  }

  #[test]
  pub fn colour_green_u32() {
      let rgba_hex = 0xffa89321;
      let colour = Colour::new_rgba_hex(rgba_hex);

      assert_eq!(colour.green(), 168.0 / 255.0);
      assert_eq!(colour.green_u32(), 168);
  }

  #[test]
  pub fn colour_blue_u32() {
      let rgba_hex = 0xffa89321;
      let colour = Colour::new_rgba_hex(rgba_hex);

      assert_eq!(colour.blue(), 147.0 / 255.0);
      assert_eq!(colour.blue_u32(), 147);
  }

  #[test]
  pub fn colour_alpha_u32() {
      let rgba_hex = 0xffa89321;
      let colour = Colour::new_rgba_hex(rgba_hex);

      assert_eq!(colour.alpha(), 33.0 / 255.0);
      assert_eq!(colour.alpha_u32(), 33);
  }

  #[test]
  pub fn colour_u32_matches_value_created_with() {
      let rgba_hex = 0xffa89321;
      let colour = Colour::new_rgba_hex(rgba_hex);

      assert_eq!(colour.red_u32(), 255);
      assert_eq!(colour.green_u32(), 168);
      assert_eq!(colour.blue_u32(), 147);
      assert_eq!(colour.alpha_u32(), 33);
      assert_eq!(colour.to_rgba_u32(), rgba_hex);
  }
}
