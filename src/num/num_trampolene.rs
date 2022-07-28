use ::std::cmp::Ord;

/// The functions defined here don't have traits,
/// or the traits are only partially implemented on numeric types.
///
/// This exists so I can use any numbers with things like Point.
pub trait NumTrampolene {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn abs(self) -> Self;
}

impl NumTrampolene for u8 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self
    }
}

impl NumTrampolene for u16 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self
    }
}

impl NumTrampolene for u32 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self
    }
}

impl NumTrampolene for u64 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self
    }
}

impl NumTrampolene for u128 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self
    }
}

impl NumTrampolene for i8 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl NumTrampolene for i16 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl NumTrampolene for i32 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl NumTrampolene for i64 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl NumTrampolene for i128 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl NumTrampolene for usize {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self
    }
}

impl NumTrampolene for isize {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl NumTrampolene for f32 {
    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }

    fn abs(self) -> Self {
        self.abs()
    }
}

impl NumTrampolene for f64 {
    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }

    fn abs(self) -> Self {
        self.abs()
    }
}
