//! strfmt crate

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::hash::Hash;
use std::str::FromStr;
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
pub fn strfmt<K, T: fmt::Display>(fmtstr: &str, vars: &HashMap<K, T>) -> Result<String>
where
    K: Hash + Eq + FromStr,
{
    let formatter = |mut fmt: Formatter| {
        let k: K = match fmt.key.parse() {
            Ok(k) => k,
            Err(_) => {
                return Err(new_key_error(fmt.key));
            }
        };
        let v = match vars.get(&k) {
            Some(v) => v,
            None => {
                return Err(new_key_error(fmt.key));
            }
        };
        fmt.str(v.to_string().as_str())
    };
    strfmt_map(fmtstr, &formatter)
}

pub trait Format {
    fn format<K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
    where
        K: Hash + Eq + FromStr;
}

impl Format for String {
    fn format<'a, K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
    where
        K: Hash + Eq + FromStr,
    {
        strfmt(self.as_str(), vars)
    }
}

impl Format for str {
    fn format<K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
    where
        K: Hash + Eq + FromStr,
    {
        strfmt(self, vars)
    }
}

fn new_key_error(key: &str) -> FmtError {
    let mut msg = String::new();
    write!(msg, "Invalid key: {}", key).unwrap();
    FmtError::KeyError(msg)
}
