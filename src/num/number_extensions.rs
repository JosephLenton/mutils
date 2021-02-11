pub trait NumberExtensions {
    fn range_transform(self, src_from: Self, src_to: Self, dest_from: Self, dest_to: Self) -> Self;

    fn percentage_transform(self, from: Self, to: Self) -> Self;

    /**
     * 1 - self.
     */
    fn inverse(self) -> Self;

    /**
     * Returns self limited to between min and max.
     */
    fn limit(self, min: Self, max: Self) -> f32;
}

impl NumberExtensions for f32 {
    fn range_transform(self, src_from: Self, src_to: Self, dest_from: Self, dest_to: Self) -> Self {
        let percent = (self - src_from) / (src_to - src_from);
        percent.percentage_transform(dest_from, dest_to)
    }

    fn percentage_transform(self, from: Self, to: Self) -> Self {
        from + (self * (to - from))
    }

    fn inverse(self) -> Self {
        1.0 - self
    }

    fn limit(self, min: f32, max: f32) -> Self {
        self.max(min).min(max)
    }
}
