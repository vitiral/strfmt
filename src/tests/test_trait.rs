use super::super::*;
use std::collections::HashMap;

#[test]
fn test_trait() {
    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert("x".to_string(), "X".to_string());

    assert_eq!("hi {x}".format(&vars).unwrap(), "hi X");
    assert_eq!("hi {x}".to_string().format(&vars).unwrap(), "hi X");
}
