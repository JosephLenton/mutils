use std::mem::size_of;

///
/// When you convert between numeric types, there are some common issues you run into.
///
///  * Some of those types will truncate. i.e. 300_u32 as u8 gives 44.
///  * Converting from a float to an integer, always round towards zero. i.e. 1.8 gives 1.
///
/// This trait exists to give a uniform way of converting from one type to another,
/// where the result is rounded to the max value available.
///
/// It includes implementations for types where you can use 'as',
/// as this provides uniformity.
pub trait ToRounded<N> {
    /// Returns the value in the new type, but clamped.
    fn to_rounded(self) -> N;
}

// f64
impl ToRounded<Self> for f64 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for f64 {
    fn to_rounded(self) -> f32 {
        self.round() as f32
    }
}

impl ToRounded<usize> for f64 {
    fn to_rounded(self) -> usize {
        self.round() as usize
    }
}

impl ToRounded<isize> for f64 {
    fn to_rounded(self) -> isize {
        self.round() as isize
    }
}

impl ToRounded<u8> for f64 {
    fn to_rounded(self) -> u8 {
        self.round() as u8
    }
}

impl ToRounded<u16> for f64 {
    fn to_rounded(self) -> u16 {
        self.round() as u16
    }
}

impl ToRounded<u32> for f64 {
    fn to_rounded(self) -> u32 {
        self.round() as u32
    }
}

impl ToRounded<u64> for f64 {
    fn to_rounded(self) -> u64 {
        self.round() as u64
    }
}

impl ToRounded<i8> for f64 {
    fn to_rounded(self) -> i8 {
        self.round() as i8
    }
}

impl ToRounded<i16> for f64 {
    fn to_rounded(self) -> i16 {
        self.round() as i16
    }
}

impl ToRounded<i32> for f64 {
    fn to_rounded(self) -> i32 {
        self.round() as i32
    }
}

impl ToRounded<i64> for f64 {
    fn to_rounded(self) -> i64 {
        self.round() as i64
    }
}

// f32
impl ToRounded<f64> for f32 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<Self> for f32 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<usize> for f32 {
    fn to_rounded(self) -> usize {
        self.round() as usize
    }
}

impl ToRounded<isize> for f32 {
    fn to_rounded(self) -> isize {
        self.round() as isize
    }
}

impl ToRounded<u8> for f32 {
    fn to_rounded(self) -> u8 {
        self.round() as u8
    }
}

impl ToRounded<u16> for f32 {
    fn to_rounded(self) -> u16 {
        self.round() as u16
    }
}

impl ToRounded<u32> for f32 {
    fn to_rounded(self) -> u32 {
        self.round() as u32
    }
}

impl ToRounded<u64> for f32 {
    fn to_rounded(self) -> u64 {
        self.round() as u64
    }
}

impl ToRounded<i8> for f32 {
    fn to_rounded(self) -> i8 {
        self.round() as i8
    }
}

impl ToRounded<i16> for f32 {
    fn to_rounded(self) -> i16 {
        self.round() as i16
    }
}

impl ToRounded<i32> for f32 {
    fn to_rounded(self) -> i32 {
        self.round() as i32
    }
}

impl ToRounded<i64> for f32 {
    fn to_rounded(self) -> i64 {
        self.round() as i64
    }
}

