//! strfmt crate

use std::fmt::Write;
use std::collections::HashMap;
use std::string::String;

#[cfg(test)]
mod tests;
mod types;
mod formatter;
mod parser;
mod fmtstr;

pub use types::{Result, FmtError};
pub use fmtstr::strfmt_map;
pub use formatter::Formatter;


/// rust-style format a string given a HashMap of the variables
pub fn strfmt(fmtstr: &str, vars: &HashMap<String, String>) -> Result<String> {
    let formatter = |mut fmt: Formatter| {
        let v = match vars.get(fmt.key) {
            Some(v) => v,
            None => {
                let mut msg = String::new();
                write!(msg, "Invalid key: {}", fmt.key).unwrap();
                return Err(FmtError::KeyError(msg));
            },
        };
        fmt.str(v.as_str())
    };
    strfmt_map(fmtstr, &formatter)
}

pub trait Format {
    fn format(&self, vars: &HashMap<String, String>) -> Result<String>;
}

impl Format for String {
    fn format(&self, vars: &HashMap<String, String>) -> Result<String> {
        strfmt(self.as_str(), vars)
    }
}

impl Format for str {
    fn format(&self, vars: &HashMap<String, String>) -> Result<String> {
        strfmt(self, vars)
    }
}
