use crate::num::Num;
use std::mem::size_of;

/// It's useful to convert from a higher precision numerical type,
/// to a lower one. i.e. u64 to u32.
///
/// When this happens you can lose data. There are multiple ways to deal with
/// this. One approach is to clamp within the target range.
///
/// This isn't just about moving to the same type with less bits. It's for any
/// change where you end up with less data. Such as moving between signed and
/// unsigned. i.e. i32 -> u32, and u32 -> i32.
pub trait FromClamped<N: Num> {
    /// Returns the value in the new type, but clamped.
    fn from_clamped(n: N) -> Self;
}

// All numbers can clamp themselves.
impl<N: Num> FromClamped<N> for N {
    #[inline(always)]
    fn from_clamped(n: N) -> Self {
        n
    }
}

// i8
impl FromClamped<i16> for i8 {
    fn from_clamped(n: i16) -> Self {
        n.max(<i16>::from(<i8>::min_value()))
            .min(<i16>::from(<i8>::max_value())) as Self
    }
}

impl FromClamped<i32> for i8 {
    fn from_clamped(n: i32) -> Self {
        n.max(<i32>::from(<i8>::min_value()))
            .min(<i32>::from(<i8>::max_value())) as Self
    }
}

impl FromClamped<i64> for i8 {
    fn from_clamped(n: i64) -> Self {
        n.max(<i64>::from(<i8>::min_value()))
            .min(<i64>::from(<i8>::max_value())) as Self
    }
}

impl FromClamped<u8> for i8 {
    fn from_clamped(n: u8) -> Self {
        n.min(<i8>::max_value() as u8) as Self
    }
}

impl FromClamped<u16> for i8 {
    fn from_clamped(n: u16) -> Self {
        n.min(<i8>::max_value() as u16) as Self
    }
}

impl FromClamped<u32> for i8 {
    fn from_clamped(n: u32) -> Self {
        n.min(<i8>::max_value() as u32) as Self
    }
}