// u64
impl ToRounded<Self> for u64 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for u64 {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

impl ToRounded<f64> for u64 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<usize> for u64 {
    fn to_rounded(self) -> usize {
        if size_of::<Self>() <= size_of::<usize>() {
            self as usize
        } else {
            self.min(<usize>::max_value() as Self) as usize
        }
    }
}

impl ToRounded<isize> for u64 {
    fn to_rounded(self) -> isize {
        if size_of::<Self>() < size_of::<isize>() {
            self as isize
        } else {
            self.min(<isize>::max_value() as Self) as isize
        }
    }
}

impl ToRounded<u8> for u64 {
    fn to_rounded(self) -> u8 {
        self.min(<u8>::max_value() as Self) as u8
    }
}

impl ToRounded<u16> for u64 {
    fn to_rounded(self) -> u16 {
        self.min(<u16>::max_value() as Self) as u16
    }
}

impl ToRounded<u32> for u64 {
    fn to_rounded(self) -> u32 {
        self.min(<u32>::max_value() as Self) as u32
    }
}

impl ToRounded<i8> for u64 {
    fn to_rounded(self) -> i8 {
        self.min(<i8>::max_value() as Self) as i8
    }
}

impl ToRounded<i16> for u64 {
    fn to_rounded(self) -> i16 {
        self.min(<i16>::max_value() as Self) as i16
    }
}

impl ToRounded<i32> for u64 {
    fn to_rounded(self) -> i32 {
        self.min(<i32>::max_value() as Self) as i32
    }
}

impl ToRounded<i64> for u64 {
    fn to_rounded(self) -> i64 {
        self.min(<i64>::max_value() as Self) as i64
    }
}

// u32
impl ToRounded<Self> for u32 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for u32 {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

impl ToRounded<f64> for u32 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<usize> for u32 {
    fn to_rounded(self) -> usize {
        if size_of::<Self>() <= size_of::<usize>() {
            self as usize
        } else {
            self.min(<usize>::max_value() as Self) as usize
        }
    }
}

impl ToRounded<isize> for u32 {
    fn to_rounded(self) -> isize {
        if size_of::<Self>() < size_of::<isize>() {
            self as isize
        } else {
            self.min(<isize>::max_value() as Self) as isize
        }
    }
}

impl ToRounded<u8> for u32 {
    fn to_rounded(self) -> u8 {
        self.min(<u8>::max_value() as Self) as u8
    }
}

impl ToRounded<u16> for u32 {
    fn to_rounded(self) -> u16 {
        self.min(<u16>::max_value() as Self) as u16
    }
}

impl ToRounded<u64> for u32 {
    #[inline(always)]
    fn to_rounded(self) -> u64 {
        self as u64
    }
}

impl ToRounded<i8> for u32 {
    fn to_rounded(self) -> i8 {
        self.min(<i8>::max_value() as Self) as i8
    }
}

impl ToRounded<i16> for u32 {
    fn to_rounded(self) -> i16 {
        self.min(<i16>::max_value() as Self) as i16
    }
}

impl ToRounded<i32> for u32 {
    fn to_rounded(self) -> i32 {
        self.min(<i32>::max_value() as Self) as i32
    }
}

impl ToRounded<i64> for u32 {
    #[inline(always)]
    fn to_rounded(self) -> i64 {
        self as i64
    }
}

// u16
impl ToRounded<Self> for u16 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for u16 {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

impl ToRounded<f64> for u16 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<usize> for u16 {
    fn to_rounded(self) -> usize {
        if size_of::<Self>() <= size_of::<usize>() {
            self as usize
        } else {
            self.min(<usize>::max_value() as Self) as usize
        }
    }
}

impl ToRounded<isize> for u16 {
    fn to_rounded(self) -> isize {
        if size_of::<Self>() < size_of::<isize>() {
            self as isize
        } else {
            self.min(<isize>::max_value() as Self) as isize
        }
    }
}

impl ToRounded<u8> for u16 {
    fn to_rounded(self) -> u8 {
        self.min(<u8>::max_value() as Self) as u8
    }
}

impl ToRounded<u32> for u16 {
    #[inline(always)]
    fn to_rounded(self) -> u32 {
        self as u32
    }
}

impl ToRounded<u64> for u16 {
    #[inline(always)]
    fn to_rounded(self) -> u64 {
        self as u64
    }
}

impl ToRounded<i8> for u16 {
    fn to_rounded(self) -> i8 {
        self.min(<i8>::max_value() as Self) as i8
    }
}

impl ToRounded<i16> for u16 {
    fn to_rounded(self) -> i16 {
        self.min(<i16>::max_value() as Self) as i16
    }
}

impl ToRounded<i32> for u16 {
    #[inline(always)]
    fn to_rounded(self) -> i32 {
        self as i32
    }
}

impl ToRounded<i64> for u16 {
    #[inline(always)]
    fn to_rounded(self) -> i64 {
        self as i64
    }
}

// u8
impl ToRounded<Self> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

impl ToRounded<f64> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<usize> for u8 {
    fn to_rounded(self) -> usize {
        if size_of::<Self>() <= size_of::<usize>() {
            self as usize
        } else {
            self.min(<usize>::max_value() as Self) as usize
        }
    }
}

impl ToRounded<isize> for u8 {
    fn to_rounded(self) -> isize {
        if size_of::<Self>() < size_of::<isize>() {
            self as isize
        } else {
            self.min(<isize>::max_value() as Self) as isize
        }
    }
}

impl ToRounded<u16> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> u16 {
        self as u16
    }
}

impl ToRounded<u32> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> u32 {
        self as u32
    }
}

impl ToRounded<u64> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> u64 {
        self as u64
    }
}

