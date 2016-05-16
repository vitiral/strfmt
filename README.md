# strfmt: rust library for formatting dynamic strings

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
    assert!(strfmt(&fmt, &vars).unwrap() == "hi, my name is bob and I am a python developer!")
}
```

You can use this library any time you have dynamic strings you want to format, such as
if you are providing your users a ui or are reading configuration files.

## Scope

**strfmt** aims to support all of the formatting options defined in
[`std::fmt`](https://doc.rust-lang.org/std/fmt/) that are useful for
formatting strings and none of the options that are not useful for
formatting strings.

Items in the stdlib [syntax](https://doc.rust-lang.org/std/fmt/#syntax) that
strfmt will support (with comments on what isn't supported) are:
```
format_string := <text> [ format <text> ] *
format := '{' argument [ ':' format_spec ] '}'  # does not support empty `{}`
argument := identifier  # does not support integer

# format_spec does not support: sign, '#', 0, '.' precision or type
format_spec := [[fill]align][width]
fill := character
align := '<' | '^' | '>'
width := count
count := parameter | integer
```

Some notes:
- `integer` and empty `{}` are not supported because the formatter is always a
    HashMap<&str, &str> (dynamic types do not exist in rust like they do in python)
- `type` is not supported because it is always "s"
- `sign`, `#`, `0` and `.` are not supported because they are only related to numeric
    types

```
