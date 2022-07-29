mod float;
mod fmt;
mod key;
mod legacy;
mod strfmt;
mod test_trait;
mod macros;

use super::FmtError;

#[test]
fn test_error() {
    // just make sure this compiles mostly
    let err = FmtError::Invalid("fmt error".to_string());
    let v = err.to_string();
    println!("{}", v);
}