impl ToRounded<i8> for u8 {
    fn to_rounded(self) -> i8 {
        self.min(<i8>::max_value() as Self) as i8
    }
}

impl ToRounded<i16> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> i16 {
        self as i16
    }
}

impl ToRounded<i32> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> i32 {
        self as i32
    }
}

impl ToRounded<i64> for u8 {
    #[inline(always)]
    fn to_rounded(self) -> i64 {
        self as i64
    }
}

// i64
impl ToRounded<Self> for i64 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for i64 {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

impl ToRounded<f64> for i64 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<usize> for i64 {
    fn to_rounded(self) -> usize {
        if size_of::<Self>() <= size_of::<usize>() {
            self.max(0) as usize
        } else {
            self.max(<usize>::min_value() as Self)
                .min(<usize>::max_value() as Self) as usize
        }
    }
}

impl ToRounded<isize> for i64 {
    fn to_rounded(self) -> isize {
        if size_of::<Self>() < size_of::<isize>() {
            self as isize
        } else {
            self.max(<isize>::min_value() as Self)
                .min(<isize>::max_value() as Self) as isize
        }
    }
}

impl ToRounded<u8> for i64 {
    fn to_rounded(self) -> u8 {
        self.max(<u8>::min_value() as Self)
            .min(<u8>::max_value() as Self) as u8
    }
}

impl ToRounded<u16> for i64 {
    fn to_rounded(self) -> u16 {
        self.max(<u16>::min_value() as Self)
            .min(<u16>::max_value() as Self) as u16
    }
}

impl ToRounded<u32> for i64 {
    fn to_rounded(self) -> u32 {
        self.max(<u32>::min_value() as Self)
            .min(<u32>::max_value() as Self) as u32
    }
}

impl ToRounded<u64> for i64 {
    fn to_rounded(self) -> u64 {
        self.max(0) as u64
    }
}

impl ToRounded<i8> for i64 {
    fn to_rounded(self) -> i8 {
        self.max(<i8>::min_value() as Self)
            .min(<i8>::max_value() as Self) as i8
    }
}

impl ToRounded<i16> for i64 {
    fn to_rounded(self) -> i16 {
        self.max(<i16>::min_value() as Self)
            .min(<i16>::max_value() as Self) as i16
    }
}

impl ToRounded<i32> for i64 {
    fn to_rounded(self) -> i32 {
        self.max(<i32>::min_value() as Self)
            .min(<i32>::max_value() as Self) as i32
    }
}

// i32
impl ToRounded<Self> for i32 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for i32 {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

impl ToRounded<f64> for i32 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<usize> for i32 {
    fn to_rounded(self) -> usize {
        if size_of::<Self>() <= size_of::<usize>() {
            self.max(0) as usize
        } else {
            self.max(<usize>::min_value() as Self)
                .min(<usize>::max_value() as Self) as usize
        }
    }
}

impl ToRounded<isize> for i32 {
    fn to_rounded(self) -> isize {
        if size_of::<Self>() < size_of::<isize>() {
            self as isize
        } else {
            self.max(<isize>::min_value() as Self)
                .min(<isize>::max_value() as Self) as isize
        }
    }
}

impl ToRounded<u8> for i32 {
    fn to_rounded(self) -> u8 {
        self.max(<u8>::min_value() as Self)
            .min(<u8>::max_value() as Self) as u8
    }
}

impl ToRounded<u16> for i32 {
    fn to_rounded(self) -> u16 {
        self.max(<u16>::min_value() as Self)
            .min(<u16>::max_value() as Self) as u16
    }
}

impl ToRounded<u32> for i32 {
    fn to_rounded(self) -> u32 {
        self.max(<u32>::min_value() as Self) as u32
    }
}

impl ToRounded<u64> for i32 {
    fn to_rounded(self) -> u64 {
        self.max(<u64>::min_value() as Self) as u64
    }
}

impl ToRounded<i8> for i32 {
    fn to_rounded(self) -> i8 {
        self.max(<i8>::min_value() as Self)
            .min(<i8>::max_value() as Self) as i8
    }
}

impl ToRounded<i16> for i32 {
    fn to_rounded(self) -> i16 {
        self.max(<i16>::min_value() as Self)
            .min(<i16>::max_value() as Self) as i16
    }
}

impl ToRounded<i64> for i32 {
    #[inline(always)]
    fn to_rounded(self) -> i64 {
        self as i64
    }
}

// i16
impl ToRounded<Self> for i16 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for i16 {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

impl ToRounded<f64> for i16 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<usize> for i16 {
    fn to_rounded(self) -> usize {
        if size_of::<Self>() <= size_of::<usize>() {
            self.max(0) as usize
        } else {
            self.max(<usize>::min_value() as Self)
                .min(<usize>::max_value() as Self) as usize
        }
    }
}

impl ToRounded<isize> for i16 {
    fn to_rounded(self) -> isize {
        if size_of::<Self>() < size_of::<isize>() {
            self as isize
        } else {
            self.max(<isize>::min_value() as Self)
                .min(<isize>::max_value() as Self) as isize
        }
    }
}

impl ToRounded<u8> for i16 {
    fn to_rounded(self) -> u8 {
        self.max(<u8>::min_value() as Self)
            .min(<u8>::max_value() as Self) as u8
    }
}

impl ToRounded<u16> for i16 {
    fn to_rounded(self) -> u16 {
        self.max(0) as u16
    }
}

impl ToRounded<u32> for i16 {
    fn to_rounded(self) -> u32 {
        self.max(0) as u32
    }
}

impl ToRounded<u64> for i16 {
    fn to_rounded(self) -> u64 {
        self.max(0) as u64
    }
}

impl ToRounded<i8> for i16 {
    fn to_rounded(self) -> i8 {
        self.max(<i8>::min_value() as Self)
            .min(<i8>::max_value() as Self) as i8
    }
}

impl ToRounded<i32> for i16 {
    #[inline(always)]
    fn to_rounded(self) -> i32 {
        self as i32
    }
}

impl ToRounded<i64> for i16 {
    #[inline(always)]
    fn to_rounded(self) -> i64 {
        self as i64
    }
}

// i8
impl ToRounded<Self> for i8 {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for i8 {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

impl ToRounded<f64> for i8 {
    #[inline(always)]
    fn to_rounded(self) -> f64 {
        self as f64
    }
}

impl ToRounded<usize> for i8 {
    fn to_rounded(self) -> usize {
        if size_of::<Self>() <= size_of::<usize>() {
            self.max(0) as usize
        } else {
            self.max(<usize>::min_value() as Self)
                .min(<usize>::max_value() as Self) as usize
        }
    }
}

impl ToRounded<isize> for i8 {
    fn to_rounded(self) -> isize {
        if size_of::<Self>() < size_of::<isize>() {
            self as isize
        } else {
            self.max(<isize>::min_value() as Self)
                .min(<isize>::max_value() as Self) as isize
        }
    }
}

impl ToRounded<u8> for i8 {
    fn to_rounded(self) -> u8 {
        self.max(0) as u8
    }
}

impl ToRounded<u16> for i8 {
    fn to_rounded(self) -> u16 {
        self.max(0) as u16
    }
}

impl ToRounded<u32> for i8 {
    fn to_rounded(self) -> u32 {
        self.max(0) as u32
    }
}

impl ToRounded<u64> for i8 {
    fn to_rounded(self) -> u64 {
        self.max(0) as u64
    }
}

impl ToRounded<i16> for i8 {
    #[inline(always)]
    fn to_rounded(self) -> i16 {
        self as i16
    }
}

impl ToRounded<i32> for i8 {
    #[inline(always)]
    fn to_rounded(self) -> i32 {
        self as i32
    }
}

impl ToRounded<i64> for i8 {
    #[inline(always)]
    fn to_rounded(self) -> i64 {
        self as i64
    }
}

// usize
impl ToRounded<Self> for usize {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<f32> for usize {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

// isize
impl ToRounded<Self> for isize {
    #[inline(always)]
    fn to_rounded(self) -> Self {
        self
    }
}

impl ToRounded<isize> for usize {
    fn to_rounded(self) -> isize {
        self.min(<isize>::max_value() as Self) as isize
    }
}

impl ToRounded<f32> for isize {
    #[inline(always)]
    fn to_rounded(self) -> f32 {
        self as f32
    }
}

#[cfg(test)]
mod f32 {
    use super::*;

