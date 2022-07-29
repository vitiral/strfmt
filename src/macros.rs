/// format a given string with the passed variables
/// This macro is creating an single used Hashmap, for performance optimizations it might be
/// more efficient to reuse an existing one.
///
/// # Arguments
/// * `inst` - A string with an Rust-style format instructions
/// * `values` - A list of values to use for formatting
///
/// # Errors
/// see [strfmt]
///
/// # Example
/// ```
/// use strfmt::FmtError;
/// use strfmt::{strfmt,strfmt_builder};
///
/// let first = "test";
/// //test  77.65
/// println!("{}",strfmt!("{first}{second:7.2}", first,second => 77.6543210).unwrap());
/// ```
#[macro_export]
macro_rules! strfmt {
    ($inst:expr,$($values:tt)*) =>({
        use std::collections::HashMap;
        use $crate::{DisplayStr,strfmt_builder};
        let mut vars: HashMap<String, Box<dyn DisplayStr>> = HashMap::new();
        strfmt_builder!(vars,$($values)*);
        strfmt($inst,&vars)
    });
}

#[macro_export]
macro_rules! strfmt_builder {
    ($vars:expr,$value:expr) => (
        $vars.insert(stringify!($value).to_string(),Box::new($value));
    );
    ($vars:expr,$name:ident => $value:expr) => {
        $vars.insert(stringify!($name).to_string(),Box::new($value));
    };
    ($vars:expr,$value:expr,$($values:tt)*) => {
        $vars.insert(stringify!($value).to_string(),Box::new($value));
        strfmt_builder!($vars,$($values)*)
    };
    ($vars:expr,$name:ident => $value:expr,$($values:tt)*) => {
        $vars.insert(stringify!($name).to_string(),Box::new($value));
        strfmt_builder!($vars,$($values)*)
    };
}
