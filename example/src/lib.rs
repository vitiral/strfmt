extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

#[test]
fn it_works() {
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), "bob".to_string());
    vars.insert("job".to_string(), "python developer".to_string());

    let fmt = "hi, my name is {name} and I am a {job}!".to_string();
    assert!(strfmt(&fmt, &vars).unwrap() == "hi, my name is bob and I am a python developer!")
}
