use std::fmt::Write;
use std::string::String;

use types::*;
use formatter::Formatter;

/// implement formatting of u16
impl<'a, 'b> Formatter<'a, 'b> {
    /// format the given string onto the buffer
    pub fn u16(&mut self, u: u16) -> Result<()> {
        let ty = match self.ty() {
            None => ' ',
            Some(c) => c,
        };
        if !self.is_int_type() {
            let mut msg = String::new();
            write!(msg, "Unkown format code {:?} for object type u16", ty).unwrap();
            return Err(FmtError::TypeError(msg));
        }
        if self.precision() != None {
            return Err(FmtError::TypeError("precision not allowed for integers".to_string()));
        } else if self.thousands() {
            return Err(FmtError::Invalid("thousands specifier not yet supported".to_string()));
        } else if self.fill() == '0' && self.align() == Alignment::Right {
            return Err(FmtError::Invalid("sign aware 0 padding not yet supported".to_string()));
        }

        let mut s = String::new();

        if self.sign_plus() {
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
            ' ' => write!(s, "{}", u).unwrap(),
            'b' => write!(s, "{:b}", u).unwrap(),
            'o' => write!(s, "{:o}", u).unwrap(),
            'x' => write!(s, "{:x}", u).unwrap(),
            'X' => write!(s, "{:X}", u).unwrap(),
            _ => unreachable!(),
        }

        self.str_unchecked(s.as_str())
    }
}
