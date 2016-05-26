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
        ("{long:<<}", &too_long, false),
        ("{long:<<5}", &too_long, false),

        // fun
        ("{x:<>}", "X", false),
        ("{x:<>3}", "<<X", false),
        ("{{}}", "{}", false),
        ("{{{x}}}", "{X}", false),
        ("{{{x}{{{{{{", "{X{{{", false),
        ("{x}}}}}", "X}}", false),

        // invalid
        ("{}", "", true),
        ("{:3}", "", true),
        ("{x:*}", "", true),
        ("{x::}", "", true),
        ("{x:<<<}", "", true),
        ("{xxx:  <88.3}", "", true),
        ("}", "", true),
        ("{{}}}", "", true),
        ("hi } there", "", true),
        ("hi }", "", true),
        ("w { ho", "", true),

        // escape
        ("{{}}", "{}", false),
        ("{{long}}", "{long}", false),
        ("{{{x}}}", "{X}", false),

        // escape errors
    ];

    for (fmtstr, expected, expect_err) in values {
        let result = strfmt(fmtstr, &vars);
        let failure = expect_err != result.is_err();

        if failure {
            println!("FAIL:");
            println!("     input: {:?}", (fmtstr, expected, expect_err));
            println!("    output: {:?}", result);
            if expect_err {
                println!("    expected: error");
            } else {
                println!("    expected: {:?}", expected);
            }
            assert!(false);
        }
    }
}


// #[bench]
// fn bench_strfmt(b: &mut Bencher) {
//     let mut vars: HashMap<String, String> = HashMap::new();
//     let too_long = "toooloooong".to_string();
//     vars.insert("x".to_string(), "X".to_string());
//     let fmtstr = "short: {x:*^10.3} long: {long:%<14.9}";
//     b.iter(|| strfmt(fmtstr, &vars));
// }

