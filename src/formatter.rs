use std::str;
use std::fmt;
use std::fmt::Write;
use std::iter::Iterator;
use std::string::String;

use types::*;

#[derive(Debug, PartialEq)]
pub struct Formatter<'a, 'b> {
    pub key: &'a str,
    fill: char,
    align: Alignment,  // default Right
    sign: Sign,
    alternate: bool,
    width: Option<usize>,
    thousands: bool,
    precision: Option<usize>,
    ty: Option<char>,
    buff: &'b mut String,
    pattern: &'a str,
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
        ty: '\0',
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

impl<'a, 'b> Formatter<'a, 'b> {
    /// create Formatter from format string
    pub fn from_str(s: &'a str, buff: &'b mut String) -> Result<Formatter<'a, 'b>> {
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
        let identifier = if found_colon {
            let (i, _) = identifier.split_at(identifier.len() - 1); // get rid of ':'
            i
        } else {
            identifier
        };

        let format = try!(parse_like_python(rest));

        Ok(Formatter{
            key: identifier,
            fill: format.fill,
            align: match format.align {
                '<' => Alignment::Left,
                '^' => Alignment::Center,
                '>' => Alignment::Right,
                '=' => Alignment::Equal,
                _ => unreachable!(),
            },
            sign: match format.sign {
                '\0' => Sign::Unspecified,
                '+' => Sign::Plus,
                '-' => Sign::Minus,
                ' ' => Sign::Space,
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
                '\0' => None,
                _ => Some(format.ty),
            },
            buff: buff,
            pattern: s,
        })
    }

    /// call this to re-write the original format string verbatum
    /// back to the output
    pub fn skip(mut self) -> Result<()> {
        self.buff.push('{');
        self.write_str(self.pattern).unwrap();
        self.buff.push('}');
        Ok(())
    }


    /// fill getter
    pub fn fill(&self) -> char {
        self.fill
    }

    /// align getter
    pub fn align(&self) -> Alignment {
        self.align.clone()
    }

    /// width getter
    pub fn width(&self) -> Option<usize> {
        self.width
    }

    /// thousands getter
    pub fn thousands(&self) -> bool {
        self.thousands
    }

    /// precision getter
    pub fn precision(&self) -> Option<usize> {
        self.precision
    }

    /// set precision to None, used for formatting int, float, etc
    pub fn clear_precision(&mut self) {
        self.precision = None;
    }

    /// sign getter
    pub fn sign(&self) -> Sign {
        self.sign.clone()
    }

    /// sign plus getter
    /// here because it is in fmt::Formatter
    pub fn sign_plus(&self) -> bool {
        if self.sign == Sign::Plus {
            true
        } else {
            false
        }
    }

    /// sign minus getter
    /// here because it is in fmt::Formatter
    pub fn sign_minus(&self) -> bool {
        if self.sign == Sign::Plus {
            true
        } else {
            false
        }
    }

    /// alternate getter
    pub fn alternate(&self) -> bool {
        self.alternate
    }

    // sign_aware_zero_pad // Not supported

    /// type getter
    pub fn ty(&self) -> Option<char> {
        self.ty
    }
}


impl <'a, 'b>fmt::Write for Formatter<'a, 'b> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buff.write_str(s)
    }
}
