
use std::collections::HashMap;
use super::super::*;

#[test]
fn test_trait() {
    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert("x".to_string(), "X".to_string());

    assert_eq!("hi {x}".format(&vars), "hi X");
    assert_eq!("hi {x}".to_string().format(&vars), "hi X");
}
