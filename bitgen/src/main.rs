use core::panic;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use std::fs::*;
use syn;
use syn::Attribute;
use syn::Fields;
use syn::FieldsNamed;
use syn::Ident;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::Lit;
use syn::{Expr, Meta, MetaNameValue};

mod common;
mod construct;
mod destruct;
mod implementer;
mod implementer_example;

fn generate_files(path: String, output: String) {
    use implementer::implementer;
    let read_file = read_to_string(path).unwrap();
    let res = syn::parse_file(&read_file).unwrap();
    let impls = implementer(&res.items);

    let v = quote! {
        use crate::core::*;
        use crate::packer::*;
        #(#impls)*
    };

    println!("Unformatted: {}", v.to_string());
    let f = syn::parse2(v).unwrap();
    let c = prettyplease::unparse(&f);
    println!("{}", c);
    std::fs::write(output, c.clone()).unwrap();
}

fn main() {
    generate_files("./src/core.rs".to_string(), "./src/core_gen.rs".to_string());
}
