use std::collections::HashMap;
use ::{FmtError, Format};

#[test]
fn test_fmt_float64() -> Result<(),FmtError> {
    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("Zero".to_string(), 0.0);
    vars.insert("Three".to_string(), 10.0 / 3.0);
    vars.insert("Two".to_string(), 2.0);

    assert_eq!("0".to_string(),"{Zero}".format(&vars)?);
    assert_eq!("0.00".to_string(),"{Zero:.2}".format(&vars)?);
    assert_eq!("3.33333".to_string(),"{Three:.5}".format(&vars)?);
    assert_eq!("2".to_string(),"{Two}".format(&vars)?);
    assert_eq!("0.00 ".to_string(),"{Zero:<5.2}".format(&vars)?);

    Ok(())
}

#[test]
fn test_fmt_float32() -> Result<(),FmtError> {
    let mut vars: HashMap<String, f32> = HashMap::new();
    vars.insert("Zero".to_string(), 0.0);
    vars.insert("Three".to_string(), 10.0 / 3.0);
    vars.insert("Two".to_string(), 2.0);

    assert_eq!("0".to_string(),"{Zero}".format(&vars)?);
    assert_eq!("0.00".to_string(),"{Zero:.2}".format(&vars)?);
    assert_eq!("3.33333".to_string(),"{Three:.5}".format(&vars)?);
    assert_eq!("2".to_string(),"{Two}".format(&vars)?);
    assert_eq!("0.00 ".to_string(),"{Zero:<5.2}".format(&vars)?);

    Ok(())
}