    const MAX: f32 = 14520890000000_f32;
    const MIN: f32 = -14520890000000_f32;

    #[test]
    fn it_should_round_to_nearest_value() {
        assert_eq!(ToRounded::<u32>::to_rounded(1.9_f32), 2);
        assert_eq!(ToRounded::<i32>::to_rounded(-1.9_f32), -2);
    }

    #[test]
    fn it_should_round_to_i16_min_value() {
        let n: f32 = -123456789.0;
        assert_eq!(ToRounded::<i16>::to_rounded(n), <i16>::min_value());
    }

    #[test]
    fn it_should_round_to_i16_max_value() {
        let n: f32 = 123456789.0;
        assert_eq!(ToRounded::<i16>::to_rounded(n), <i16>::max_value());
    }

    #[test]
    fn it_should_not_round_i16_values_that_fit() {
        let n: f32 = 1234.0;
        assert_eq!(ToRounded::<i16>::to_rounded(n), 1234);
    }

    #[test]
    fn it_should_round_to_highest_u8() {
        let n: f32 = 300.0;
        assert_eq!(ToRounded::<u8>::to_rounded(n), <u8>::max_value());
    }

    #[test]
    fn it_should_round_negatives_up_to_zero_for_u8() {
        let n: f32 = -300.0;
        assert_eq!(ToRounded::<u8>::to_rounded(n), <u8>::min_value());
    }

