
use std::fmt::Write;
use std::string::String;

use types::*;
use formatter::Formatter;


/// implement formatting of f64
impl<'a, 'b> Formatter<'a, 'b> {
    /// format the given string onto the buffer
    pub fn i64(&mut self, i: i64) -> Result<()> {
        let ty = match self.ty() {
            None => ' ',
            Some(c) => {
                match c {
                    'b' | 'o' | 'x' | 'X' => c,
                    _ => {
                        let mut msg = String::new();
                        write!(msg, "Unknown format code {:?} for object of type i64", c)
                            .unwrap();
                        return Err(FmtError::TypeError(msg));
                    }
                }
            }
        };
        if self.precision() != None {
            return Err(FmtError::TypeError("precision not allowed for integers".to_string()));
        } else if self.thousands() {
            return Err(FmtError::Invalid("thousands specifier not yet supported".to_string()));
        } else if self.fill() == '0' && self.align() == Alignment::Right {
            return Err(FmtError::Invalid("sign aware 0 padding not yet supported".to_string()));
        }
        let mut s = String::new();

        // handle the sign
        if i >= 0 && self.sign_plus() {
            self.write_str("+").unwrap();
        }
        if self.alternate() {
            match ty {
                'b' => self.write_str("0b").unwrap(),
                'o' => self.write_str("0o").unwrap(),
                'x' | 'X' => self.write_str("0x").unwrap(),
                _ => {
                    let mut msg = String::new();
                    write!(msg, "alternate ('#') cannot be used with type {:?}", ty).unwrap();
                    return Err(FmtError::Invalid(msg));
                }
            }
        }

        match ty {
            ' ' => write!(s, "{}", i).unwrap(),
            'b' => write!(s, "{:b}", i).unwrap(),
            'o' => write!(s, "{:o}", i).unwrap(),
            'x' => write!(s, "{:x}", i).unwrap(),
            'X' => write!(s, "{:X}", i).unwrap(),
            _ => unreachable!(),
        }

        self.str_unchecked(s.as_str())
    }
}
