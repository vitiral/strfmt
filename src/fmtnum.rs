macro_rules! fmtint {
    ($($t:ident)*) => ($(
        #[allow(unused_comparisons)]
        impl<'a, 'b> Formatter<'a, 'b> {
            pub fn $t(&mut self, x: $t) -> Result<()> {
                let ty = match self.ty() {
                    None => ' ',
                    Some(c) => c,
                };

                if !self.is_int_type() {
                    let mut msg = String::new();
                    write!(msg, "Unknown format code {:?} for type", ty).unwrap();
                    return Err(FmtError::TypeError(msg));
                }

                if self.precision() != None {
                    return Err(FmtError::TypeError("precision not allowed for integers".to_string()));
                }

                if self.thousands() {
                    return Err(FmtError::Invalid("thousands specifier not yet supported".to_string()));
                }

                if self.fill() == '0' && self.align() == Alignment::Right {
                    return Err(FmtError::Invalid("sign aware 0 padding not yet supported".to_string()));
                }

                let mut s = String::new();

                if x >= 0 && self.sign_plus() {
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
                    ' ' => write!(s, "{}", x).unwrap(),
                    'b' => write!(s, "{:b}", x).unwrap(),
                    'o' => write!(s, "{:o}", x).unwrap(),
                    'x' => write!(s, "{:x}", x).unwrap(),
                    'X' => write!(s, "{:X}", x).unwrap(),
                    _ => unreachable!(),
                }

                self.str_unchecked(s.as_str())
            }
    })*)
}

macro_rules! fmtfloat {
    ($($t:ident)*) => ($(
        impl<'a, 'b> Formatter<'a, 'b> {
            pub fn $t(&mut self, x: $t) -> Result<()> {
                let ty = match self.ty() {
                    None => 'f',
                    Some(c) => c,
                };

                if !self.is_float_type() {
                    let mut msg = String::new();
                    write!(msg, "Unknown format code {:?} for type", ty).unwrap();
                    return Err(FmtError::TypeError(msg));
                }

                if self.alternate() {
                    return Err(FmtError::TypeError("Alternate form (#) not allowed for floats".to_string()));
                }

                if self.thousands() {
                    return Err(FmtError::Invalid("thousands specifier not yet supported".to_string()));
                }

                if self.fill() == '0' && self.align() == Alignment::Right {
                    return Err(FmtError::Invalid("sign aware 0 padding not yet supported".to_string()));
                }

                let mut s = String::new();

                if x >= (0 as $t) && self.sign_plus() {
                    self.write_str("+").unwrap();
                }

                match self.precision() {
                    None => {
                        match ty {
                            'f' => write!(s, "{}", x).unwrap(),
                            'e' => write!(s, "{:e}", x).unwrap(),
                            'E' => write!(s, "{:E}", x).unwrap(),
                            _ => unreachable!(),
                        }
                    }
                    Some(p) => {
                        match ty {
                            'f' => write!(s, "{:.*}", p, x).unwrap(),
                            'e' => write!(s, "{:.*e}", p, x).unwrap(),
                            'E' => write!(s, "{:.*E}", p, x).unwrap(),
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
        })*)
}