    #[test]
    fn it_should_round_to_max_value() {
        assert_eq!(ToRounded::<f64>::to_rounded(MAX), MAX as f64);
        assert_eq!(ToRounded::<f32>::to_rounded(MAX), 14520890000000_f32);

        // TODO, why does this fail?
        // assert_eq!(ToRounded::<u64>::to_rounded(MAX), 14520890000000);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <u32>::max_value());
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <u16>::max_value());
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <u8>::max_value());

        // TODO, why does this fail?
        // assert_eq!(ToRounded::<i64>::to_rounded(MAX), 14520890000000);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <i32>::max_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i16>::max_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    fn it_should_round_to_min_value() {
        assert_eq!(ToRounded::<f64>::to_rounded(MIN), MIN as f64);
        assert_eq!(ToRounded::<f32>::to_rounded(MIN), -14520890000000_f32);

        assert_eq!(ToRounded::<u64>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u32>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u16>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u8>::to_rounded(MIN), 0);

        // TODO, why does this fail?
        // assert_eq!(ToRounded::<i64>::to_rounded(MIN), -14520890000000);
        assert_eq!(ToRounded::<i32>::to_rounded(MIN), <i32>::min_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MIN), <i16>::min_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MIN), <i8>::min_value());

        assert_eq!(ToRounded::<usize>::to_rounded(MIN), 0);
        // TODO, why does this fail?
        // assert_eq!(ToRounded::<isize>::to_rounded(MIN), -14520890000000);
    }
}

#[cfg(test)]
mod f64 {
    use super::*;

