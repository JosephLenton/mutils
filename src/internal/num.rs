use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

use super::Half;
use super::NumTrampolene;

use ::num_traits::identities::{One, Zero};

/// A common number trait.
///
/// A number is a collection of the traits below.
/// But you cannot get them all together in Rust.
/// So we have had to build our own.
///
/// The Points, Rects, and Shapes, all use these traits.
pub trait Num:
    Add<Self, Output = Self>
    + AddAssign
    + Sub<Self, Output = Self>
    + SubAssign
    + Mul<Self, Output = Self>
    + MulAssign
    + Div<Self, Output = Self>
    + DivAssign
    + Rem<Self, Output = Self>
    + PartialOrd
    + Display
    + Copy
    + PartialEq
    + NumTrampolene
    + Half
    + Zero
    + One
    + Sized
{
}

/// You might be wondering 'what does this do?'
/// So am I!
///
/// I think it's saying *'the trait above really does exists, and is
/// implemented, for `U`'*. This applies for cases where `U` matches the `Num`
/// trait.
impl<U> Num for U where
    U: Add<Self, Output = Self>
        + AddAssign
        + Sub<Self, Output = Self>
        + SubAssign
        + Mul<Self, Output = Self>
        + MulAssign
        + Div<Self, Output = Self>
        + DivAssign
        + Rem<Self, Output = Self>
        + PartialOrd
        + Display
        + Copy
        + PartialEq
        + NumTrampolene
        + Half
        + Zero
        + One
        + Sized
{
}
