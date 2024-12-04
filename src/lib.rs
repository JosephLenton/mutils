#![deny(unused_must_use)]
#![deny(unused_macros)]
#![deny(unused_mut)]
#![deny(unused_variables)]
#![deny(large_assignments)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![warn(dead_code)]

mod colour;
mod random;
mod vec2d;

pub mod geom;
pub mod num;

pub use self::colour::{Color, Colour};
pub use self::random::random;
pub use self::random::Random;
pub use self::vec2d::*;

#[macro_use]
pub(crate) mod internal;