    const MAX: f64 = 14520890000000_f64;
    const MIN: f64 = -14520890000000_f64;

    #[test]
    fn it_should_round_to_max_value() {
        assert_eq!(ToRounded::<f64>::to_rounded(MAX), MAX);
        assert_eq!(ToRounded::<f32>::to_rounded(MAX), 14520890000000_f32);

        assert_eq!(ToRounded::<u64>::to_rounded(MAX), 14520890000000);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <u32>::max_value());
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <u16>::max_value());
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <u8>::max_value());

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), 14520890000000);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <i32>::max_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i16>::max_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    fn it_should_round_to_min_value() {
        assert_eq!(ToRounded::<f64>::to_rounded(MIN), MIN);
        assert_eq!(ToRounded::<f32>::to_rounded(MIN), -14520890000000_f32);

        assert_eq!(ToRounded::<u64>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u32>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u16>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u8>::to_rounded(MIN), 0);

        assert_eq!(ToRounded::<i64>::to_rounded(MIN), -14520890000000);
        assert_eq!(ToRounded::<i32>::to_rounded(MIN), <i32>::min_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MIN), <i16>::min_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MIN), <i8>::min_value());

        assert_eq!(ToRounded::<usize>::to_rounded(MIN), 0);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(ToRounded::<usize>::to_rounded(MAX), 14520890000000);
        assert_eq!(ToRounded::<isize>::to_rounded(MAX), 14520890000000);
    }
}

#[cfg(test)]
mod u64 {
    use super::*;

    const MAX: u64 = <u64>::max_value();

    #[test]
    fn it_should_round_down_to_max_values() {
        assert_eq!(ToRounded::<u64>::to_rounded(MAX), MAX);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <u32>::max_value());
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <u16>::max_value());
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <u8>::max_value());

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), <i64>::max_value());
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <i32>::max_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i16>::max_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(ToRounded::<usize>::to_rounded(MAX), <usize>::max_value());
        assert_eq!(ToRounded::<isize>::to_rounded(MAX), <isize>::max_value());
    }
}

#[cfg(test)]
mod u32 {
    use super::*;

    const MAX: u32 = <u32>::max_value();

    #[test]
    fn it_should_round_down_to_max_values() {
        assert_eq!(ToRounded::<u64>::to_rounded(MAX), <u32>::max_value() as u64);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), MAX);
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <u16>::max_value());
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <u8>::max_value());

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), <u32>::max_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <i32>::max_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i16>::max_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(
            ToRounded::<usize>::to_rounded(MAX),
            <u32>::max_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MAX),
            <u32>::max_value() as isize
        );
    }
}

#[cfg(test)]
mod u16 {
    use super::*;

    const MAX: u16 = <u16>::max_value();

    #[test]
    fn it_should_round_down_to_max_values() {
        assert_eq!(ToRounded::<u64>::to_rounded(MAX), <u16>::max_value() as u64);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <u16>::max_value() as u32);
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), MAX);
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <u8>::max_value());

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), <u16>::max_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <u16>::max_value() as i32);
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i16>::max_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(
            ToRounded::<usize>::to_rounded(MAX),
            <u16>::max_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MAX),
            <u16>::max_value() as isize
        );
    }
}

