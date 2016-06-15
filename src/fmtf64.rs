use std::fmt::Write;
use std::string::String;

use types::*;
use formatter::Formatter;


/// implement formatting of f64
impl<'a, 'b> Formatter<'a, 'b> {
    /// format the given string onto the buffer
    pub fn f64(&mut self, f: f64) -> Result<()> {
        let ty = match self.ty() {
            None => 'f',
            Some(c) => c,
        };
        if !self.is_float_type() {
            let mut msg = String::new();
            write!(msg, "Unknown format code {:?} for object of type f64", ty).unwrap();
            return Err(FmtError::TypeError(msg));
        }
        if self.alternate() {
            return Err(FmtError::TypeError("Alternate form (#) not allowed in f64 format \
                                            specifier"
                                               .to_string()));
        } else if self.thousands() {
            return Err(FmtError::Invalid("thousands specifier not yet supported".to_string()));
        } else if self.fill() == '0' && self.align() == Alignment::Right {
            return Err(FmtError::Invalid("sign aware 0 padding not yet supported".to_string()));
        }
        let mut s = String::new();

        // handle the sign
        if f >= 0_f64 && self.sign_plus() {
            self.write_str("+").unwrap();
        }

        match self.precision() {
            None => {
                match ty {
                    'f' => write!(s, "{}", f).unwrap(),
                    'e' => write!(s, "{:e}", f).unwrap(),
                    'E' => write!(s, "{:E}", f).unwrap(),
                    _ => unreachable!(),
                }
            }
            Some(p) => {
                match ty {
                    'f' => write!(s, "{:.*}", p, f).unwrap(),
                    'e' => write!(s, "{:.*e}", p, f).unwrap(),
                    'E' => write!(s, "{:.*E}", p, f).unwrap(),
                    _ => unreachable!(),
                }
            }
        }

        let prev_prec = self.precision();
        self.set_precision(None);
        let out = self.str_unchecked(s.as_str());
        self.set_precision(prev_prec);
        out
    }
}
