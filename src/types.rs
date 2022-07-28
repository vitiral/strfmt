use std::error;
use std::fmt;
use std::result;
use std::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum Alignment {
    Unspecified, // default Left for strings, Right for numbers
    Left,
    Center,
    Right,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sign {
    Unspecified,
    Plus,
    Minus,
    Space,
}

impl Sign {
    pub fn is_unspecified(&self) -> bool {
        match *self {
            Sign::Unspecified => false,
            _ => true,
        }
    }
}

pub type Result<T> = result::Result<T, FmtError>;

/// LOC-error
#[derive(Debug, PartialEq)]
pub enum FmtError {
    Invalid(String),   // format string is structued incorrectly
    KeyError(String),  // key error in formatting string
    TypeError(String), // invalid type used
}

impl fmt::Display for FmtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FmtError::Invalid(ref s) => write!(f, "Invalid({})", s),
            FmtError::KeyError(ref s) => write!(f, "KeyError({})", s),
            FmtError::TypeError(ref s) => write!(f, "TypeError({})", s),
        }
    }
}

impl error::Error for FmtError {
    fn description(&self) -> &str {
        match *self {
            FmtError::Invalid(_) => "invalid format string",
            FmtError::KeyError(_) => "invalid key",
            FmtError::TypeError(_) => "error during type resolution",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

// enum Type {
//     // integer types
//     Bin,
//     Char,
//     Decimal,
//     Octal,
//     Hex,
//     HexUpper,

//     // both
//     Number,

//     // Floating point types
//     Exponent,
//     ExponentUpper,
//     Fixed,
//     General,
//     GeneralUppercase,
//     Percengage,

//     // other types
//     None,
//     String,
//     Debug,
// }