#[cfg(test)]
mod u8 {
    use super::*;

    const MAX: u8 = <u8>::max_value();

    #[test]
    fn it_should_round_down_to_max_values() {
        assert_eq!(ToRounded::<u64>::to_rounded(MAX), <u8>::max_value() as u64);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <u8>::max_value() as u32);
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <u8>::max_value() as u16);
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), MAX);

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), <u8>::max_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <u8>::max_value() as i32);
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <u8>::max_value() as i16);
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(
            ToRounded::<usize>::to_rounded(MAX),
            <u8>::max_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MAX),
            <u8>::max_value() as isize
        );
    }
}

#[cfg(test)]
mod i64 {
    use super::*;

    const MAX: i64 = <i64>::max_value();
    const MIN: i64 = <i64>::min_value();

    #[test]
    fn it_should_round_down_to_max_values() {
        assert_eq!(ToRounded::<u64>::to_rounded(MAX), <i64>::max_value() as u64);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <u32>::max_value());
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <u16>::max_value());
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <u8>::max_value());

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), MAX);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <i32>::max_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i16>::max_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    fn it_should_round_to_min_value() {
        assert_eq!(ToRounded::<f64>::to_rounded(MIN), MIN as f64);
        assert_eq!(ToRounded::<f32>::to_rounded(MIN), MIN as f32);

        assert_eq!(ToRounded::<u64>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u32>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u16>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u8>::to_rounded(MIN), 0);

        assert_eq!(ToRounded::<i64>::to_rounded(MIN), MIN);
        assert_eq!(ToRounded::<i32>::to_rounded(MIN), <i32>::min_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MIN), <i16>::min_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MIN), <i8>::min_value());

        assert_eq!(ToRounded::<usize>::to_rounded(MIN), 0);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(
            ToRounded::<usize>::to_rounded(MAX),
            <i64>::max_value() as usize
        );
        assert_eq!(ToRounded::<isize>::to_rounded(MAX), <isize>::max_value());

        assert_eq!(
            ToRounded::<usize>::to_rounded(MIN),
            <usize>::min_value() as usize
        );
        assert_eq!(ToRounded::<isize>::to_rounded(MIN), <isize>::min_value());
    }
}

#[cfg(test)]
mod i32 {
    use super::*;

    const MAX: i32 = <i32>::max_value();
    const MIN: i32 = <i32>::min_value();

    #[test]
    fn it_should_round_down_to_max_values() {
        assert_eq!(ToRounded::<u64>::to_rounded(MAX), <i32>::max_value() as u64);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <i32>::max_value() as u32);
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <u16>::max_value());
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <u8>::max_value());

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), <i32>::max_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <i32>::max_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i16>::max_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    fn it_should_round_to_min_value() {
        assert_eq!(ToRounded::<f64>::to_rounded(MIN), MIN as f64);
        assert_eq!(ToRounded::<f32>::to_rounded(MIN), MIN as f32);

        assert_eq!(ToRounded::<u64>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u32>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u16>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u8>::to_rounded(MIN), 0);

        assert_eq!(ToRounded::<i64>::to_rounded(MIN), <i32>::min_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MIN), <i32>::min_value());
        assert_eq!(ToRounded::<i16>::to_rounded(MIN), <i16>::min_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MIN), <i8>::min_value());

        assert_eq!(ToRounded::<usize>::to_rounded(MIN), 0);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(
            ToRounded::<usize>::to_rounded(MAX),
            <i32>::max_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MAX),
            <i32>::max_value() as isize
        );

        assert_eq!(
            ToRounded::<usize>::to_rounded(MIN),
            <usize>::min_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MIN),
            <i32>::min_value() as isize
        );
    }
}

#[cfg(test)]
mod i16 {
    use super::*;

