//! strfmt crate

use std::collections::{BTreeMap, HashMap};
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

/// Rust-style format a string given some type of map of the variables.
pub fn strfmt<K: FromStr, T: fmt::Display, M: Map<Key=K, Out=T>>(fmtstr: &str, vars: M) -> Result<String> {
    let formatter = |mut fmt: Formatter| {
        let k: K = match fmt.key.parse() {
            Ok(k) => k,
            Err(_) => {
                return Err(new_key_error(fmt.key));
            }
        };
        let v = match vars.get_map(&k) {
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
    fn format<K: FromStr, D: fmt::Display, M: Map<Key=K, Out=D>>(&self, vars: M) -> Result<String>;
}

impl Format for String {
    fn format<K: FromStr, D: fmt::Display, M: Map<Key=K, Out=D>>(&self, vars: M) -> Result<String> {
        strfmt(self.as_str(), vars)
    }
}

impl Format for str {
    fn format<K: FromStr, D: fmt::Display, M: Map<Key=K, Out=D>>(&self, vars: M) -> Result<String> {
        strfmt(self, vars)
    }
}

fn new_key_error(key: &str) -> FmtError {
    let mut msg = String::new();
    write!(msg, "Invalid key: {}", key).unwrap();
    FmtError::KeyError(msg)
}

pub trait Map {
   type Key: ?Sized;
   type Out;
   fn get_map(&self, k: &Self::Key) -> Option<Self::Out>;
}

impl<'b, K: Eq + Hash, T, S: std::hash::BuildHasher> Map for &'b HashMap<K, T, S> {
   type Key = K;
   type Out = &'b T;
   fn get_map(&self, k: &Self::Key) -> Option<Self::Out>{
       self.get(k)
   }
}

impl<'b, K: Ord, T> Map for &'b BTreeMap<K, T> {
   type Key = K;
   type Out = &'b T;
   fn get_map(&self, k: &Self::Key) -> Option<Self::Out> {
       self.get(k)
   }
}

impl<'b, K: ?Sized, T> Map for &dyn Fn(&K) -> Option<T> {
   type Key = K;
   type Out = T;
   fn get_map(&self, k: &Self::Key) -> Option<Self::Out> {
       self(k)
   }
}
