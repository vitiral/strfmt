// use std::collections::HashMap;
// use super::super::types::{Align, FmtChunk};

// #[test]
// fn test_fmt_align() {
//     let mut vars: HashMap<String, String> = HashMap::new();
//     vars.insert("x".to_string(), "X".to_string());
//     vars.insert("long".to_string(), "tooooloong".to_string());

//     let mut fmt = FmtChunk {
//         identifier: "x",
//         fill: None,
//         align: Align::None,
//         sign: None,
//         width: None,
//         precision: None,
//         ty: None,
//     };

//     // test basic
//     let mut s = String::new();
//     fmt.write(&mut s, &vars).unwrap();
//     assert!(s == "X");

//     // test alignment
//     s.clear();
//     fmt.width = Some(5);
//     fmt.write(&mut s, &vars).unwrap();
//     assert!(s == "    X");

//     s.clear();
//     fmt.align = Align::Right;
//     fmt.write(&mut s, &vars).unwrap();
//     assert!(s == "    X");

//     s.clear();
//     fmt.align = Align::Center;
//     fmt.write(&mut s, &vars).unwrap();
//     assert!(s == "  X  ");

//     s.clear();
//     fmt.align = Align::Left;
//     fmt.write(&mut s, &vars).unwrap();
//     assert!(s == "X    ");

//     // more center tests
//     s.clear();
//     fmt.align = Align::Center;
//     fmt.width = Some(6);
//     fmt.write(&mut s, &vars).unwrap();
//     assert!(s == "  X   ");

//     // with precision

//     // normally is ignored
//     s.clear();
//     fmt.width = Some(5);
//     fmt.align = Align::None;
//     fmt.precision = Some(6);
//     fmt.write(&mut s, &vars).unwrap();
//     assert!(s == "    X");

//     // unless width is < len
//     s.clear();
//     fmt.identifier = "long";
//     fmt.write(&mut s, &vars).unwrap();
//     assert!(s == "tooool");
// }


// #[test]
// fn test_fmt_from_str() {
//     let mut expected = FmtChunk {
//         identifier: "x",
//         fill: None,
//         align: Align::None,
//         sign: None,
//         width: None,
//         precision: None,
//         ty: None,
//     };
//     let mut result: FmtChunk;
//     assert!(FmtChunk::from_str("x").unwrap() == expected);
//     assert!(FmtChunk::from_str("x:").unwrap() == expected);

//     expected.fill = Some('3');
//     expected.align = Align::Left;
//     result = FmtChunk::from_str("x:3<").unwrap();
//     assert!(result == expected);

//     expected.precision = Some(5);
//     expected.width = Some(3);
//     result = FmtChunk::from_str("x:3<3.5").unwrap();
//     assert!(result == expected);

//     expected = FmtChunk {
//         identifier: "x",
//         fill: None,
//         align: Align::None,
//         sign: None,
//         width: Some(33),
//         precision: Some(5),
//         ty: None,
//     };
//     result = FmtChunk::from_str("x:33.5").unwrap();
//     assert!(result == expected);

//     assert!(FmtChunk::from_str("x:<.3").is_ok());
//     assert!(FmtChunk::from_str("x:^.3").is_ok());
//     assert!(FmtChunk::from_str("xxx: <88.3").is_ok());
//     assert!(FmtChunk::from_str("xxx:  <88.3").is_err());
//     assert!(FmtChunk::from_str("xxx:a34").is_err());
// }
