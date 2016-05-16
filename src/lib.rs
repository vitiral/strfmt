// use std::io::Write;
use std::fmt::Write;
use std::str;
use std::iter::Iterator;
use std::collections::HashMap;
use std::string::String;

#[derive(Debug)]
enum Align {
    Left,
    Center,
    Right,
    None,
}

pub struct FmtError(String);

/// LOC-fmtu
#[derive(Debug)]
struct Fmt <'a>{
    pub identifier: &'a str,
    pub fill: Option<char>,
    pub align: Align,
    pub width: Option<usize>,
    pub precision: Option<usize>,
}

fn write_char(s: &mut String, c: char, n: usize) {
    for _ in 0..n {
        s.push(c);
    }
}

fn write_from<'a, I>(s: &mut String, f: I, n: usize) -> usize
        where I: Iterator<Item=char> {
    // eexaust f or run out of n, return chars written
    if n == 0 {
        return 0;
    }
    let mut n_written: usize = 0;
    for c in f {
        s.push(c);
        n_written += 1;
        if n_written == n {
            return n_written;
        }
    }
    n_written
}

impl<'a> Fmt <'a> {
    /// write the formatted string to s and return true. If there is an error, clear `s` and write the
    /// error andw rite the error and return false
    pub fn write(&self, s: &mut String, vars: &'a HashMap<String, String>) -> bool {
        let ref value = match vars.get(self.identifier) {
            Some(v) => v,
            None => {
                // let mut msg: Vec<u8> = Vec::new();
                // write!(&mut msg, "invalid identifier: {}", self.identifier).unwrap();
                // s.clear();
                // s.extend(str::from_utf8(&msg).unwrap().iter());

                s.clear();
                write!(s, "invalid identifier: {}", self.identifier).unwrap();
                return false;
            }
        };
        let len = value.len();
        let mut value = value.chars();
        let mut written: usize = 0;
        let mut pad: usize = 0;
        let fill = self.fill.unwrap_or(' ');
        match self.width {
            Some(width) => {
                match self.align {
                    Align::Left => pad = width,
                    Align::Center => {
                        pad = width / 2;
                        write_char(s, fill, pad);
                        pad += width % 2;
                    },
                    Align::Right | Align::None => {
                        pad = width;
                    },
                }
            },
            None => {
                // no alignment, precision setting is possible
                match self.precision {
                    Some(prec) => {
                        let n = write_from(s, &mut value, prec);
                        if n < prec {
                            // only write more if align == Left
                            match self.align {
                                Align::Left => pad = prec - n,
                                _ => return true, // wrote all characters
                            }
                        }
                        else {
                            return true; // precision has written maximum characters
                        }
                    },
                    None => {}, // no special settings
                }
            },
        }
        // Done reading settings, now just write and then pad
        s.extend(value);
        write_char(s, fill, pad);
        true
    }
}


#[test]
fn test_fmt () {
    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert("x".to_string(), "X".to_string());

    let fmt = Fmt{
        identifier: "x",
        fill: None,
        align: Align::None,
        width: None,
        precision: None,
    };

    let mut s = String::new();
    fmt.write(&mut s, &vars);
    assert!(s == "X");
}
