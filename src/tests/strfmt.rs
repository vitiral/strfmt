use std::fmt;
use std::collections::HashMap;
use super::super::*;


macro_rules! matches {
    ($e:expr, $p:pat) => {
        match $e { $p => true, _ => false }
    }
}

fn run_tests<T: fmt::Display>(values: &Vec<(&str, &str, u8)>,
             vars: &HashMap<String, T>,
             call: &Fn(&str, &HashMap<String, T>)
                      -> Result<String>) {
    for &(fmtstr, expected, expect_err) in values.iter() {
    // for input in values {
    //     let fmtstr = input.0;
    //     let expected = input.1;
    //     let expect_err = input.2;
        let result = call(fmtstr, vars);
        let mut failure = match expect_err {
            0 => result.is_err(),
            1 => !matches!(result, Err(FmtError::Invalid(_))),
            2 => !matches!(result, Err(FmtError::KeyError(_))),
            3 => !matches!(result, Err(FmtError::TypeError(_))),
            c@_ => panic!("error code {} DNE", c),
        };
        let result = match result {
            Err(e) => e.to_string(),
            Ok(s) => s,
        };
        if !failure && expect_err == 0 {
            failure = !(expected == result);
        }

        if failure {
            println!("FAIL:");
            println!("     input: {:?}", fmtstr);
            println!("    output: {:?}", result);
            if expect_err != 0 {
                let expected = match expect_err {
                    1 => "FmtError::Invalid",
                    2 => "FmtError::KeyError",
                    3 => "FmtError::TypeError",
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

        // valid types
        ("{x:<4s}", "X   ", 0),

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

        // invalid types
        ("{x:#}", "", 3),
        ("{x:<4n}", "", 3),
        ("{x:<4d}", "", 3),
        ("{x:,}", "", 3),
        ("{x:<-10}", "", 3),
    ];

    run_tests(&values, &vars, &strfmt);
}

#[test]
/// test using integers directly into format (uses Display)
fn test_ints_basic() {
    let mut vars: HashMap<String, u64> = HashMap::new();
    vars.insert("x".to_string(), 6);
    vars.insert("long".to_string(), 100000);  // len=10
    vars.insert("hi".to_string(), 42);

    // format, expected, error
    // error codes: 0 == no error, 1 == Invalid, 2 == KeyError
    let values: Vec<(&str, &str, u8)> = vec![
        // simple positioning
        ("{x}", "6", 0),
        ("{long}", "100000", 0),
        (" the answer is {hi}, haven't you read anything?",
         " the answer is 42, haven't you read anything?", 0),
    ];

    run_tests(&values, &vars, &strfmt);
}

#[test]
fn test_ignore_missing() {
    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert("x".to_string(), "X".to_string());
    let values: Vec<(&str, &str, u8)> = vec![
        // simple positioning
        ("{y}", "{y}", 0),
        ("{y} {x}", "{y} X", 0),
        ("{x} {longish:<32.3} {x} is nice", "X {longish:<32.3} X is nice", 0),
    ];
    let f = |mut fmt: Formatter| {
        match vars.get(fmt.key) {
            Some(v) => fmt.str(v),
            None => fmt.skip(),
        }
    };

    let strfmt_ignore = |fmtstr: &str, vars: &HashMap<String, String>| -> Result<String> {
        strfmt_map(fmtstr, &f)
    };
    run_tests(&values, &vars, &strfmt_ignore);
}

#[test]
fn test_f64() {
    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("x".to_string(), 42.4242);
    vars.insert("y".to_string(), -100.11111);
    vars.insert("z".to_string(), 0.);
    let values: Vec<(&str, &str, u8)> = vec![
        // simple valid
        ("{x}", "42.4242", 0),
        ("{x:.2}", "42.42", 0),
        ("{x:<7.2}", "42.42  ", 0),
        ("{x:.2e}", "4.24e1", 0),
        ("{x:.2E}", "4.24E1", 0),
        ("{x:+}", "+42.4242", 0),
        ("{y:.2E}", "-1.00E2", 0),
        ("{y:+.2E}", "-1.00E2", 0),
        ("{z:+.2E}", "+0.00E0", 0),

        // invalid
        ("{x:s}", "", 3),
        ("{x:#}", "", 3),
    ];
    let f = |mut fmt: Formatter| {
        match vars.get(fmt.key) {
            Some(v) => fmt.f64(*v),
            None => panic!(),
        }
    };

    let strfmt_f64 = |fmtstr: &str, vars: &HashMap<String, f64>| -> Result<String> {
        strfmt_map(fmtstr, &f)
    };
    run_tests(&values, &vars, &strfmt_f64);
}


// #[bench]
// fn bench_strfmt(b: &mut Bencher) {
//     let mut vars: HashMap<String, String> = HashMap::new();
//     let too_long = "toooloooong".to_string();
//     vars.insert("x".to_string(), "X".to_string());
//     let fmtstr = "short: {x:*^10.3} long: {long:%<14.9}";
//     b.iter(|| strfmt(fmtstr, &vars));
// }

