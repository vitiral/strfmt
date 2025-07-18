///wrap to simulate external use without uses of mod.rs
mod macro_test {
    use crate::FmtError;

    #[test]
    fn test_macros() -> Result<(), FmtError> {
        let first = "test";
        let second = 2;
        assert_eq!("test", crate::strfmt!("{first}", first)?);
        assert_eq!("test2", crate::strfmt!("{first}{second}", first, second)?);
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
