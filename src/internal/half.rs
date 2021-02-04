pub trait Half {
    fn half(&self) -> Self;
}

impl Half for usize {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for isize {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u8 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u16 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u32 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u64 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for i8 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for i16 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for i32 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for i64 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for f32 {
    fn half(&self) -> Self {
        self / 2.0
    }
}

impl Half for f64 {
    fn half(&self) -> Self {
        self / 2.0
    }
}
