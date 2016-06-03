use std::collections::HashMap;
use super::super::*;


macro_rules! matches {
    ($e:expr, $p:pat) => {
        match $e { $p => true, _ => false }
    }
}

fn run_tests(values: &Vec<(&str, &str, u8)>,
             vars: &HashMap<String, String>,
             call: &Fn(&str, &HashMap<String, String>)
                      -> Result<String>) {
    for &(fmtstr, expected, expect_err) in values.iter() {
    // for input in values {
    //     let fmtstr = input.0;
    //     let expected = input.1;
    //     let expect_err = input.2;
        let result = call(fmtstr, vars);
        let failure = match expect_err {
            0 => result.is_err(),
            1 => !matches!(result, Err(FmtError::Invalid(_))),
            2 => !matches!(result, Err(FmtError::KeyError(_))),
            c@_ => panic!("error code {} DNE", c),
        };

        if failure {
            println!("FAIL:");
            println!("     input: {:?}", fmtstr);
            println!("    output: {:?}", result);
            if expect_err != 0 {
                let expected = match expect_err {
                    1 => "FmtError::Invalid",
                    2 => "FmtError::KeyError",
                    _ => unreachable!()
                };
                println!("  expected: {}", expected)
            } else {
                println!("  expected: {:?}", expected);
            }
            assert!(false);
        }
    }

}

#[test]
fn test_values() {
    let mut vars: HashMap<String, String> = HashMap::new();
    let too_long = "toooloooong".to_string();
    vars.insert("x".to_string(), "X".to_string());
    vars.insert("long".to_string(), too_long.clone());  // len=10
    vars.insert("hi".to_string(), "hi".to_string());

    // format, expected, error
    // error codes: 0 == no error, 1 == Invalid, 2 == KeyError
    let values: Vec<(&str, &str, u8)> = vec![
        // simple positioning
        ("{x}", "X", 0),
        ("{x:}", "X", 0),
        ("{x:3}", "  X", 0),
        ("{x:>3}", "  X", 0),
        ("{x:<3}", "X  ", 0),
        ("{x:^3}", " X ", 0),
        ("{x:^4}", " X  ", 0),

        // extra text
        (" {x}yz", " Xyz", 0),
        (" hi {x:^4}-you rock", " hi  X  -you rock", 0),

        // fill confusion
        ("{x:10}", "         X", 0),
        ("{long:.3}", "too", 0),
        ("{long:<5.3}", "too  ", 0),
        ("{long:5.3}", "  too", 0),
        ("{long:5.7}", "toooloo", 0),
        ("{long:<5.7}", "toooloo", 0),
        ("{long:^5.7}", "toooloo", 0),
        ("{long:<}", &too_long, 0),
        ("{long:<<}", &too_long, 0),
        ("{long:<<5}", &too_long, 0),

        // escape
        ("{{}}", "{}", 0),
        ("{{long}}", "{long}", 0),
        ("{{{x}}}", "{X}", 0),

        // fun
        ("{x:<>}", "X", 0),
        ("{x:<>3}", "<<X", 0),
        ("{{}}", "{}", 0),
        ("{{{x}}}", "{X}", 0),
        ("{{{x}{{{{{{", "{X{{{", 0),
        ("{x}}}}}", "X}}", 0),

        // invalid fmt
        ("{}", "", 1),
        ("{:3}", "", 1),
        ("{x:*}", "", 1),
        ("{x::}", "", 1),
        ("{x:<<<}", "", 1),
        ("{xxx:  <88.3}", "", 1),

        // invalid escape
        ("}", "", 1),
        ("{{}}}", "", 1),
        ("hi } there", "", 1),
        ("hi }", "", 1),
        ("w { ho", "", 1),

        // invalid keys
        ("{what}", "{}", 2),
        ("{who}", "{}", 2),
        ("{x} {where}", "{}", 2),
    ];

    run_tests(&values, &vars, &strfmt);
}

// #[test]
// fn test_ignore_missing() {
//     let mut vars: HashMap<String, String> = HashMap::new();
//     vars.insert("x".to_string(), "X".to_string());
//     let values: Vec<(&str, &str, u8)> = vec![
//         // simple positioning
//         ("{y}", "{y}", 0),
//         ("{y} {x}", "{y} X", 0),
//         ("{x} {longish:<32.3} {x} is nice", "X {longish:<32.3} X is nice", 0),
//     ];
//     fn strfmt_ignore(fmtstr: &str, vars: &HashMap<String, String>) -> Result<String> {
//         strfmt_options(fmtstr, vars, true)
//     }
//     run_tests(&values, &vars, &strfmt_ignore);
// }


// #[bench]
// fn bench_strfmt(b: &mut Bencher) {
//     let mut vars: HashMap<String, String> = HashMap::new();
//     let too_long = "toooloooong".to_string();
//     vars.insert("x".to_string(), "X".to_string());
//     let fmtstr = "short: {x:*^10.3} long: {long:%<14.9}";
//     b.iter(|| strfmt(fmtstr, &vars));
// }

