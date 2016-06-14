//! strfmt crate

use std::fmt;
use std::fmt::Write;
use std::collections::HashMap;
use std::string::String;

#[cfg(test)]
mod tests;
mod types;
mod formatter;
mod fmtstr;

pub use types::{Result, FmtError, Alignment, Sign};
pub use fmtstr::strfmt_map;
pub use formatter::Formatter;


/// rust-style format a string given a HashMap of the variables
pub fn strfmt<T: fmt::Display>(fmtstr: &str, vars: &HashMap<String, T>) -> Result<String> {
    let formatter = |mut fmt: Formatter| {
        let v = match vars.get(fmt.key) {
            Some(v) => v,
            None => {
                let mut msg = String::new();
                write!(msg, "Invalid key: {}", fmt.key).unwrap();
                return Err(FmtError::KeyError(msg));
            },
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
