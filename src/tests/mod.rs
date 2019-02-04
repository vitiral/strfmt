mod fmt;
mod key;
mod strfmt;
mod test_trait;

use super::FmtError;

#[test]
fn test_error() {
    // just make sure this compiles mostly
    let err = FmtError::Invalid("fmt error".to_string());
    let v = err.to_string();
    println!("{}", v);
}
