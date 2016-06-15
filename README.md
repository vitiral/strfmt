# strfmt: rust library for formatting dynamic strings

> Note: this library is fairly stable and tested, but new features are in the early stages of development and feedback (positive or negative)
> would be much appreciated. If you use this library and liked it or decided not to use it, 
> please ping me at [@vitiral](https://twitter.com/vitiral) on twitter or vitiral@gmail.com via email to tell me about your
> experience. I would particularily like to see the code where it is being used. Thankyou!

This library is for rust developers who want to bring rust-like
formatting to non-static strings. 

Basic use:
```
extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

#[test]
fn it_works() {
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), "bob".to_string());
    vars.insert("job".to_string(), "python developer".to_string());

    let fmt = "hi, my name is {name} and I am a {job}!".to_string();
    assert_eq!(strfmt(&fmt, &vars).unwrap(), "hi, my name is bob and I am a python developer!")
}
```

In addition to the `strfmt` function, this library has the `Format` trait which adds the
`format` method to `str` and `String` types.

```
assert_eq!("hi, my name is {name}".format(&vars), "hi, my name is bob")
```

You can use this library any time you have dynamic strings you want to format, such as
if you are providing your users a ui or are reading configuration files.

strfmt does not support empty identifiers (i.e. `{}` or `{:<10}`. Integer identifiers
will be read as str keys to the hashmap (i.e. `{1:<10}` will have key == "1")

## Status and Goals

**strfmt** aims to support all of the formatting options defined in
[`std::fmt`](https://doc.rust-lang.org/std/fmt/). Currently it only supports the
format options for strings, but it has been built in such a way that it can support
any type (see HELP section below)

See the [syntax](https://doc.rust-lang.org/std/fmt/#syntax) for how to create a formatted string

### Current Status (in order of priority)
[x]: format any Display type
[x]: stabalize `strfmt_map` and add `format_map` to the `Format` trait.
[x]: add `f64` method to `Formatter` allowing those using `strfmt_map` to format
  f64s according to the spec
[ ]: add `format_f64(&self, HashMap<String, f64>` method to `Format` allowing users
  to easily format a hashmap of i64 values
[x]: add `i64` method to `Formatter` allowing those using `strfmt_map` to format
  i64s according to the spec
[ ]: add `format_i64(&self, HashMap<String, i64>` method to `Format` allowing users
  to easily format a hashmap of i64 values
[ ]: look for a rust library has "unbounded float" (like python) and add that to the formatter
[ ]: look for a rust library has "unbounded integer" (like python) and add that to the formatter
[ ]: Implement `vec` method to `Formatter` allowing those usin `strfmt_map` to format
  types of `Vec<Display>` in a way that uses precision and width
  (precision will limit the number of elements displayed, width the width of each element)
[ ]: special suppport to format HashMap<String, String> for improved speed
[ ]: special suppport to format HashMap<String, &str> for improved speed
[ ]: special suppport to format HashMap<&str, &str> for improved speed
    
    
### HELP
Adding functionality should be fairly easy, the main piece of work is checking and handling
the flags correctly and creating comprehensive tests. Hopefully I will be creating the `f64`
method soon to show how it can be done, but I could really use all the help I can get on
making this libray complete.
