//! Composable structs - reuse struct fields across multiple structs in code by storing the source of structs across macros.
//! 
//! # Usage
//! 1. Add this macro to your project
//! 
//! 2. Add a `#[reusable(name)]` attribute to the structs you want to able to copy, where the name is a globally unique identifier for the struct (best to namespace using your crate and module names)
//! 
//! 3. Add a `#[reuse(name)]` attribute to the structs you want to reuse using the same name used in the attribute above
//! 
//! # Example
//! ```ignore
//! use reusable::{reusable, reuse};
//! 
//! #[reusable(test_name)]
//! #[derive(Debug)]
//! struct Name {
//!     firstname: String,
//!     surname: String,
//! }
//! 
//! #[reuse(test_name)]
//! #[derive(Debug)]
//! struct Fullname {
//!     middlename: String,
//! }
//! 
//! fn main() {
//!     let example = Fullname {
//!         firstname: "Bob".to_string(),
//!         middlename: "Frank".to_string(),
//!         surname: "Junior".to_string(),
//!     };
//!     dbg!(example);
//! }
//! ```
//! 
//! # How it works
//! 0. This crate relies heavily on [macro_state](https://crates.io/crates/macro_state) to share data between macro calls.
//! 
//! 1. The `reusable` attribute copies the tokenstream of a struct to a global state using the provided name as a key
//! 
//! 2. The `reuse` attribute reads the tokenstream set by `reusable`, parses the structs matching the names given and then appends the fields to the generated struct (any fields with the same name are skipped so can be overriden)
//! 
//! Note: multiple names can be provided to the `reuse` attribute, e.g. `#[reuse(name1, name2)]`.
//! 
//! Works in stable Rust, no nightly required.

use macro_state::{proc_read_state, proc_write_state};
use proc_macro::TokenStream;
use quote::ToTokens;
use std::{collections::HashSet, str::FromStr};
use syn::{parse_macro_input, punctuated::Punctuated, Fields, Ident, ItemStruct, Token};

#[proc_macro_attribute]
pub fn reusable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = parse_macro_input!(attr as Ident);
    let key = name.to_string();

    //store the item as token stream based on the
    proc_write_state(&key, &item.to_string()).expect("Could not store the obj");

    item
}

#[proc_macro_attribute]
pub fn reuse(attr: TokenStream, item: TokenStream) -> TokenStream {
    let reuse_attr = parse_macro_input!(attr with Punctuated::<Ident, Token![,]>::parse_terminated);

    //parse the item and extend it with the fields from the other structs
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let Fields::Named(ref mut fields) = item_struct.fields {
        let field_names: HashSet<&Ident> =
            fields.named.iter().flat_map(|field| &field.ident).collect();

        //fetch the extended objects we need
        let mut new_fields = vec![];

        for reuse_name in reuse_attr {
            let key = reuse_name.to_string();
            let inherit_str = proc_read_state(&key).expect("object does not exist");
            let inherit_item =
                TokenStream::from_str(&inherit_str).expect("object is not a token steram");
            let inherit_item_struct = parse_macro_input!(inherit_item as ItemStruct);

            let mut new_inherit_fields = inherit_item_struct
                .fields
                .into_iter()
                .filter(|field| !field_names.contains(field.ident.as_ref().unwrap()))
                .collect::<Vec<_>>();

            new_fields.append(&mut new_inherit_fields);
        }

        fields.named.extend(new_fields);
    }

    item_struct.to_token_stream().into()
}