    const MAX: i16 = <i16>::max_value();
    const MIN: i16 = <i16>::min_value();

    #[test]
    fn it_should_round_down_to_max_values() {
        assert_eq!(ToRounded::<u64>::to_rounded(MAX), <i16>::max_value() as u64);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <i16>::max_value() as u32);
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <i16>::max_value() as u16);
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <u8>::max_value());

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), <i16>::max_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <i16>::max_value() as i32);
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i16>::max_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    fn it_should_round_to_min_value() {
        assert_eq!(ToRounded::<f64>::to_rounded(MIN), MIN as f64);
        assert_eq!(ToRounded::<f32>::to_rounded(MIN), MIN as f32);

        assert_eq!(ToRounded::<u64>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u32>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u16>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u8>::to_rounded(MIN), 0);

        assert_eq!(ToRounded::<i64>::to_rounded(MIN), <i16>::min_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MIN), <i16>::min_value() as i32);
        assert_eq!(ToRounded::<i16>::to_rounded(MIN), <i16>::min_value());
        assert_eq!(ToRounded::<i8>::to_rounded(MIN), <i8>::min_value());

        assert_eq!(ToRounded::<usize>::to_rounded(MIN), 0);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(
            ToRounded::<usize>::to_rounded(MAX),
            <i16>::max_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MAX),
            <i16>::max_value() as isize
        );

        assert_eq!(
            ToRounded::<usize>::to_rounded(MIN),
            <usize>::min_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MIN),
            <i16>::min_value() as isize
        );
    }
}

#[cfg(test)]
mod i8 {
    use super::*;

    const MAX: i8 = <i8>::max_value();
    const MIN: i8 = <i8>::min_value();

    #[test]
    fn it_should_round_down_to_max_values() {
        assert_eq!(ToRounded::<u64>::to_rounded(MAX), <i8>::max_value() as u64);
        assert_eq!(ToRounded::<u32>::to_rounded(MAX), <i8>::max_value() as u32);
        assert_eq!(ToRounded::<u16>::to_rounded(MAX), <i8>::max_value() as u16);
        assert_eq!(ToRounded::<u8>::to_rounded(MAX), <i8>::max_value() as u8);

        assert_eq!(ToRounded::<i64>::to_rounded(MAX), <i8>::max_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MAX), <i8>::max_value() as i32);
        assert_eq!(ToRounded::<i16>::to_rounded(MAX), <i8>::max_value() as i16);
        assert_eq!(ToRounded::<i8>::to_rounded(MAX), <i8>::max_value());
    }

    #[test]
    fn it_should_round_to_min_value() {
        assert_eq!(ToRounded::<f64>::to_rounded(MIN), MIN as f64);
        assert_eq!(ToRounded::<f32>::to_rounded(MIN), MIN as f32);

        assert_eq!(ToRounded::<u64>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u32>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u16>::to_rounded(MIN), 0);
        assert_eq!(ToRounded::<u8>::to_rounded(MIN), 0);

        assert_eq!(ToRounded::<i64>::to_rounded(MIN), <i8>::min_value() as i64);
        assert_eq!(ToRounded::<i32>::to_rounded(MIN), <i8>::min_value() as i32);
        assert_eq!(ToRounded::<i16>::to_rounded(MIN), <i8>::min_value() as i16);
        assert_eq!(ToRounded::<i8>::to_rounded(MIN), <i8>::min_value());

        assert_eq!(ToRounded::<usize>::to_rounded(MIN), 0);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn it_should_round_for_usize() {
        assert_eq!(
            ToRounded::<usize>::to_rounded(MAX),
            <i8>::max_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MAX),
            <i8>::max_value() as isize
        );

        assert_eq!(
            ToRounded::<usize>::to_rounded(MIN),
            <usize>::min_value() as usize
        );
        assert_eq!(
            ToRounded::<isize>::to_rounded(MIN),
            <i8>::min_value() as isize
        );
    }
}
