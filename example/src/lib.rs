extern crate strfmt;
use strfmt::{strfmt, strfmt_map, Formatter};
use std::collections::HashMap;

#[test]
fn it_works() {
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), "bob".to_string());
    vars.insert("job".to_string(), "python developer".to_string());

    let fmt = "hi, my name is {name} and I am a {job}!".to_string();
    assert!(strfmt(&fmt, &vars).unwrap() == "hi, my name is bob and I am a python developer!");


    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("x".to_string(), 42.4242);
    vars.insert("y".to_string(), -100.11111);
    vars.insert("z".to_string(), 0.);

    let f = |mut fmt: Formatter| {
        fmt.f64(*vars.get(fmt.key).unwrap())
    };
    assert_eq!(strfmt_map("{x:<7.2}", f).unwrap(), "42.42  ");
    assert_eq!(strfmt_map("{y:+.2E}", f).unwrap(), "-1.00E2");
    assert_eq!(strfmt_map("{z:+.2E}", f).unwrap(), "+0.00E0");
}
