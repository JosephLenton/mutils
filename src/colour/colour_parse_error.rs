use ::std::convert::From;
use ::std::num::ParseIntError;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ColourParseError {
    InvalidFormat,
}

impl From<ParseIntError> for ColourParseError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidFormat
    }
}
