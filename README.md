# strfmt: rust library for formatting dynamic strings

> Note: this library is fairly stable and tested, but new features are in the early stages of development and feedback (positive or negative)
> would be much appreciated. If you use this library and liked it or decided not to use it, 
> please ping me at [@vitiral](https://twitter.com/vitiral) on twitter or vitiral@gmail.com via email to tell me about your
> experience. I would particularily like to see the code where it is being used. Thankyou!

This library is for rust developers who want to bring rust-like
formatting to non-static strings. 

## Basic use of formatting Display types
``` rust
extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

#[test]
fn it_works() {
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), "bob");
    vars.insert("job".to_string(), "python developer");

    let fmt = "hi, my name is {name} and I am a {job}!".to_string();
    assert_eq!(strfmt(&fmt, &vars).unwrap(), "hi, my name is bob and I am a python developer!")
}
```

In addition to the `strfmt` function, this library has the `Format` trait which adds the
`format` method to `str` and `String` types.

``` rust
assert_eq!("hi, my name is {name}".format(&vars), "hi, my name is bob")
```

You can use this library any time you have dynamic strings you want to format, such as
if you are providing your users a ui or are reading configuration files.

strfmt does not support empty identifiers (i.e. `{}` or `{:<10}`. Integer identifiers
will be read as str keys to the hashmap (i.e. `{1:<10}` will have key == "1")

## Legacy
In the 0.2.0 update, the signature of `strfmt` and `Format::format` changed to fix a bug with numeric formatting.
For easy migration the `strfmt_display` and `Format::format_dispaly` function provide the old behaviour.

## **BETA**: Formatting numeric types
> This feature is in Beta and may change. I expect it to be fairly stable
> at this point but would appreciate feedback on development.
>
> In addition, "signed 0 padding" (i.e. +000042) is not yet supported
> for numeric types

Using `strfmt_map` it is also possible to format integers and floats:
``` rust
let mut vars: HashMap<String, f64> = HashMap::new();
vars.insert("x".to_string(), 42.4242);
vars.insert("y".to_string(), -100.11111);
vars.insert("z".to_string(), 0.);

let f = |mut fmt: Formatter| {
    fmt.f64(*vars.get(fmt.key).unwrap())
};
assert_eq!(strfmt_map("{x:<7.2}", f).unwrap(), "42.42  ");
assert_eq!(strfmt_map("{y:+.2E}", f).unwrap(), "-1.00E2");
assert_eq!(strfmt_map("{z:+.2E}", f).unwrap(), "+0.00E0");
```

# Status and Goals

**strfmt** aims to support all of the formatting options defined in
[`std::fmt`](https://doc.rust-lang.org/std/fmt/). Currently it officially only supports 
the format options for strings (beta support for i64 and f64)

See the [syntax](https://doc.rust-lang.org/std/fmt/#syntax) for how to create a formatted string

### Current Status (in order of priority)
- [ ] get strfmt_map out of Beta and create Format.format_map method
- [ ] handle sign aware zero padding for numeric types
- [x] format any Display type
- [x] stabilize `strfmt_map` and add `format_map` to the `Format` trait.
- [x] add `f64` method to `Formatter` allowing those using `strfmt_map` to format f64s according to the spec
- [x] add `i64` method to `Formatter` allowing those using `strfmt_map` to format i64s according to the spec
- [x] use DisplayStr trait for formatting, permitting proper formatting of integer types.
- [ ] look for a rust library has "unbounded float" (like python) and add that to the formatter
- [ ] look for a rust library has "unbounded integer" (like python) and add that to the formatter
- [ ] Implement `vec` method to `Formatter` allowing those usin `strfmt_map` to format types of `Vec<Display>` in a way that uses precision and width (precision will limit the number of elements displayed, width the width of each element)
- [ ] special suppport to format HashMap<String, String> for improved speed
- [ ] special suppport to format HashMap<String, &str> for improved speed
- [ ] special suppport to format HashMap<&str, &str> for improved speed
    
    
### HELP
Adding functionality should be fairly easy, the main piece of work is checking and handling
the flags correctly and creating comprehensive tests. Hopefully I will be creating the `f64`
method soon to show how it can be done, but I could really use all the help I can get on
making this libray complete.
