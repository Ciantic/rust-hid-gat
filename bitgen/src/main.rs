use quote::quote;
use std::fs::*;
use syn;

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
        use crate::messages::*;
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
    generate_files(
        "./src/messages.rs".to_string(),
        "./src/messages_impl.rs".to_string(),
    );
}
