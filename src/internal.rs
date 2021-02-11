mod from_clamped;
mod half;
/// A trait for representing the `saturating_add` function.
mod num;
mod num_trampolene;
mod to_signed_clamped;

pub use self::from_clamped::FromClamped;
pub use self::half::Half;
pub use self::num::Num;
pub use self::num_trampolene::NumTrampolene;
pub use self::to_signed_clamped::ToSignedClamped;
