//! Test keys other than String.

use super::super::*;
use std::collections::HashMap;
use std::str::FromStr;

#[test]
fn test_key_u32() {
    let mut vars: HashMap<u32, String> = HashMap::new();
    vars.insert(0, "X".to_string());

    assert_eq!("hi {0}".format(&vars).unwrap(), "hi X");
    assert_eq!("hi {0}".to_string().format(&vars).unwrap(), "hi X");
    assert_eq!(
        "hi {1}".format(&vars),
        Err(FmtError::KeyError("Invalid key: 1".into()))
    );
    assert_eq!(
        "hi {you}".format(&vars),
        Err(FmtError::KeyError("Invalid key: you".into()))
    );
}

#[test]
fn test_key_i32() {
    let mut vars: HashMap<i32, String> = HashMap::new();
    vars.insert(-1, "X".to_string());

    assert_eq!("hi {-1}".format(&vars).unwrap(), "hi X");
    assert_eq!("hi {-1}".to_string().format(&vars).unwrap(), "hi X");
    assert_eq!(
        "hi {1}".format(&vars),
        Err(FmtError::KeyError("Invalid key: 1".into()))
    );
    assert_eq!(
        "hi {you}".format(&vars),
        Err(FmtError::KeyError("Invalid key: you".into()))
    );
}

#[derive(PartialEq, Eq, Hash)]
enum Key {
    Zero,
    One,
    Two,
}

impl FromStr for Key {
    type Err = FmtError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Zero" => Ok(Key::Zero),
            "One" => Ok(Key::One),
            "Two" => Ok(Key::Two),
            _ => Err(FmtError::KeyError(s.to_string())),
        }
    }
}

#[test]
fn test_key_enum() {
    let mut vars: HashMap<Key, String> = HashMap::new();
    vars.insert(Key::Zero, "X".to_string());

    assert_eq!("hi {Zero}".format(&vars).unwrap(), "hi X");
    assert_eq!("hi {Zero}".to_string().format(&vars).unwrap(), "hi X");
    assert_eq!(
        "hi {One}".format(&vars),
        Err(FmtError::KeyError("Invalid key: One".into()))
    );
    assert_eq!(
        "hi {you}".format(&vars),
        Err(FmtError::KeyError("Invalid key: you".into()))
    );
}
