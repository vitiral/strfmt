///wrap to simulate external use without uses of mod.rs
mod macro_test{
    use crate::FmtError;
    use crate::{strfmt,strfmt_builder};

    #[test]
    fn test_macros() -> Result<(),FmtError>{
        let first = "test";
        let second = 2;
        assert_eq!("test",strfmt!("{first}", first)?);
        assert_eq!("test2",strfmt!("{first}{second}", first,second)?);
        assert_eq!("test77.65  ",strfmt!("{first}{third:<7.2}", first,second, third => 77.6543210)?);
        assert_eq!("test  77.65",strfmt!("{first}{third:7.2}", first,second, third => 77.6543210)?);
        Ok(())
    }
}

