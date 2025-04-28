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

mod construct;

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

fn get_field_names(fields: &FieldsNamed) -> Vec<&Ident> {
    fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().expect("Expected named fields"))
        .collect::<Vec<_>>()
}

fn get_packer(fields: &FieldsNamed) -> Vec<TokenStream> {
    fields
        .named
        .iter()
        .map(|field| {
            let bits = find_attr_by_name(&field.attrs, "bits");
            let prepend_length = find_attr_by_name(&field.attrs, "prepend_length");
            let mut ret = quote! {};

            if let Some(expr) = prepend_length {
                ret.extend(quote! {
                    pack_length::<#expr>()?.
                });
            }
            if let Some(bexpr) = bits {
                ret.extend(quote! {
                    set_bits(#bexpr).
                });
            }
            ret.extend(quote! { pack });
            ret
        })
        .collect::<Vec<_>>()
}

fn get_unpacker(fields: &FieldsNamed) -> Vec<TokenStream> {
    fields
        .named
        .iter()
        .map(|field| {
            let bits = find_attr_by_name(&field.attrs, "bits");
            let prepend_length = find_attr_by_name(&field.attrs, "prepend_length");
            let mut ret = quote! {};
            if let Some(expr) = prepend_length {
                ret.extend(quote! {
                    unpack_length::<#expr>()?.
                });
            }
            if let Some(bexpr) = bits {
                ret.extend(quote! {
                    set_bits(#bexpr).
                });
            }

            ret.extend(quote! { unpack });
            ret
        })
        .collect::<Vec<_>>()
}

// TODO: Unpacker and packer support for other than Named fields (currently only
// Named fields are supported)

enum IdBytes {
    Bytes(TokenStream),
    Passthrough,
}

fn get_id_bytes(variant: &syn::Variant) -> IdBytes {
    // Try to find from attribute first
    let id_bytes = find_attr_by_name(&variant.attrs, "id");
    if let Some(expr) = id_bytes {
        if let Expr::Infer(_) = &expr {
            return IdBytes::Passthrough;
        }
        return IdBytes::Bytes(expr.to_token_stream());
    }

    // Then try to find from discriminant e.g. `enum Bar { Foo = 0x01 }`
    if let Some((_, val)) = &variant.discriminant {
        return IdBytes::Bytes(quote! {
            &[#val]
        });
    }
    panic!("No id found for variant {:?}", variant.to_token_stream());
}

/// Add pack_length and unpack_length to the top level struct or enum
fn add_toplevel_length_prepends(
    attrs: &Vec<Attribute>,
    prepend_to_packet: &mut TokenStream,
    prepend_from_packet: &mut TokenStream,
) {
    // parse `prepend = u16` type of doc attribute, and store length position
    if let Some(expr) = find_attr_by_name(&attrs, "prepend_length") {
        prepend_to_packet.extend(quote! {
            bytes.pack_length::<#expr>()?;
        });
        prepend_from_packet.extend(quote! {
            // Intentionally ignores the unpacked length for now
            bytes.unpack_length::<#expr>()?;
        });
    }
}

fn impl_enum(enu: &ItemEnum) -> TokenStream {
    let enum_name = enu.ident.clone();
    let variants: Vec<&syn::Variant> = enu.variants.iter().collect::<Vec<_>>();

    let mut prepend_to_packet = quote! {};
    let mut prepend_from_packet = quote! {};

    add_toplevel_length_prepends(&enu.attrs, &mut prepend_to_packet, &mut prepend_from_packet);

    let enum_name = enum_name.clone();
    let last_variant = variants.last().unwrap();
    let last_variant_id = get_id_bytes(&last_variant);
    let to_packet = variants.iter().map(|variant| {
        let name = variant.ident.clone();

        // Match variant id bytes (&[0x01, 0x02] or _ which is the passthrough)
        let id_bytes_match = match get_id_bytes(variant) {
            IdBytes::Bytes(id_bytes) => Some(quote! {
                bytes.pack_bytes(#id_bytes)
            }),
            IdBytes::Passthrough => {
                if variant.ident != last_variant.ident {
                    panic!(
                        "Passthrough (_) id bytes are only allowed on the last variant '{}'",
                        last_variant.ident
                    );
                }
                None
            }
        };

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names = get_field_names(fields);
                let packer = get_packer(fields);

                let id_bytes = match id_bytes_match {
                    Some(id_bytes) => quote! {
                        #id_bytes?;
                    },
                    None => quote! {},
                };

                quote! {
                    #enum_name::#name {
                        #(#field_names),*
                    } => {
                        #id_bytes
                        #(
                            bytes.#packer(#field_names)?;
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

                let id_bytes = match id_bytes_match {
                    Some(id_bytes) => quote! {
                        #id_bytes?;
                    },
                    None => quote! {},
                };

                quote! {
                    #enum_name::#name(
                        #(#matchers),*
                    ) => {
                        #id_bytes
                        #(
                            bytes.pack(#matchers)?;
                        )*
                        Ok(())

                    }
                }
            }
            Fields::Unit => {
                let id_bytes = match id_bytes_match {
                    Some(id_bytes) => quote! {
                        {
                            #id_bytes?;
                            Ok(())
                        }
                    },
                    None => quote! {
                        Ok(())
                    },
                };

                quote! {
                    #enum_name::#name => #id_bytes

                }
            }
        }
    });

    let from_packet = variants.iter().map(|variant| {
        let name = variant.ident.clone();

        // Match variant id bytes (&[0x01, 0x02] or _ which is the passthrough)
        let id_bytes_match = get_id_bytes(variant);

        let make_fields = match &variant.fields {
            Fields::Named(fields) => {
                let field_names = get_field_names(fields);
                let unpacker = get_unpacker(fields);

                quote! {
                    #enum_name::#name {
                        #(
                            #field_names: bytes.#unpacker()?,
                        )*
                    }
                }
            }
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
            Fields::Unit => {
                quote! {
                    #enum_name::#name
                }
            }
        };

        match id_bytes_match {
            IdBytes::Bytes(id_bytes) => quote! {
                if bytes.next_if_eq(#id_bytes) {
                    return Ok(#make_fields);
                }
            },
            IdBytes::Passthrough => {
                if variant.ident != last_variant.ident {
                    panic!(
                        "Passthrough (_) id bytes are only allowed on the last variant '{}'",
                        last_variant.ident
                    );
                }
                quote! {
                    return Ok(#make_fields);
                }
            }
        }
    });

    let err = if let IdBytes::Passthrough = last_variant_id {
        quote! {}
    } else {
        quote! {
            Err(PacketError::Unspecified(format!("No matching variant found for {}", stringify!(#enum_name))))
        }
    };

    quote! {
        impl FromToPacket for #enum_name {
            fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                #prepend_from_packet
                #(#from_packet)*
                #err
            }

            fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                #prepend_to_packet
                match self {
                    #(#to_packet),*
                }
            }
        }
    }
}

fn impl_struct(strut: &ItemStruct) -> TokenStream {
    let struct_name = strut.ident.clone();

    let mut prepend_to_packet = quote! {};
    let mut prepend_from_packet = quote! {};

    add_toplevel_length_prepends(
        &strut.attrs,
        &mut prepend_to_packet,
        &mut prepend_from_packet,
    );

    match &strut.fields {
        Fields::Named(f) => {
            let field_names = get_field_names(f);
            let packer = get_packer(f);
            let unpacker = get_unpacker(f);

            quote! {

                impl FromToPacket for #struct_name {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        #prepend_from_packet
                        let result = Self {
                            #(
                                #field_names: {
                                    bytes.#unpacker()?
                                },
                            )*
                        };
                        Ok(result)
                    }

                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        #prepend_to_packet
                        #(
                            bytes.#packer(&self.#field_names)?;
                        )*
                        Ok(())
                    }
                }
            }
        }
        Fields::Unnamed(f) => {
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
                        #prepend_from_packet
                        Ok(Self(
                            #(
                                #unpacks
                            ),*
                        ))
                    }

                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        #prepend_to_packet
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
