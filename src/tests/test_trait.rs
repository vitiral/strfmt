use super::super::*;
use std::collections::HashMap;

#[test]
fn test_trait() {
    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert("x".to_string(), "X".to_string());

    assert_eq!("hi {x}".format(&vars).unwrap(), "hi X");
    assert_eq!("hi {x}".to_string().format(&vars).unwrap(), "hi X");
}

#[test]
fn test_heterogenous_values() {
    let mut data: HashMap<String, &dyn DisplayStr> = HashMap::new();
    data.insert("body".to_string(), &"someString");
    data.insert("some_number".to_string(), &5.0);

    let output = strfmt("{body} = {some_number:2.4}", &data).unwrap();

    assert_eq!(output, "someString = 5.0000");
}
