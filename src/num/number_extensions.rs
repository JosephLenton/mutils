use ::std::ops::Range;

pub trait NumberExtensions {
    fn range_transform(self, src_from: Self, src_to: Self, dest_from: Self, dest_to: Self) -> Self;

    fn percentage_transform(self, from: Self, to: Self) -> Self;

    /**
     * 1 - self.
     */
    fn inverse(self) -> Self;

    /**
     * For use on 0.0..=1.0 ranges, to transform into a different
     * 0.0..=1.0 range.
     *
     *  - If the number is below the range, then 0.0 is returned.
     *  - If the number is above the range, then 1.0 is returned.
     *  - Between these n will return 0.0 to 1.0.
     */
    fn scale_percentage(self, range: Range<f32>) -> f32;

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

    /// From converting from 0.0 to 1.0, to say 20.0 to 50.0.
    fn percentage_transform(self, from: Self, to: Self) -> Self {
        from + (self * (to - from))
    }

    fn scale_percentage(self, range: Range<Self>) -> Self {
        if self < range.start {
            0.0
        } else if range.end < self {
            1.0
        } else {
            let diff = range.end - range.start;
            (self - range.start) / diff
        }
    }

    fn inverse(self) -> Self {
        1.0 - self
    }

    fn limit(self, min: f32, max: f32) -> Self {
        self.max(min).min(max)
    }
}
