macro_rules! fmttype {
    ($($t:ident)*) => ($(
        impl TypeFormatting for $t {
            fn do_format(&self,f:&mut Formatter) -> Result<()> {
                f.$t(*self)
            }
        }
    )*)
}