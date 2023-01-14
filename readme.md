# Reuse - composable struct fields
An example of re-using struct fields across multiple structs in code by storing the source of structs across macros.

## Usage
1. Add this macro to your project
```toml
[dependencies]
reusable = "0.1.0"
```

2. Add a `#[reusable(name)]` attribute to the structs you want to able to copy, where the name is a globally unique identifier for the struct (best to namespace using your crate and module names)

3. Add a `#[reuse(name)]` attribute to the structs you want to reuse using the same name used in the attribute above

## Example
```rust
use reusable::{reusable, reuse};

#[reusable(test_name)]
#[derive(Debug)]
struct Name {
    firstname: String,
    surname: String,
}

#[reuse(test_name)]
#[derive(Debug)]
struct Fullname {
    middlename: String,
}

fn main() {
    let example = Fullname {
        firstname: "Bob".to_string(),
        middlename: "Frank".to_string(),
        surname: "Junior".to_string(),
    };
    dbg!(example);
}
```

## How it works
0. This crate relies heavily on [macro_state](https://crates.io/crates/macro_state) to share data between macro calls.

1. The `reusable` attribute copies the tokenstream of a struct to a global state using the provided name as a key

2. The `reuse` attribute reads the tokenstream set by `reusable`, parses the structs matching the names given and then appends the fields to the generated struct (any fields with the same name are skipped so can be overriden)

Note: multiple names can be provided to the `reuse` attribute, e.g. `#[reuse(name1, name2)]`.

Works in stable Rust, no nightly required.

## Alternatives
Other crates can provide similar functionality such as:

[born](https://github.com/steadylearner/born) - generates macros from inlined struct definitions that generate new structs
