///wrap to simulate external use without uses of mod.rs
mod macro_test {
    use crate::FmtError;

    #[test]
    fn test_macros() -> Result<(), FmtError> {
        let first = "test";
        let second = 2;
        // Note: some use strfmt with crate:: and some don't on purpose.
        {
            use crate::strfmt;
            assert_eq!("test", strfmt!("{first}", first)?);
            assert_eq!("test2", strfmt!("{first}{second}", first, second)?);
        }
        assert_eq!(
            "test77.65  ",
            crate::strfmt!("{first}{third:<7.2}", first,second, third => 77.6543210)?
        );
        assert_eq!(
            "test  77.65",
            crate::strfmt!("{first}{third:7.2}", first,second, third => 77.6543210)?
        );
        Ok(())
    }
}
