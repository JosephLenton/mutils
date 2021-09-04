#![deny(private_in_public)]
#![deny(unused_must_use)]
#![deny(unused_mut)]
#![deny(unused_variables)]
#![deny(large_assignments)]
#![deny(non_camel_case_types)]
#![deny(dead_code)]
#![deny(non_snake_case)]

mod colour;
mod random;
mod vec2d;

pub mod geom;
pub mod num;

pub use self::colour::{Color, Colour};
pub use self::random::random;
pub use self::random::Random;
pub use self::vec2d::*;
