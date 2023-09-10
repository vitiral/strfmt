
# strfmt: rust library for formatting dynamic strings

This library is for rust developers who want to bring rust-like
formatting to non-static strings.

# Table Of Contents  
- [Basic use of formatting Display types ](#basic-usage)
    - [Code Snippet](#codesnippet1)
- [Using the Map trait for customizing the display](#map-trait)
    - [Note on Future Stdlib Implementation of a Map trait](#stdlib-impl)
    - [Trait Definition](#map-trait-definition)
    - [Method Prioritization](#map-trait-priority)
    - [Note on Format::format_display and strfmt_display ](#strfmt-display-note)
- [Legacy ](#legacy)
- [**BETA**: Formatting numeric types](#beta-numeric)
- [Status and Goals ](#status-and-goals)
    - [Current Status (in order of priority) ](#current-status)
- [HELP ](#help)



## Basic use of formatting Display types <a name = "basic-usage"></a>

<a name = "codesnippet1"></a>
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
    assert_eq!(
      strfmt(&fmt, &vars).unwrap(),
      "hi, my name is bob and I am a python developer!")
}
```

In addition to the `strfmt` function, this library has the `Format` trait which
adds the `format` method to `str` and `String` types.

``` rust
assert_eq!("hi, my name is {name}".format(&vars), "hi, my name is bob")
```

You can use this library any time you have dynamic strings you want to format,
such as if you are providing your users a ui or are reading configuration files.

strfmt does not support empty identifiers (i.e. `{}` or `{:<10}`. Integer
identifiers will be read as str keys to the hashmap (i.e. `{1:<10}` will have
key == "1")

## Using the Map trait for customizing the display<a name="map-trait"></a> 

Pursuant to [Github Issue #17](https://github.com/vitiral/strfmt/issues/17) this trait is implemented. 

The Map trait is automatically implemented for all compliant HashMaps<br>
It can also be optionally implemented for `serde_json::Map`, `serde_json::Value` and `hashbrown::HashMap` (automatically) by using the features `serde_json` and `hashbrown` respectively


> <a name = "stdlib-impl"></a>
Note: If the rust stdlib implements a Map trait for HashMap-like containers (equivalent to the Iterator trait for containers), this trait should probably be overriden 

> Or implement a default implementation of this trait for all generic types implementing the stdlib equivalent 

Instead of using a `HashMap`, you can also use your custom structs using the `strfmt::Map` trait

It implements two methods as described below: 

<a name = "map-trait-definition"></a>
```rust 
pub trait Map<K , V> where K: Hash + Eq + FromStr, V: DisplayStr{
    
    /// Prioritized first 
    /// Returns a reference 
    /// Use to return a reference if possible 
    fn get(&self, key : &K) -> Option<&V>; 


    ///Returned an owned item instead of a reference 
    ///Prioritized second
    ///Use this one if you cannot return a reference due to borrow checker limitations 
    #[allow(unused_variables)]
    fn get_owned(&self, key : &K) -> Option<V>{
        None //This is done because HashMap and serde_json Map can both return references  
    }
}
```

<a name = "map-trait-priority"></a>
Function `get` is prioritzed, and your custom type should return a *reference* if possible. If not possible, it should return `None` and use `get_owned` instead. Which returns an *owned* type. 

If both return `None`, an error is raised. 

> <a name = "strfmt-display-note"></a> Note: strfmt_display only works for HashMaps. It is not implemented for Map trait implementations. Similarly, Format::format_display is not implemented for Map trait objects. 

## Legacy <a name = "legacy"></a>
In the 0.2.0 update, the signature of `strfmt` and `Format::format` changed to
fix a bug with numeric formatting.  For easy migration the `strfmt_display` and
`Format::format_display` function provide the old behaviour.

## **BETA**: Formatting numeric types<a name="beta-numeric"></a>
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

# Status and Goals <a name="status-and-goals"></a>

**strfmt** aims to support all of the formatting options defined in
[`std::fmt`](https://doc.rust-lang.org/std/fmt/). Currently it officially only
supports the format options for strings (beta support for i64 and f64)

See the [syntax](https://doc.rust-lang.org/std/fmt/#syntax) for how to create a
formatted string

### Current Status (in order of priority) <a name = "current-status"></a>
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


### HELP <a name="help"></a>
I (@vitiral) am no longer an active maintainer of this library or any rust code,
but I accept pull requests that fix bugs or implement the above features. All
pull requests must be tested appropriately.

Adding functionality should be fairly easy, the main piece of work is checking
and handling the flags correctly and creating comprehensive tests. Hopefully I
will be creating the `f64` method soon to show how it can be done, but I could
really use all the help I can get on making this libray complete.

