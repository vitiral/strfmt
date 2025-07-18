/// Format a given string with the passed variables.
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
/// let fmt = "{first}{second:7.2}";
/// // ... do stuff and adjust fmt as you need
/// let first = "test";
/// //test  77.65
/// println!("{}",strfmt!(fmt, first,second => 77.6543210).unwrap());
/// ```
#[macro_export]
macro_rules! strfmt {
    ($inst:expr, $($key:ident => $value:tt),*,) => {
        $crate::strfmt!($inst, $($key => $value)*)
    };
    ($inst:expr, $($values:tt),*,) => {
        $crate::strfmt!($inst, $($values)*)
    };
    ($inst:expr,$($values:tt)*) =>({
        let mut vars: std::collections::HashMap<String, Box<dyn $crate::DisplayStr>> = std::collections::HashMap::new();
        $crate::strfmt_builder!(vars,$($values)*);
        $crate::strfmt($inst,&vars)
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
        $crate::strfmt_builder!($vars,$($values)*)
    };
    ($vars:expr,$name:ident => $value:expr,$($values:tt)*) => {
        $vars.insert(stringify!($name).to_string(),Box::new($value));
        $crate::strfmt_builder!($vars,$($values)*)
    };
}
