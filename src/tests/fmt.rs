use super::super::formatter::Formatter;
use super::super::types::*;

#[test]
fn test_fmt_from_str() {
    let s = String::new();
    {
        let mut s = s.clone();
        let f = Formatter::from_str("x:<.3", &mut s).unwrap();
        // defaults
        assert_eq!(f.fill(), ' ');
        assert_eq!(f.sign(), Sign::Unspecified);
        assert_eq!(f.alternate(), false);
        assert_eq!(f.width(), None);
        assert_eq!(f.thousands(), false);
        assert_eq!(f.ty(), None);

        // specified
        assert_eq!(f.key, "x");
        assert_eq!(f.precision().unwrap(), 3);
        assert_eq!(f.align(), Alignment::Left);
    }
    assert!(Formatter::from_str("x:^.3", &mut s.clone()).is_ok());
    assert!(Formatter::from_str("xxx: <88.3", &mut s.clone()).is_ok());
    assert!(Formatter::from_str("xxx:  <88.3", &mut s.clone()).is_err());
    assert!(Formatter::from_str("xxx:a34", &mut s.clone()).is_err());
}
