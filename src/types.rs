use std::fmt;
use std::error;
use std::fmt::Write;
use std::string::String;
use std::result;


#[derive(Debug, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
    None,
}

pub type Result<T> = result::Result<T, FmtError>;

/// LOC-error
#[derive(Debug, PartialEq)]
pub enum FmtError {
    Invalid(String),  // format string is structued incorrectly
    KeyError(String), // key error in formatting string
}

impl fmt::Display for FmtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FmtError::Invalid(ref s) => write!(f, "Invalid({})", s),
            &FmtError::KeyError(ref s) => write!(f, "KeyError({})", s),
        }
    }
}

impl error::Error for FmtError {
    fn description(&self) -> &str {
        match self {
            &FmtError::Invalid(_) => "invalid format string",
            &FmtError::KeyError(_) => "invalid key",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

