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
/// Full example: `bytes.pack_length::<u16>().set_bits(4,6).pack::<MyType>()?`
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
/// Full example: `bytes.unpack_length::<u16>().set_bits(4,6).unpack::<MyType>()?`
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
        } => build_packer(attrs, variant_name),
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
                    for variant in &ienum.variants {
                        let variant_name = &variant.ident;
                        let genitem = GenItem::Enum(ienum.clone(), variant.clone());
                        let constr = construct(&Constructor {
                            item: genitem.clone(),
                            constructer: construct_callback,
                        });
                        let destr = destruct(&Destructurer {
                            item: genitem.clone(),
                            destructrurer: destruct_callback,
                        });
                        destructed.push(quote! {
                            #destr
                        });
                        constructed.push(quote! {
                            if bytes.next_if_eq(&[0x01,0x01,0x01]) {
                                return #constr;
                            }
                        });
                    }

                    (
                        enum_name,
                        quote! {
                            #(#destructed)*
                        },
                        quote! {
                            #(#constructed);*
                            Err(
                                PacketError::Unspecified(
                                    format!("No matching variant found for {}", stringify!(H4Packet)),
                                ),
                            )
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
                field1: i32,
                field2: String,
            }

            struct AnotherStruct(u32, String);

            struct ThirdStruct;

            enum MyEnum {
                NamedVariant { field: u32, field2: String },
                UnnamedVariant(u32, String),
                UnitVariant,
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
                            field1: bytes.unpack()?,
                            field2: bytes.unpack()?,
                        }
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            MyStruct { field1, field2 } => {
                                bytes.pack(field1)?;
                                bytes.pack(field2)?;
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
                        if bytes.next_if_eq(&[0x01, 0x01, 0x01]) {
                            return MyEnum::NamedVariant {
                                field: bytes.unpack()?,
                                field2: bytes.unpack()?,
                            };
                        }
                        if bytes.next_if_eq(&[0x01, 0x01, 0x01]) {
                            return MyEnum::UnnamedVariant(bytes.unpack()?, bytes.unpack()?);
                        }
                        if bytes.next_if_eq(&[0x01, 0x01, 0x01]) {
                            return MyEnum::UnitVariant;
                        }
                        Err(
                            PacketError::Unspecified(
                                format!("No matching variant found for {}", stringify!(H4Packet)),
                            ),
                        )
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            MyEnum::NamedVariant { field, field2 } => {
                                bytes.pack(field)?;
                                bytes.pack(field2)?;
                            },
                            MyEnum::UnnamedVariant(m0, m1) => {
                                bytes.pack(m0)?;
                                bytes.pack(m1)?;
                            },
                            MyEnum::UnitVariant => {
                                bytes.pack(UnitVariant)?;
                            }
                        };
                        Ok(())
                    }
                }
            })
        );
    }
}
