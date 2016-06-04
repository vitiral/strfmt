use std::fmt::Write;
use std::string::String;

use types::*;
use formatter::Formatter;

fn write_char(f: &mut Formatter, c: char, n: usize) {
    for _ in 0..n {
        f.write_char(c).unwrap();
    }
}

#[test]
fn test_write_char() {
    let mut s = String::new();
    s.write_str("h ").unwrap();
    {
        let mut f = Formatter::from_str("{}", &mut s).unwrap();
        write_char(&mut f, 'f', 3);
    }
    assert!(s == "h fff");
}

fn write_from<'a, I>(fmt: &mut Formatter, f: I, n: usize) -> usize
    where I: Iterator<Item = char>
{
    // eexaust f or run out of n, return chars written
    if n == 0 {
        return 0;
    }
    let mut n_written: usize = 0;
    for c in f {
        fmt.write_char(c).unwrap();
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
    {
        let mut f = Formatter::from_str("{}", &mut s).unwrap();
        write_from(&mut f, "fff".chars(), 5);
    }
    assert!(s == "h fff");
    {
        let mut f = Formatter::from_str("{}", &mut s).unwrap();
        write_from(&mut f, "xxxx".chars(), 2);
    }
    assert!(s == "h fffxx");
    {
        let mut f = Formatter::from_str("{}", &mut s).unwrap();
        write_from(&mut f, "333".chars(), 3);
    }
    assert!(s == "h fffxx333");
    s.clear();
    {
        let mut f = Formatter::from_str("{}", &mut s).unwrap();
        write!(f, "hello").unwrap();
    }
    assert!(s == "hello");
}

/// The formatter object given to closures
impl<'a, 'b> Formatter<'a, 'b> {
    /// format the given string onto the buffer
    pub fn str(self, s: &str) -> Result<()> {
        if !(self.ty() == None || self.ty() == Some('s')) {
            let mut msg = String::new();
            write!(msg, "Unknown format code {:?} for object of type 'str'", self.ty()).unwrap();
            return Err(FmtError::TypeError(msg));
        } else if self.alternate() {
            return Err(FmtError::TypeError("Alternate form (#) not allowed in string \
                                            format specifier".to_string()));
        } else if self.thousands() {
            return Err(FmtError::TypeError("Cannot specify ',' with 's'".to_string()));
        } else if self.sign().is_unspecified() {
            return Err(FmtError::TypeError("Sign not allowed in string format specifier"
                                           .to_string()));
        }
        self.str_unchecked(s)
    }

    /// UNSTABLE+UNTESTED: do not use in your own code (yet)
    /// Do the same as `str` but do not check the format string for errors.
    /// This gives a moderate performance boost.
    /// This isn't exactly unsafe, it just ends up ignoring extranious format
    /// specifiers
    //  For example, {x:<-#10} should technically be formatting an int, but ignoring the
    /// integer specific formatting is probably not the end of the world
    /// This can also be used by the `u64` etc methods to finish their formatting while
    /// still using the str formatter for width and alignment
    pub fn str_unchecked(mut self, s: &str) -> Result<()> {
        let fill = self.fill();
        let width = self.width();
        let precision = self.precision();
        // precision will limit length
        let len = match precision {
            Some(p) => if p < s.len() {
                p
            } else {
                s.len()
            },
            None => s.len(),
        };

        let mut chars = s.chars();
        let mut pad: usize = 0;
        match width {
            Some(mut width) => {
                if width > len {
                    let align = self.align();
                    match align {
                        Alignment::Left => pad = width - len,
                        Alignment::Center => {
                            width = width - len;
                            pad = width / 2;
                            write_char(&mut self, fill, pad);
                            pad += width % 2;
                        }
                        Alignment::Right => {
                            write_char(&mut self, fill, width - len);
                        }
                        Alignment::Equal => panic!("not yet supported"), // TODO
                    }
                }
            }
            None => {}
        }
        write_from(&mut self, &mut chars, len);
        write_char(&mut self, fill, pad);
        Ok(())
    }

}


/// UNSTABLE: the Formatter object is still considered unstable
/// Do not use this function if you aren't willing to have changes
/// forced on you!
///
/// format a string given the string and a closure that uses
/// a Formatter
pub fn strfmt_map<F>(fmtstr: &str, f: &F) -> Result<String>
    where F: Fn(Formatter) -> Result<()>
{
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
                // use the closure to write the formatted string
                let fmt = try!(Formatter::from_str(fmt_pattern, &mut out));
                try!(f(fmt));
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
