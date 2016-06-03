//! strfmt crate

use std::collections::HashMap;
use std::string::String;

#[cfg(test)]
mod tests;
mod types;
mod parser;
mod fmtstr;

pub use types::{Result, FmtError};
pub use parser::strfmt_options;


/// rust-style format a string given a HashMap of the variables
pub fn strfmt(fmtstr: &str, vars: &HashMap<String, String>) -> Result<String> {
    strfmt_options(fmtstr, vars, false)
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
