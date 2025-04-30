use proc_macro2::Literal;
use proc_macro2::Span;
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
use syn::Item;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::Lit;
use syn::Type;
use syn::{Expr, Meta, MetaNameValue};

use crate::common::find_attr_by_name;
use crate::common::FieldDef;
use crate::common::GenItem;
use crate::construct::construct;
use crate::construct::Constructor;
use crate::construct::ConstructorCbArg;
use crate::destruct::destruct;
use crate::destruct::Destructurer;
use crate::destruct::DestructurerCbArg;

/// Build packer tokens
///
/// Full example: `bytes.pack_length::<u16>().set_bits(12).pack::<MyType>()?`
fn build_packer(attrs: &Vec<Attribute>, field_name: &Ident) -> TokenStream {
    let bits = find_attr_by_name(&attrs, "bits");
    let prepend_length = find_attr_by_name(&attrs, "prepend_length");
    let mut ret = quote! {
        bytes
    };

    if let Some(expr) = prepend_length {
        ret.extend(quote! {
            .pack_length::<#expr>()?
        });
    }
    if let Some(bexpr) = bits {
        ret.extend(quote! {
            .set_bits(#bexpr)
        });
    }
    ret.extend(quote! { .pack(#field_name)?; });
    ret
}

/// Build unpacker tokens
///
/// Full example: `bytes.unpack_length::<u16>().set_bits(12).unpack::<MyType>()?`
fn build_unpacker(attrs: &Vec<Attribute>, ty: Option<Type>) -> TokenStream {
    let bits = find_attr_by_name(&attrs, "bits");
    let prepend_length = find_attr_by_name(&attrs, "prepend_length");
    let mut ret = quote! {
        bytes
    };
    if let Some(expr) = prepend_length {
        ret.extend(quote! {
            .unpack_length::<#expr>()?
        });
    }
    if let Some(bexpr) = bits {
        ret.extend(quote! {
            .set_bits(#bexpr)
        });
    }

    ret.extend(quote! { .unpack });

    if let Some(ty) = ty {
        ret.extend(quote! {::<#ty>()? });
    } else {
        ret.extend(quote! {()? });
    }
    ret
}

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

fn construct_callback(arg: &ConstructorCbArg) -> TokenStream {
    let field = &arg.field;
    let type_name = &arg.type_name;
    let top_level_attrs = &arg.top_level_attrs;

    match field {
        FieldDef::Named { attrs, .. } => build_unpacker(&attrs, None),
        FieldDef::Unnamed { attrs, .. } => build_unpacker(&attrs, None),

        // I'm not sure about UnitStruct, my use-case doesn't use those yet
        FieldDef::UnitStruct => build_unpacker(&top_level_attrs, Some(type_name.clone())),
        FieldDef::UnitEnum {
            variant_name,
            discriminant,
            attrs,
            ..
        } => quote! { #type_name::#variant_name },
    }
}

fn destruct_callback(args: &DestructurerCbArg) -> TokenStream {
    // let var_name = &args.var_name;
    // let type_name = &args.type_name;
    let top_level_attrs = &args.top_level_attrs;

    match &args.field {
        FieldDef::Named { attrs, name, .. } => build_packer(attrs, name),
        FieldDef::Unnamed {
            attrs, var_match, ..
        } => build_packer(attrs, var_match),
        FieldDef::UnitStruct => {
            build_packer(&top_level_attrs, &Ident::new("self", Span::call_site()))
        }
        FieldDef::UnitEnum {
            variant_name,
            attrs,
            ..
        } => quote! {},
    }
}

pub fn implementer(items: &Vec<Item>) -> Vec<proc_macro2::TokenStream> {
    let mut impls = items
        .iter()
        .filter_map(|item| {
            let (name, destructed, constructed) = match item {
                Item::Struct(istruct) => {
                    let genitem = GenItem::Struct(istruct.clone());
                    let struct_name = &istruct.ident;
                    let destructed = destruct(&Destructurer {
                        item: genitem.clone(),
                        prepend: quote! {},
                        append: quote! {},
                        destructrurer: destruct_callback,
                    });
                    let constructed = construct(&Constructor {
                        item: genitem.clone(),
                        constructer: construct_callback,
                    });
                    (struct_name, destructed, constructed)
                }
                Item::Enum(ienum) => {
                    let enum_name = &ienum.ident;
                    let mut destructed = Vec::new();
                    let mut constructed = Vec::new();
                    let last_variant = ienum.variants.last().unwrap();
                    let last_variant_id = get_id_bytes(&last_variant);
                    for variant in &ienum.variants {
                        
                        // Match variant id bytes (&[0x01, 0x02] or _ which is the passthrough)
                        let id_bytes = get_id_bytes(variant);
                        let destruct_id_bytes = match &id_bytes {
                            IdBytes::Bytes(id_bytes) => Some(quote! {
                                bytes.pack_bytes(#id_bytes)?;
                            }),
                            IdBytes::Passthrough => {
                                None
                            }
                        };

                        let genitem = GenItem::Enum(ienum.clone(), variant.clone());
                        let constr = construct(&Constructor {
                            item: genitem.clone(),
                            constructer: construct_callback,
                        });
                        let destr = destruct(&Destructurer {
                            item: genitem.clone(),
                            prepend: quote! { #destruct_id_bytes },
                            append: quote! {},
                            destructrurer: destruct_callback,
                        });
                        destructed.push(quote! {
                            #destr
                        });
                        constructed.push(match id_bytes {
                            IdBytes::Bytes(token_stream) =>    quote! {
                                if bytes.next_if_eq(#token_stream) {
                                    return Ok(#constr);
                                }
                            },
                            IdBytes::Passthrough => quote! {
                                Ok(#constr)
                            },
                        });
                    }

                    let err_or_nothing = match last_variant_id {
                        IdBytes::Bytes(token_stream) => quote! {
                            Err(PacketError::Unspecified(format!("No matching variant found for {}", stringify!(#enum_name))))
                        },
                        IdBytes::Passthrough => quote! {
                            
                        },
                    };

                    (
                        enum_name,
                        quote! {
                            #(#destructed)*
                        },
                        quote! {
                            #(#constructed);*
                            #err_or_nothing
                        },
                    )
                }
                _ => return None,
            };

            Some(quote! {
                impl FromToPacket for #name {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        #constructed
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            #destructed
                        };
                        Ok(())
                    }
                }
            })
        })
        .collect();

    impls
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn pretty_string(toks: TokenStream) -> String {
        match syn::parse2(toks.clone()) {
            Ok(f) => prettyplease::unparse(&f),
            Err(e) => {
                println!("Error: {}", e);
                println!("Unformatted {}", &toks.to_string());
                panic!("Failed to parse tokens");
            }
        }
    }

    #[test]
    fn test_implementer() {
        let input_file_contents = quote! {

            struct MyStruct {
                /// bits = 12
                field1: u16,
                /// bits = 4
                field2: u8,
                /// prepend_length = u16
                field3: String,
            }

            struct AnotherStruct(u32, String);

            struct ThirdStruct;

            enum MyEnum {
                /// id = &[0x01, 0x02]
                NamedVariant { field: u32, field2: String },
                /// id = &[0x03]
                UnnamedVariant(u32, String),
                /// id = &[0x04]
                UnitVariant,
            }

            enum Status {
                /// id = &[0x01]
                Success,
                /// id = _
                Error(u8)
            }
        };
        let res = syn::parse2::<syn::File>(input_file_contents).unwrap();
        let output_toks = implementer(&res.items);
        let output = quote! {
            #(#output_toks)*
        };
        assert_eq!(
            pretty_string(output),
            pretty_string(quote! {
                impl FromToPacket for MyStruct {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        MyStruct {
                            field1: bytes.set_bits(12).unpack()?,
                            field2: bytes.set_bits(4).unpack()?,
                            field3: bytes.unpack_length::<u16>()?.unpack()?,
                        }
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            MyStruct { field1, field2, field3 } => {
                                bytes.set_bits(12).pack(field1)?;
                                bytes.set_bits(4).pack(field2)?;
                                bytes.pack_length::<u16>()?.pack(field3)?;
                            }
                        };
                        Ok(())
                    }
                }
                impl FromToPacket for AnotherStruct {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        AnotherStruct(bytes.unpack()?, bytes.unpack()?)
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            AnotherStruct(m0, m1) => {
                                bytes.pack(m0)?;
                                bytes.pack(m1)?;
                            }
                        };
                        Ok(())
                    }
                }
                impl FromToPacket for ThirdStruct {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {

                        // Not sure about this?
                        bytes.unpack::<ThirdStruct>()?
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            ThirdStruct => {
                                bytes.pack(self)?;
                            }
                        };
                        Ok(())
                    }
                }
                impl FromToPacket for MyEnum {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        if bytes.next_if_eq(&[0x01, 0x02]) {
                            return Ok(MyEnum::NamedVariant {
                                field: bytes.unpack()?,
                                field2: bytes.unpack()?,
                            });
                        }
                        if bytes.next_if_eq(&[0x03]) {
                            return Ok(MyEnum::UnnamedVariant(bytes.unpack()?, bytes.unpack()?));
                        }
                        if bytes.next_if_eq(&[0x04]) {
                            return Ok(MyEnum::UnitVariant);
                        }
                        Err(
                            PacketError::Unspecified(
                                format!("No matching variant found for {}", stringify!(MyEnum)),
                            ),
                        )
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            MyEnum::NamedVariant { field, field2 } => {
                                bytes.pack_bytes(&[0x01, 0x02])?;
                                bytes.pack(field)?;
                                bytes.pack(field2)?;
                            },
                            MyEnum::UnnamedVariant(m0, m1) => {
                                bytes.pack_bytes(&[0x03])?;
                                bytes.pack(m0)?;
                                bytes.pack(m1)?;
                            },
                            MyEnum::UnitVariant => {
                                bytes.pack_bytes(&[0x04])?;
                            }
                        };
                        Ok(())
                    }
                }

                impl FromToPacket for Status {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        if bytes.next_if_eq(&[0x01]) {
                            return Ok(Status::Success);
                        }
                        Ok(Status::Error(bytes.unpack()?))
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            Status::Success => {
                                bytes.pack_bytes(&[0x01])?;
                            }
                            Status::Error(m0) => {
                                bytes.pack(m0)?;
                            }
                        };
                        Ok(())
                    }
                }
                    
            })
        );
    }
}
