use std::collections::HashMap;
use ::{FmtError, Format};

/// cautione here, this test just checks if the old float formatting behaviour is still present,
/// as this was changed in 0.2.0
#[test]
fn test_legacy() -> Result<(),FmtError> {
    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("Zero".to_string(), 0.0);
    vars.insert("One".to_string(), 10.0 / 3.0);
    vars.insert("Two".to_string(), 2.0);

    assert_eq!("0".to_string(),"{Zero}".format_display(&vars)?);
    assert_eq!("0".to_string(),"{Zero:.2}".format_display(&vars)?);
    assert_eq!("3.333".to_string(),"{One:.5}".format_display(&vars)?);
    assert_eq!("2".to_string(),"{Two}".format_display(&vars)?);
    assert_eq!("0    ".to_string(),"{Zero:<5.2}".format_display(&vars)?);

    Ok(())
}