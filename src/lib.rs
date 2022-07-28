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
/// # Arguments
///
/// * `fmtstr` - A string defining the format
/// * `vars` - A `HashMap` holding the variables to use
///
/// # Exceptions
///
/// * [FmtError::Invalid] - The format string is structured incorrectly
/// * [FmtError::KeyError] - `vars` contains an invalid key
/// * [FmtError::TypeError] - the given format code for a section contains an unexpected option
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use std::f64::consts::PI;
/// use strfmt::strfmt;
///
/// let mut my_vars: HashMap<String, f64> = HashMap::new();
/// my_vars.insert("Alpha".to_string(),42.0);
/// my_vars.insert("Beta".to_string(),PI);
///
/// println!("{}", strfmt("{Alpha} {Beta:<5.2}",&my_vars).unwrap());
/// ```
pub fn strfmt<'a, K, T: DisplayStr>(fmtstr: &str, vars: &HashMap<K, T>) -> Result<String>
    where
        K: Hash + Eq + FromStr
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
        v.display_str(&mut fmt)
    };
    strfmt_map(fmtstr, &formatter)
}

/// Rust-style format a string given a `HashMap` of the variables.
/// see [strfmt] for details
#[deprecated(since = "0.2.0", note = "This function contains a bug when formatting numbers. Use strfmt instead")]
pub fn strfmt_display<'a, K, T: fmt::Display>(fmtstr: &str, vars: &HashMap<K, T>) -> Result<String>
    where
        K: Hash + Eq + FromStr
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

macro_rules! display_str_impl {
    ($($t:ident)*) => ($(
        impl DisplayStr for $t {
            fn display_str(&self,f:&mut Formatter) -> Result<()> {
                f.$t(*self)
            }
        }
    )*)
}

display_str_impl!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
display_str_impl!(f32 f64);

impl DisplayStr for String{
    fn display_str(&self, f: &mut Formatter) -> Result<()> {
        f.str(self.as_str())
    }
}

impl DisplayStr for &str{
    fn display_str(&self, f: &mut Formatter) -> Result<()> {
        f.str(self)
    }
}
/// This trait is effectively an re-implementation for [std::fmt::Display]
/// It is used to disguise between the value types that should be formatted
pub trait DisplayStr {
    fn display_str(&self, f:&mut Formatter) -> Result<()>;
}

/// This trait is a shortcut for [strfmt]
/// for an example see [Format::format]
pub trait Format {
    /// format a string using strfmt
    /// # Arguments
    ///
    /// * `vars` - A `HashMap` holding the variables to use
    ///
    /// # Errors
    /// Errors are passed directly from strfmt, for details see [strfmt]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::f64::consts::PI;
    /// use strfmt::Format;
    ///
    /// let mut my_vars: HashMap<String, f64> = HashMap::new();
    /// my_vars.insert("Alpha".to_string(),42.0);
    /// my_vars.insert("Beta".to_string(),PI);
    ///
    /// println!("{}", "|{Alpha}|{Beta:<5.2}|".format(&my_vars).unwrap());
    /// ```
    fn format<K, D: DisplayStr>(&self, vars: &HashMap<K, D>) -> Result<String>
        where
            K: Hash + Eq + FromStr;

    /// format a string using strfmt_display
    /// see [Format::format] for usage
    #[deprecated(since = "0.2.0", note = "This function contains a bug when formatting numbers. Use format instead")]
    fn format_display<K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
        where
            K: Hash + Eq + FromStr;
}

impl Format for String {
    fn format<'a, K, D: DisplayStr>(&self, vars: &HashMap<K, D>) -> Result<String>
        where
            K: Hash + Eq + FromStr,
    {
        strfmt(self.as_str(), vars)
    }
    fn format_display<'a, K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
        where
            K: Hash + Eq + FromStr,
    {
        strfmt_display(self.as_str(), vars)
    }
}

impl Format for str {
    fn format<K, D: DisplayStr>(&self, vars: &HashMap<K, D>) -> Result<String>
        where
            K: Hash + Eq + FromStr,
    {
        strfmt(self, vars)
    }
    fn format_display<'a, K, D: fmt::Display>(&self, vars: &HashMap<K, D>) -> Result<String>
        where
            K: Hash + Eq + FromStr,
    {
        strfmt_display(self, vars)
    }
}

fn new_key_error(key: &str) -> FmtError {
    let mut msg = String::new();
    write!(msg, "Invalid key: {}", key).unwrap();
    FmtError::KeyError(msg)
}
