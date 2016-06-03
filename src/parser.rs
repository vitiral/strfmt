use std::str;
use std::fmt::Write;
use std::iter::Iterator;
use std::collections::HashMap;
use std::string::String;

use types::*;

fn write_char(s: &mut String, c: char, n: usize) {
    for _ in 0..n {
        s.push(c);
    }
}

#[test]
fn test_write_char() {
    let mut s = String::new();
    s.write_str("h ").unwrap();
    write_char(&mut s, 'f', 3);
    assert!(s == "h fff");
}

fn write_from<'a, I>(s: &mut String, f: I, n: usize) -> usize
    where I: Iterator<Item = char>
{
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

#[test]
fn test_write_from() {
    let mut s = String::new();
    s.write_str("h ").unwrap();
    write_from(&mut s, "fff".chars(), 5);
    assert!(s == "h fff");
    write_from(&mut s, "xxxx".chars(), 2);
    assert!(s == "h fffxx");
    write_from(&mut s, "333".chars(), 3);
    assert!(s == "h fffxx333");
}

fn is_alignment_token(c: char) -> bool {
    match c {
        '=' | '<' | '^' | '>' => true,
        _ => false,
    }
}

fn is_sign_element(c: char) -> bool {
    match c {
        ' ' | '-' | '+' => true,
        _ => false,
    }
}

fn is_type_element(c: char) -> bool {
    match c {
        'b' | 'c' | 'd' | 'o' | 'x' | 'X' | 'n' |
        'e' | 'E' | 'f' | 'F' | 'g' | 'G' | '%' |
        's' | '?' => true,
        _ => false,
    }
}

// get an integer from pos, returning the number of bytes
// consumed and the integer
fn get_integer(s: &[u8], pos: usize) -> (usize, Option<i64>) {
    let (_, rest) = s.split_at(pos);
    let mut consumed: usize = 0;
    for b in rest {
        match *b as char {
            '0'...'9' => {},
            _ => break,
        };
        consumed += 1;
    }
    if consumed == 0 {
        (0, None)
    } else {
        let (intstr, _) = rest.split_at(consumed);
        let val = unsafe { // I think I can be reasonably sure that 0-9 chars are utf8 :)
            match str::from_utf8_unchecked(intstr).parse::<i64>() {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        };
        (consumed, val)
    }
}


#[derive(Debug)]
/// The format struct as it is defined in the python source
struct FmtPy {
    pub fill: char,
    pub align: char,
    pub alternate: bool,
    pub sign: char,
    pub width: i64,
    pub thousands: bool,
    pub precision: i64,
    pub ty: char,
}

fn parse_like_python(rest: &str) -> Result<FmtPy> {
    /* The rest of this was pretty much strait up copied from python's format parser
        All credit goes to python source file: formatter_unicode.c
    */

    let mut format = FmtPy {
        fill: ' ',
        align: '>',
        alternate: false,
        sign: '\0',
        width: -1,
        thousands: false,
        precision: -1,
        ty: ' ',
    };
    let mut chars = rest.chars();
    let fake_fill = match chars.next() {
        Some(c) => c,
        None => return Ok(format),
    };
    // from now on all format characters MUST be valid
    // ASCII characters (fill and identifier were the
    // only ones that weren't.
    // Therefore we can use bytes for the rest
    let rest = rest.as_bytes();
    let mut align_specified = false;
    let mut fill_specified = false;

    let end: usize = rest.len();
    let mut pos: usize = 0;

    /* If the second char is an alignment token,
        then fake_fill as fill */
    if end-pos >= 1 + fake_fill.len_utf8() && is_alignment_token(rest[pos + fake_fill.len_utf8()] as char) {
        format.align = rest[pos + fake_fill.len_utf8()] as char;
        format.fill = fake_fill;
        fill_specified = true;
        align_specified = true;
        pos += 1 + fake_fill.len_utf8();
    } else if end-pos >= 1 && is_alignment_token(fake_fill) {
        format.align = fake_fill;
        pos += fake_fill.len_utf8();
    }

    /* Parse the various sign options */
    if end-pos >= 1 && is_sign_element(rest[pos] as char) {
        format.sign = rest[pos] as char;
        pos+=1;
    }

    /* If the next character is #, we're in alternate mode.  This only
        applies to integers. */
    if end-pos >= 1 && rest[pos] as char == '#' {
        format.alternate = true;
        pos+=1;
    }

    /* The special case for 0-padding (backwards compat) */
    if !fill_specified && end-pos >= 1 && rest[pos] == '0' as u8 {
        format.fill = '0';
        if !align_specified {
            format.align = '=';
        }
        pos+=1;
    }

    // check to make sure that val is good
    let (consumed, val) = get_integer(rest, pos);
    pos += consumed;
    if consumed != 0 {
        match val {
            None => return Err(FmtError::Invalid("overflow error when parsing width".to_string())),
            Some(v) => {
                format.width = v;
            }
        }
    }

    /* Comma signifies add thousands separators */
    if end-pos > 0 && rest[pos] as char == ',' {
        format.thousands = true;
        pos+=1;
    }

    /* Parse field precision */
    if end-pos > 0 && rest[pos] as char == '.' {
        pos+=1;

        let (consumed, val) = get_integer(rest, pos);
        if consumed != 0 {
            match val {
                None => return Err(FmtError::Invalid("overflow error when parsing precision"
                                            .to_string())),
                Some(v) => {
                    format.precision = v;
                }
            }
        } else {
            /* Not having a precision after a dot is an error. */
            if consumed == 0 {
                return Err(FmtError::Invalid("Format specifier missing precision".to_string()));
            }
        }
        pos += consumed;

    }

    /* Finally, parse the type field. */
    if end-pos > 1 {
        /* More than one char remain, invalid format specifier. */
        return Err(FmtError::Invalid("Invalid format specifier".to_string()));
    }

    if end-pos == 1 {
        format.ty = rest[pos] as char;
        if !is_type_element(format.ty) {
            let mut msg = String::new();
            write!(msg, "Invalid type specifier: {:?}", format.ty).unwrap();
            return Err(FmtError::Invalid(msg));
        }
        // pos+=1;
    }

    /* Do as much validating as we can, just by looking at the format
        specifier.  Do not take into account what type of formatting
        we're doing (int, float, string). */
    if format.thousands {
        match format.ty {
            'd' | 'e' | 'f' | 'g' | 'E' | 'G' |
            '%' | 'F' | '\0' => {}, /* These are allowed. See PEP 378.*/

            _ => {
                let mut msg = String::new();
                write!(msg, "Invalid comma type: {}", format.ty).unwrap();
                return Err(FmtError::Invalid(msg));
            }
        }
    }
    Ok(format)
}

impl<'a> FmtChunk<'a> {
    /// create FmtChunk from format string
    pub fn from_str(s: &'a str) -> Result<FmtChunk> {
        let mut found_colon = false;
        let mut chars = s.chars();
        let mut c = match chars.next() {
            Some(':') | None => return Err(
                FmtError::Invalid("must specify identifier".to_string())),
            Some(c) => c,
        };
        let mut consumed = 0;
        // find the identifier
        loop {
            consumed += c.len_utf8();
            if c == ':' {
                found_colon = true;
                break;
            }
            c = match chars.next() {
                Some(c) => c,
                None => {
                    break;
                }
            };
        }
        let (identifier, rest) = s.split_at(consumed);
        println!("iden: {:?} rest: {:?}", identifier, rest);
        let identifier = if found_colon {
            let (i, _) = identifier.split_at(identifier.len() - 1); // get rid of ':'
            i
        } else {
            identifier
        };

        let format = try!(parse_like_python(rest));

        Ok(FmtChunk{
            identifier: identifier,
            fill: format.fill,
            align: match format.align {
                '<' => Align::Left,
                '^' => Align::Center,
                '>' => Align::Right,
                '=' => Align::Equal,
                _ => unreachable!(),
            },
            alternate: format.alternate,
            width: match format.width {
                -1 => None,
                _ => Some(format.width as usize),
            },
            thousands: format.thousands,
            precision: match format.precision {
                -1 => None,
                _ => Some(format.precision as usize),
            },
            ty: match format.ty {
                ' ' => None,
                _ => Some(format.ty),
            },
        })
    }

    /// write the formatted string to `s` and return true. If there is an error: clear `s`,
    /// write the error and return false
    pub fn write(&self, s: &mut String, vars: &'a HashMap<String, String>) -> Result<()> {
        let ref value = match vars.get(self.identifier) {
            Some(v) => v,
            None => {
                return Err(FmtError::KeyError(self.identifier.to_string()));
            }
        };
        let len = match self.precision {
            None => value.len(),
            Some(p) => {
                if p < value.len() {
                    p
                } else {
                    value.len()
                }
            }
        };
        let mut value = value.chars();
        let mut pad: usize = 0;

        match self.width {
            Some(mut width) => {
                if width > len {
                    match self.align {
                        Align::Left => pad = width - len,
                        Align::Center => {
                            width = width - len;
                            pad = width / 2;
                            write_char(s, self.fill, pad);
                            pad += width % 2;
                        }
                        Align::Right => {
                            write_char(s, self.fill, width - len);
                        }
                        Align::Equal => panic!("not yet supported"), // TODO
                    }
                }
            }
            None => {}
        }
        if self.precision.is_none() {
            s.extend(value);
        } else {
            write_from(s, &mut value, self.precision.unwrap());
        }
        write_char(s, self.fill, pad);
        Ok(())
    }
}


/// UNSTABLE: rust-style format a string given a HashMap of the variables and additional options
/// variables:
///   ignore_missing: if true, ignore missing variables
pub fn strfmt_options(fmtstr: &str, vars: &HashMap<String, String>, ignore_missing: bool) -> Result<String> {
    let mut out = String::with_capacity(fmtstr.len() * 2);
    let mut bytes_read: usize = 0;
    let mut opening_brace: usize = 0;
    let mut closing_brace: bool = false;
    let mut reading_fmt = false;
    let mut remaining = fmtstr;
    for c in fmtstr.chars() {
        bytes_read += c.len_utf8();
        if c == '{' {
            if reading_fmt && opening_brace == bytes_read - 2 {
                // found {{
                out.push(c);
                reading_fmt = false;
            } else if !reading_fmt {
                // found a first {
                reading_fmt = true;
                opening_brace = bytes_read - 1;
            } else {
                // found a { after finding an opening brace, error!
                out.clear();
                out.write_str("extra { found").unwrap();
                return Err(FmtError::Invalid(out));
            }
        } else if c == '}' {
            if !reading_fmt && !closing_brace {
                // found a '}' that isn't after a '{'
                closing_brace = true;
            } else if closing_brace {
                // found "}}"
                out.push(c);
                closing_brace = false;
            } else {
                // found a format string
                // discard before opening brace
                let (_, r) = remaining.split_at(opening_brace);

                // get the fmt pattern and remaining
                let (fmt_pattern, r) = r.split_at(bytes_read - opening_brace);
                remaining = r;

                // discard the braces
                let (_, fmt_pattern) = fmt_pattern.split_at(1);
                let (fmt_pattern, _) = fmt_pattern.split_at(fmt_pattern.len() - 1);
                // use the FmtChunk object to write the formatted string
                let fmt = try!(FmtChunk::from_str(fmt_pattern));
                match fmt.write(&mut out, vars) {
                    Ok(_) => {},
                    Err(err) => match ignore_missing {
                        true => write!(out, "{{{}}}", fmt_pattern).unwrap(),
                        false => return Err(err),
                    }
                }
                reading_fmt = false;
                bytes_read = 0;
            }
        } else if closing_brace {
            return Err(FmtError::Invalid("Single '}' encountered in format string".to_string()));
        } else if !reading_fmt {
            out.push(c)
        } // else we are currently reading a format string, so don't push
    }
    if closing_brace {
        return Err(FmtError::Invalid("Single '}' encountered in format string".to_string()));
    } else if reading_fmt {
        return Err(FmtError::Invalid("Expected '}' before end of string".to_string()));
    }
    out.shrink_to_fit();
    Ok(out)
}