impl FromClamped<u64> for i8 {
    fn from_clamped(n: u64) -> Self {
        n.min(<i8>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for i8 {
    fn from_clamped(n: f32) -> Self {
        n.max(<f32>::from(<Self>::min_value()))
            .min(<f32>::from(<Self>::max_value())) as Self
    }
}

impl FromClamped<f64> for i8 {
    fn from_clamped(n: f64) -> Self {
        n.max(<f64>::from(<Self>::min_value()))
            .min(<f64>::from(<Self>::max_value())) as Self
    }
}

// u8
impl FromClamped<i8> for u8 {
    fn from_clamped(n: i8) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<i16> for u8 {
    fn from_clamped(n: i16) -> Self {
        n.max(0).min(<i16>::from(<u8>::max_value())) as Self
    }
}

impl FromClamped<i32> for u8 {
    fn from_clamped(n: i32) -> Self {
        n.max(0).min(<i32>::from(<u8>::max_value())) as Self
    }
}

impl FromClamped<i64> for u8 {
    fn from_clamped(n: i64) -> Self {
        n.max(0).min(<i64>::from(<u8>::max_value())) as Self
    }
}

impl FromClamped<u16> for u8 {
    fn from_clamped(n: u16) -> Self {
        n.min(<u16>::from(<u8>::max_value())) as Self
    }
}

impl FromClamped<u32> for u8 {
    fn from_clamped(n: u32) -> Self {
        n.min(<u32>::from(<u8>::max_value())) as Self
    }
}

impl FromClamped<u64> for u8 {
    fn from_clamped(n: u64) -> Self {
        n.min(<u64>::from(<u8>::max_value())) as Self
    }
}

impl FromClamped<f32> for u8 {
    fn from_clamped(n: f32) -> Self {
        n.max(0.0).min(<f32>::from(<Self>::max_value())) as Self
    }
}

impl FromClamped<f64> for u8 {
    fn from_clamped(n: f64) -> Self {
        n.max(0.0).min(<f64>::from(<u8>::max_value())) as Self
    }
}

// i16
impl FromClamped<i32> for i16 {
    fn from_clamped(n: i32) -> Self {
        n.max(<i32>::from(<Self>::min_value()))
            .min(<i32>::from(<Self>::max_value())) as Self
    }
}

impl FromClamped<i64> for i16 {
    fn from_clamped(n: i64) -> Self {
        n.max(<i64>::from(<Self>::min_value()))
            .min(<i64>::from(<Self>::max_value())) as Self
    }
}

impl FromClamped<u16> for i16 {
    fn from_clamped(n: u16) -> Self {
        n.min(<i16>::max_value() as u16) as Self
    }
}

impl FromClamped<u32> for i16 {
    fn from_clamped(n: u32) -> Self {
        n.min(<i16>::max_value() as u32) as Self
    }
}

impl FromClamped<u64> for i16 {
    fn from_clamped(n: u64) -> Self {
        n.min(<i16>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for i16 {
    fn from_clamped(n: f32) -> Self {
        n.max(<f32>::from(<Self>::min_value()))
            .min(<f32>::from(<Self>::max_value())) as Self
    }
}

impl FromClamped<f64> for i16 {
    fn from_clamped(n: f64) -> Self {
        n.max(<f64>::from(<Self>::min_value()))
            .min(<f64>::from(<Self>::max_value())) as Self
    }
}

// u16
impl FromClamped<i16> for u16 {
    fn from_clamped(n: i16) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<i32> for u16 {
    fn from_clamped(n: i32) -> Self {
        n.max(0).min(<i32>::from(<u16>::max_value())) as Self
    }
}

impl FromClamped<i64> for u16 {
    fn from_clamped(n: i64) -> Self {
        n.max(0).min(<i64>::from(<u16>::max_value())) as Self
    }
}

impl FromClamped<u32> for u16 {
    fn from_clamped(n: u32) -> Self {
        n.min(<u32>::from(<u16>::max_value())) as Self
    }
}

impl FromClamped<u64> for u16 {
    fn from_clamped(n: u64) -> Self {
        n.min(<u64>::from(<u16>::max_value())) as Self
    }
}

impl FromClamped<f32> for u16 {
    fn from_clamped(n: f32) -> Self {
        n.max(0.0).min(<f32>::from(<u16>::max_value())) as Self
    }
}

impl FromClamped<f64> for u16 {
    fn from_clamped(n: f64) -> Self {
        n.max(0.0).min(<f64>::from(<u16>::max_value())) as Self
    }
}

// i32
impl FromClamped<i64> for i32 {
    fn from_clamped(n: i64) -> Self {
        n.max(<i64>::from(<i32>::min_value()))
            .min(<i64>::from(<i32>::max_value())) as Self
    }
}

impl FromClamped<u32> for i32 {
    fn from_clamped(n: u32) -> Self {
        n.min(<i32>::max_value() as u32) as Self
    }
}

impl FromClamped<u64> for i32 {
    fn from_clamped(n: u64) -> Self {
        n.min(<i32>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for i32 {
    fn from_clamped(n: f32) -> Self {
        n.max(<Self>::min_value() as f32)
            .min(<Self>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for i32 {
    fn from_clamped(n: f64) -> Self {
        n.max(<Self>::min_value() as f64)
            .min(<Self>::max_value() as f64) as Self
    }
}

// u32
impl FromClamped<i32> for u32 {
    fn from_clamped(n: i32) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<i64> for u32 {
    fn from_clamped(n: i64) -> Self {
        n.max(0).min(<i64>::from(<u32>::max_value())) as Self
    }
}

impl FromClamped<u64> for u32 {
    fn from_clamped(n: u64) -> Self {
        n.min(<u64>::from(<u32>::max_value())) as Self
    }
}

impl FromClamped<f32> for u32 {
    fn from_clamped(n: f32) -> Self {
        n.max(0.0).min(<u32>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for u32 {
    fn from_clamped(n: f64) -> Self {
        n.max(0.0).min(<f64>::from(<u32>::max_value())) as Self
    }
}

// i64
impl FromClamped<u64> for i64 {
    fn from_clamped(n: u64) -> Self {
        n.min(<i64>::max_value() as u64) as Self
    }
}

impl FromClamped<f32> for i64 {
    fn from_clamped(n: f32) -> Self {
        n.max(<Self>::min_value() as f32)
            .min(<Self>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for i64 {
    fn from_clamped(n: f64) -> Self {
        n.max(<Self>::min_value() as f64)
            .min(<Self>::max_value() as f64) as Self
    }
}

// u64
impl FromClamped<i64> for u64 {
    fn from_clamped(n: i64) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<f32> for u64 {
    fn from_clamped(n: f32) -> Self {
        n.max(0.0).min(<u64>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for u64 {
    fn from_clamped(n: f64) -> Self {
        n.max(0.0).min(<u64>::max_value() as f64) as Self
    }
}

// usize
impl FromClamped<i8> for usize {
    fn from_clamped(n: i8) -> Self {
        n.max(0) as Self
    }
}

impl FromClamped<i16> for usize {
    fn from_clamped(n: i16) -> Self {
        if size_of::<u16>() <= size_of::<usize>() {
            n.max(0) as Self
        } else {
            n.max(0).min(<usize>::max_value() as i16) as Self
        }
    }
}

impl FromClamped<i32> for usize {
    fn from_clamped(n: i32) -> Self {
        if size_of::<u16>() <= size_of::<usize>() {
            n.max(0) as Self
        } else {
            n.max(0).min(<usize>::max_value() as i32) as Self
        }
    }
}

impl FromClamped<i64> for usize {
    fn from_clamped(n: i64) -> Self {
        if size_of::<u16>() <= size_of::<usize>() {
            n.max(0) as Self
        } else {
            n.max(0).min(<usize>::max_value() as i64) as Self
        }
    }
}

impl FromClamped<u16> for usize {
    fn from_clamped(n: u16) -> Self {
        if size_of::<u16>() <= size_of::<usize>() {
            n as Self
        } else {
            n.min(<usize>::max_value() as u16) as Self
        }
    }
}

impl FromClamped<u32> for usize {
    fn from_clamped(n: u32) -> Self {
        if size_of::<u32>() <= size_of::<usize>() {
            n as Self
        } else {
            n.min(<usize>::max_value() as u32) as Self
        }
    }
}

impl FromClamped<u64> for usize {
    fn from_clamped(n: u64) -> Self {
        if size_of::<u64>() <= size_of::<usize>() {
            n as Self
        } else {
            n.min(<usize>::max_value() as u64) as Self
        }
    }
}

impl FromClamped<f32> for usize {
    fn from_clamped(n: f32) -> Self {
        n.max(0.0).min(<usize>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for usize {
    fn from_clamped(n: f64) -> Self {
        n.max(0.0).min(<usize>::max_value() as f64) as Self
    }
}

impl FromClamped<isize> for usize {
    fn from_clamped(n: isize) -> Self {
        n.max(0) as Self
    }
}

// isize
impl FromClamped<i8> for isize {
    fn from_clamped(n: i8) -> Self {
        n as Self
    }
}

impl FromClamped<i16> for isize {
    fn from_clamped(n: i16) -> Self {
        if size_of::<i16>() <= size_of::<Self>() {
            n as Self
        } else {
            n.max(<Self>::min_value() as i16)
                .min(<Self>::max_value() as i16) as Self
        }
    }
}

impl FromClamped<i32> for isize {
    fn from_clamped(n: i32) -> Self {
        if size_of::<i32>() <= size_of::<Self>() {
            n as Self
        } else {
            n.max(<Self>::min_value() as i32)
                .min(<Self>::max_value() as i32) as Self
        }
    }
}

impl FromClamped<i64> for isize {
    fn from_clamped(n: i64) -> Self {
        if size_of::<i64>() <= size_of::<Self>() {
            n as Self
        } else {
            n.max(<Self>::min_value() as i64)
                .min(<Self>::max_value() as i64) as Self
        }
    }
}

impl FromClamped<u8> for isize {
    fn from_clamped(n: u8) -> Self {
        n as Self
    }
}

impl FromClamped<u16> for isize {
    fn from_clamped(n: u16) -> Self {
        if size_of::<u16>() < size_of::<Self>() {
            n as Self
        } else {
            n.min(<Self>::max_value() as u16) as Self
        }
    }
}

impl FromClamped<u32> for isize {
    fn from_clamped(n: u32) -> Self {
        if size_of::<u32>() < size_of::<Self>() {
            n as Self
        } else {
            n.min(<Self>::max_value() as u32) as Self
        }
    }
}

impl FromClamped<u64> for isize {
    fn from_clamped(n: u64) -> Self {
        if size_of::<u64>() < size_of::<Self>() {
            n as Self
        } else {
            n.min(<Self>::max_value() as u64) as Self
        }
    }
}

impl FromClamped<f32> for isize {
    fn from_clamped(n: f32) -> Self {
        n.max(0.0).min(<Self>::max_value() as f32) as Self
    }
}

impl FromClamped<f64> for isize {
    fn from_clamped(n: f64) -> Self {
        n.max(0.0).min(<Self>::max_value() as f64) as Self
    }
}

impl FromClamped<usize> for isize {
    fn from_clamped(n: usize) -> Self {
        n.min(<Self>::max_value() as usize) as Self
    }
}

// f32
impl FromClamped<i16> for f32 {
    fn from_clamped(n: i16) -> Self {
        <f32>::from(n)
    }
}

impl FromClamped<i32> for f32 {
    fn from_clamped(n: i32) -> Self {
        n as f32
    }
}

impl FromClamped<i64> for f32 {
    fn from_clamped(n: i64) -> Self {
        n as f32
    }
}

impl FromClamped<u32> for f32 {
    fn from_clamped(n: u32) -> Self {
        n as f32
    }
}

impl FromClamped<u64> for f32 {
    fn from_clamped(n: u64) -> Self {
        n as f32
    }
}
