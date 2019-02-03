//! strfmt crate

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::string::String;

mod fmtstr;
mod formatter;
#[cfg(test)]
mod tests;
mod types;

#[macro_use]
mod fmtnum;

pub use fmtstr::strfmt_map;
pub use formatter::Formatter;
pub use types::{Alignment, FmtError, Result, Sign};

// u128 & i128 unstable (see https://github.com/rust-lang/rust/issues/35118)
fmtint!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
fmtfloat!(f32 f64);

/// Rust-style format a string given a `HashMap` of the variables.
pub fn strfmt<T: fmt::Display>(fmtstr: &str, vars: &HashMap<String, T>) -> Result<String> {
    let formatter = |mut fmt: Formatter| {
        let v = match vars.get(fmt.key) {
            Some(v) => v,
            None => {
                let mut msg = String::new();
                write!(msg, "Invalid key: {}", fmt.key).unwrap();
                return Err(FmtError::KeyError(msg));
            }
        };
        fmt.str(v.to_string().as_str())
    };
    strfmt_map(fmtstr, &formatter)
}

pub trait Format {
    fn format<D: fmt::Display>(&self, vars: &HashMap<String, D>) -> Result<String>;
}

impl Format for String {
    fn format<D: fmt::Display>(&self, vars: &HashMap<String, D>) -> Result<String> {
        strfmt(self.as_str(), vars)
    }
}

impl Format for str {
    fn format<D: fmt::Display>(&self, vars: &HashMap<String, D>) -> Result<String> {
        strfmt(self, vars)
    }
}
