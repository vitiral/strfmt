use std::collections::HashMap;
use super::super::{strfmt, Fmt};

#[test]
fn test_values() {
    let mut vars: HashMap<String, String> = HashMap::new();
    let too_long = "toooloooong".to_string();
    vars.insert("x".to_string(), "X".to_string());
    vars.insert("long".to_string(), too_long.clone());  // len=10
    vars.insert("hi".to_string(), "hi".to_string());

    // format, expected, error
    let values = vec![
        // simple positioning
        ("{x}", "X", false),
        ("{x:}", "X", false),
        ("{x:3}", "  X", false),
        ("{x:>3}", "  X", false),
        ("{x:<3}", "X  ", false),
        ("{x:^3}", " X ", false),
        ("{x:^4}", " X  ", false),

        // extra text
        (" {x}yz", " Xyz", false),
        (" hi {x:^4}-you rock", " hi  X  -you rock", false),

        // fill confusion
        ("{x:10}", "         X", false),
        ("{long:.3}", "too", false),
        ("{long:<5.3}", "too  ", false),
        ("{long:5.3}", "  too", false),
        ("{long:5.7}", "toooloo", false),
        ("{long:<5.7}", "toooloo", false),
        ("{long:^5.7}", "toooloo", false),
        ("{long:<}", &too_long, false),

        // fun
        ("{x:<>}", "X", false),
        ("{x:<>3}", "<<X", false),

        // invalid
        ("{}", "", true),
        ("{x::}", "", true),
        ("{x:<<<}", "", true),
        ("{xxx:  <88.3}", "", true),
    ];

    for (fmtstr, expected, expect_err) in values {
        let result = strfmt(fmtstr, &vars);
        let mut err = expect_err != result.is_err();
        if !err && !expect_err {
            err = match &result {
                &Ok(ref r) => r != expected,
                _ => unreachable!(),
            };
        }

        if err {
            let ex_type = if expect_err {
                "fail"
            } else {
                "pass"
            };
            let fmt = Fmt::from_str(fmtstr);
            println!("FAIL: expected {}", ex_type);
            println!("    input: {:?}", (fmtstr, expected, expect_err));
            println!("    fmt: {:?}", fmt);
            if !expect_err {
                          println!("    expected: {:?}", expected);
            }
            match result {
                Ok(v) =>  println!("         got: {:?}", v),
                Err(v) => println!("         got: {:?}", v),
            }
            assert!(false);
        }
    }
}
