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

    fn rgb_hex_to_red_f32(val: u32) -> f32 {
        Colour::rgba_u8_to_f32(((val >> 16) & 0xff) as u8)
    }

    fn rgb_hex_to_green_f32(val: u32) -> f32 {
        Colour::rgba_u8_to_f32(((val >> 8) & 0xff) as u8)
    }

    fn rgb_hex_to_blue_f32(val: u32) -> f32 {
        Colour::rgba_u8_to_f32((val & 0xff) as u8)
    }

    fn rgba_u8_to_f32(val: u8) -> f32 {
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
            red: Colour::rgba_u8_to_f32(red),
            green: Colour::rgba_u8_to_f32(green),
            blue: Colour::rgba_u8_to_f32(blue),
            alpha: Colour::rgba_u8_to_f32(alpha),
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

    pub fn red(&self) -> f32 {
        self.red
    }

    pub fn green(&self) -> f32 {
        self.green
    }

    pub fn blue(&self) -> f32 {
        self.blue
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
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
