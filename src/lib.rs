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
