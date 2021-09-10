use ::lazy_static::lazy_static;
use ::rand::rngs::SmallRng;
use ::rand::Rng;
use ::rand::SeedableRng;
use ::std::ops::Add;
use ::std::ops::Sub;
use ::std::sync::Mutex;

use crate::geom::Line;
use crate::geom::Point;

lazy_static! {
    static ref GLOBAL_RNG: Mutex<SmallRng> = {
        let rng = SmallRng::seed_from_u64(100);
        Mutex::new(rng)
    };
}

#[derive(Copy, Clone)]
pub struct Random {
    min: f32,
    max: f32,
}

impl Random {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn random(&self) -> f32 {
        random(self.min, self.max)
    }
}

pub fn random(min: f32, max: f32) -> f32 {
    GLOBAL_RNG.lock().unwrap().gen_range(min..max)
}

impl Add<f32> for Random {
    type Output = f32;

    fn add(self, other: f32) -> f32 {
        other + self.random()
    }
}

impl Add<Point<f32>> for Random {
    type Output = Point;

    fn add(self, Point(x, y): Point) -> Point {
        Point(self + x, self + y)
    }
}

impl Add<Line<f32>> for Random {
    type Output = Line;

    fn add(self, Line(start, end): Line) -> Line {
        Line(self + start, self + end)
    }
}

impl Sub<f32> for Random {
    type Output = f32;

    fn sub(self, other: f32) -> f32 {
        other - self.random()
    }
}

impl Sub<Point<f32>> for Random {
    type Output = Point;

    fn sub(self, Point(x, y): Point) -> Point {
        Point(self - x, self - y)
    }
}

impl Sub<Line<f32>> for Random {
    type Output = Line;

    fn sub(self, Line(start, end): Line) -> Line {
        Line(self - start, self - end)
    }
}
