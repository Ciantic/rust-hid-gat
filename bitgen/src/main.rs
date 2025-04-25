use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::fs::*;
use syn;
use syn::Attribute;
use syn::Fields;
use syn::Ident;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::Lit;
use syn::{Expr, Meta, MetaNameValue};

fn find_attr_by_name(attrs: &Vec<Attribute>, name: &str) -> Option<Expr> {
    let mut res = None;

    attrs.iter().for_each(|attr| {
        let m: &Meta = &attr.meta;
        if let Meta::NameValue(nmv) = m {
            if nmv.path.is_ident("doc") {
                if let Expr::Lit(syn::ExprLit { lit, .. }) = &nmv.value {
                    if let syn::Lit::Str(lit) = lit {
                        if let Ok(v) = syn::parse_str::<MetaNameValue>(&lit.value()) {
                            if v.path.get_ident().is_some_and(|f| f.to_string() == name) {
                                res = Some(v.value);
                            }
                        }
                    }
                }
            }
        }
    });

    res
}

fn impl_enum(enu: &ItemEnum) -> TokenStream {
    let enum_name = enu.ident.clone();
    let variants: Vec<&syn::Variant> = enu.variants.iter().collect::<Vec<_>>();

    // let enum_attrs = enum_attrs.iter().collect::<Vec<_>>();
    let enum_name = enum_name.clone();
    // let variants_1 = variants.iter().map(|variant| variant.ident.clone());
    // let variants_2 = variants.iter().map(|variant| variant.ident.clone());
    let to_packet = variants.iter().map(|variant| {
        let name = variant.ident.clone();
        let id_bytes = find_attr_by_name(&variant.attrs, "id");
        let mut id_bytes_ = id_bytes.to_token_stream();
        if id_bytes.is_none() {
            if let Some(discr) = &variant.discriminant {
                let discr_value = discr.1.to_token_stream();
                id_bytes_ = quote! {
                    &[#discr_value]
                };
            } else {
                panic!("No id found for variant {enum_name}::{name}");
            }
        }

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().expect("Expected named fields"))
                    .collect::<Vec<_>>();
                quote! {
                    #enum_name::#name {
                        #(#field_names),*
                    } => {
                        bytes.pack_bytes(#id_bytes_)?;
                        #(
                            bytes.pack(#field_names)?;
                        )*
                        Ok(())
                    }
                }
            }
            Fields::Unnamed(tuple_fields) => {
                let matchers = tuple_fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Ident::new(&format!("m{}", i), name.span()))
                    .collect::<Vec<_>>();

                quote! {
                    #enum_name::#name(
                        #(#matchers),*
                    ) => {
                        bytes.pack_bytes(#id_bytes_)?;
                        #(
                            #matchers.to_packet(bytes)?;
                        )*
                        Ok(())

                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #enum_name::#name => bytes.pack_bytes(#id_bytes_)

                }
            }
        }
    });

    let from_packet = variants.iter().map(|variant| {
        let name = variant.ident.clone();
        let id_bytes = find_attr_by_name(&variant.attrs, "id");
        let mut id_bytes_ = id_bytes.to_token_stream();
        if id_bytes.is_none() {
            if let Some(discr) = &variant.discriminant {
                let discr_value = discr.1.to_token_stream();
                id_bytes_ = quote! {
                    &[#discr_value]
                };
            } else {
                panic!("No id found for variant {enum_name}::{name}");
            }
        }

        let make_fields = match &variant.fields {
            Fields::Unnamed(tuple_fields) => {
                let unpacks = tuple_fields
                    .unnamed
                    .iter()
                    .map(|_| {
                        quote! {
                            bytes.unpack()?
                        }
                    })
                    .collect::<Vec<_>>();

                quote! {
                    #enum_name::#name(
                        #(
                            #unpacks
                        ),*
                    )
                }
            }
            Fields::Named(fields) => {
                let field_names = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().expect("Expected named fields"))
                    .collect::<Vec<_>>();

                quote! {
                    #enum_name::#name {
                        #(
                            #field_names: bytes.unpack()?,
                        )*
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #enum_name::#name
                }
            }
        };

        quote! {
            if bytes.next_if_eq(#id_bytes_) {
                return Ok(#make_fields);
            }
        }
    });

    quote! {

        impl FromToPacket for #enum_name {
            fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                #(#from_packet)*
                Err(PacketError::Unspecified(format!("No matching variant found for {}", stringify!(#enum_name))))
            }

            fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                match self {
                    #(#to_packet),*
                }
            }
        }
    }
}

fn impl_struct(strut: &ItemStruct) -> TokenStream {
    let struct_name = strut.ident.clone();
    match &strut.fields {
        Fields::Named(f) => {
            let field_names = f
                .named
                .iter()
                .map(|field| field.ident.as_ref().expect("Expected named fields"))
                .collect::<Vec<_>>();

            let field_types = f.named.iter().map(|field| &field.ty).collect::<Vec<_>>();

            quote! {

                impl FromToPacket for #struct_name {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        let result = Self {
                            #(
                                #field_names: bytes.unpack(bytes)?,
                            )*
                        };
                        Ok(result)
                    }

                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        #(
                            self.#field_names.to_packet(bytes)?;
                        )*
                        Ok(())
                    }
                }
            }
        }
        Fields::Unnamed(f) => {
            // println!("Unnamed struct: {:?}", strut.ident);
            // panic!("Unnamed structs are not supported yet");

            let field_types = f.unnamed.iter().map(|field| &field.ty).collect::<Vec<_>>();
            let field_numbers = (0..f.unnamed.len())
                .map(|i| Lit::new(Literal::usize_unsuffixed(i)))
                .collect::<Vec<_>>();

            let unpacks = f
                .unnamed
                .iter()
                .map(|_| {
                    quote! {
                        bytes.unpack()?
                    }
                })
                .collect::<Vec<_>>();

            quote! {

                impl FromToPacket for #struct_name {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        Ok(Self(
                            #(
                                #unpacks
                            ),*
                        ))
                    }

                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        #(
                            self.#field_numbers.to_packet(bytes)?;
                        )*
                        Ok(())
                    }
                }
            }
        }
        Fields::Unit => {
            println!("Unit struct: {:?}", strut.ident);
            panic!("Unit structs are not supported yet")
        }
    }
}

fn generate_files(path: String, output: String) {
    let read_file = read_to_string(path).unwrap();
    let res = syn::parse_file(&read_file).unwrap();
    let mut impls = vec![];

    for item in res.items.iter() {
        if let syn::Item::Enum(ref e) = item {
            impls.push(impl_enum(&e));
        }

        if let syn::Item::Struct(ref s) = item {
            impls.push(impl_struct(&s));
        }
    }

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
