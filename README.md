# strfmt: rust library for formatting dynamic strings

This library is for rust developers who want to bring rust-like
formatting to non-static strings. For example, there is no way to
do this:

```
let mut map = HashMap::new();
map.insert("name", "bob");
map.insert("job", "python developer");

// f contains "hi, my name is {name} and I am a {job}!"
let mut f = File::open("example.txt").unwrap();
let mut s = String::new();
f.read_to_string(&mut s).unwrap();

// prints "hi, my name is bob and I am a python developer!"
println!("{}", format(s, map));
```

This means that if you ever wanted to provide an interface where
your user could use rust-style string formatting you were SOL -- well
no more!

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
