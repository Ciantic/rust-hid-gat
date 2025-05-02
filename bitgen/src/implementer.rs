use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn;
use syn::Attribute;
use syn::Ident;
use syn::Item;
use syn::Type;
use syn::Expr;

use crate::common::find_attr_by_name;
use crate::common::FieldDef;
use crate::common::GenItem;
use crate::construct::construct;
use crate::construct::Constructor;
use crate::construct::ConstructorCbArg;
use crate::destruct::destruct;
use crate::destruct::Destructurer;
use crate::destruct::DestructurerCbArg;

/// Packs unit struct id bytes
fn build_unitstruct_packer(attrs: &Vec<Attribute>, name: &Type) -> TokenStream {
    let mut ret = quote! {};
    if let Some(idbytes) = find_attr_by_name(&attrs, "id") {
        ret.extend(quote! {
            bytes.pack(&#idbytes)?;
        })
    } else {
        panic!("No id found for {}", name.to_token_stream().to_string());
    }
    ret
}


/// Build object (struct or enum) packer tokens
fn build_object_unpacker(attrs: &Vec<Attribute>, object_name: &Ident) -> TokenStream {
    let mut ret = quote! {};
    if let Some(idbytes) = find_attr_by_name(&attrs, "id") {
        ret.extend(quote! {
            if !bytes.next_if_eq(&#idbytes) {
                return Err(PacketError::Unspecified(format!("No matching bytes found for {}", stringify!(#object_name))));
            }
        })
    }
    if let Some(expr) = find_attr_by_name(&attrs, "prepend_length") {
        ret.extend(quote! {
            bytes.unpack_length::<#expr>()?;
        });
    }
    ret
}

/// Build pack length tokens
fn build_pack_length(attrs: &Vec<Attribute>) -> TokenStream {
    let mut ret = quote! {};
    
    if let Some(expr) = find_attr_by_name(&attrs, "prepend_length") {
        ret.extend(quote! {
            bytes
        });
        if let Some (offset) = find_attr_by_name(&attrs, "prepend_length_offset") {
            ret.extend(quote! {
                .pack_length_with_offset::<#expr>(#offset)?;
            });
        } else {
            ret.extend(quote! {
                .pack_length::<#expr>()?;
            });
        }
    }
    ret
}

/// Build unpack length tokens
fn build_unpack_length(attrs: &Vec<Attribute>) -> TokenStream {
    let mut ret = quote! {};
    
    if let Some(expr) = find_attr_by_name(&attrs, "prepend_length") {
        ret.extend(quote! {
            bytes.unpack_length::<#expr>()?;
        });
    }
    ret
}

/// Build field packer tokens
///
/// Full example: `bytes.pack_length::<u16>().set_bits(12).pack::<MyType>()?`
/// 
/// Type is inferred from the field type, so we don't need to specify it again
fn build_field_packer(attrs: &Vec<Attribute>, field_name: &Ident) -> TokenStream {

    let mut ret = quote! {
        bytes
    };

    if let Some(expr) = find_attr_by_name(&attrs, "prepend_length") {
        if let Some (offset) = find_attr_by_name(&attrs, "prepend_length_offset") {
            ret.extend(quote! {
                .pack_length_with_offset::<#expr>(#offset)?
            });
        } else {
            ret.extend(quote! {
                .pack_length::<#expr>()?
            });
        }
    }
    if let Some(bexpr) = find_attr_by_name(&attrs, "bits") {
        ret.extend(quote! {
            .set_bits(#bexpr)
        });
    }
    ret.extend(quote! { .pack(#field_name)?; });
    ret
}

/// Build field unpacker tokens
///
/// Full example: `bytes.unpack_length::<u16>().set_bits(12).unpack::<MyType>()?`
fn build_field_unpacker(attrs: &Vec<Attribute>, ty: Option<Type>) -> TokenStream {
    let mut ret = quote! {
        bytes
    };
    if let Some(expr) = find_attr_by_name(&attrs, "prepend_length") {
        ret.extend(quote! {
            .unpack_length::<#expr>()?
        });
    }
    if let Some(bexpr) = find_attr_by_name(&attrs, "bits") {
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

/// Parse ID bytes syntax
/// 
/// For example `id = &[0x01, 0x02]` results to `IdBytes::Bytes(&[0x01, 0x02])`
/// or `id = _` results to `IdBytes::Passthrough`
/// 
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
            [#val]
        });
    }
    panic!("No id found for variant {:?}", variant.to_token_stream());
}

fn unpacking_callback(arg: &ConstructorCbArg) -> TokenStream {
    let field = &arg.field;
    let type_name = &arg.type_name;

    match field {
        FieldDef::Named { attrs, .. } => build_field_unpacker(&attrs, None),
        FieldDef::Unnamed { attrs, .. } => build_field_unpacker(&attrs, None),
        FieldDef::UnitStruct { .. } => quote! { #type_name },
        FieldDef::UnitEnum { variant_name, .. } => quote! { #type_name::#variant_name },
    }
}

fn packing_callback(args: &DestructurerCbArg) -> TokenStream {
    // let var_name = &args.var_name;
    let type_name = &args.type_name;

    match &args.field {
        FieldDef::Named { attrs, name, .. } => build_field_packer(attrs, name),
        FieldDef::Unnamed { attrs, var_match, .. } => build_field_packer(attrs, var_match),
        FieldDef::UnitStruct { attrs } => build_unitstruct_packer(attrs, type_name),
        FieldDef::UnitEnum {..} => quote! {},
    }
}

pub fn implementer(items: &Vec<Item>) -> Vec<proc_macro2::TokenStream> {
    let impls = items
        .iter()
        .filter_map(|item| {
            match item {
                Item::Struct(istruct) => {
                    let genitem = GenItem::Struct(istruct.clone());
                    let struct_name = &istruct.ident;
                    let object_length_packer = build_pack_length(&istruct.attrs);
                    let object_unpacker = build_object_unpacker(&istruct.attrs, struct_name);
                    let pack_fields = destruct(&Destructurer {
                        item: genitem.clone(),
                        wrapper: |fields| quote! {
                            #(#fields)*
                        },
                        destructrurer: packing_callback,
                    });
                    let unpack_to_value = construct(&Constructor {
                        item: genitem.clone(),
                        constructer: unpacking_callback,
                    });

                    Some(quote! {
                        impl FromToPacket for #struct_name {
                            fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                                #object_unpacker
                                Ok(#unpack_to_value)
                            }
                            fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                                #object_length_packer
                                match self {
                                    #pack_fields
                                };
                                Ok(())
                            }
                        }
                    })
                }
                Item::Enum(ienum) => {
                    let enum_name = &ienum.ident;
                    let object_pack_length = build_pack_length(&ienum.attrs);
                    let object_unpacker = build_object_unpacker(&ienum.attrs, enum_name);
                    let mut packers = Vec::new();
                    let mut unpackers = Vec::new();
                    let last_variant = ienum.variants.last().unwrap();
                    let last_variant_id = get_id_bytes(&last_variant);
                    let length_after_id_unpack = find_attr_by_name(&ienum.attrs, "length_after_id").map(|f| quote! { 
                        bytes.unpack_length::<#f>()?;
                    });
                    let length_after_id_pack = find_attr_by_name(&ienum.attrs, "length_after_id").map(|f| quote! { 
                        bytes.pack_length::<#f>()?;
                    });
                    for variant in &ienum.variants {
                        
                        // Match variant id bytes (&[0x01, 0x02] or _ which is the passthrough)
                        let id_bytes = get_id_bytes(variant);
                        let pack_id_bytes = match &id_bytes {
                            IdBytes::Bytes(id_bytes) => Some(quote! {
                                bytes.pack(&#id_bytes)?;
                            }),
                            IdBytes::Passthrough => {
                                None
                            }
                        };

                        let genitem = GenItem::Enum(ienum.clone(), variant.clone());
                        let unpack = construct(&Constructor {
                            item: genitem.clone(),
                            constructer: unpacking_callback,
                        });
                        let pack = destruct(&Destructurer {
                            item: genitem.clone(),
                            wrapper: |fields| quote! {
                                #pack_id_bytes
                                #length_after_id_pack
                                #(#fields)*
                            },
                            destructrurer: packing_callback,
                        });
                        packers.push(pack);
                        unpackers.push(match id_bytes {
                            IdBytes::Bytes(token_stream) => quote! {
                                if bytes.next_if_eq(&#token_stream) {
                                    #length_after_id_unpack
                                    return Ok(#unpack);
                                }
                            },
                            IdBytes::Passthrough => quote! {
                                Ok(#unpack)
                            },
                        });
                    }

                    let err_or_nothing = match last_variant_id {
                        IdBytes::Bytes(_) => quote! {
                            Err(PacketError::Unspecified(format!("No matching variant found for {}", stringify!(#enum_name))))
                        },
                        IdBytes::Passthrough => quote! {},
                    };


                    Some(quote! {
                        impl FromToPacket for #enum_name {
                            fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                                #object_unpacker
                                #(#unpackers);*
                                #err_or_nothing
                            }
                            fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                                #object_pack_length
                                match self {
                                    #(#packers)*
                                };
                                Ok(())
                            }
                        }
                    })
                }
                _ => return None,
            }

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

            /// prepend_length = u16
            /// prepend_length_offset = 2
            struct MyStruct {
                /// bits = 12
                field1: u16,
                /// bits = 4
                field2: u8,
                /// prepend_length = u16
                field3: String,
            }

            struct AnotherStruct(u32, String);

            /// id = [0x99]
            struct ThirdStruct;

            /// length_after_id = u8
            enum MyEnum {
                /// id = [0x01, 0x02]
                NamedVariant { field: u32, field2: String },
                /// id = [0x03]
                UnnamedVariant(u32, String),
                /// id = [0x04]
                UnitVariant,
            }

            enum Status {
                /// id = [0x01]
                Success,
                /// id = _
                Error(u8)
            }

            enum SomeDiscriminantEnum {
                Foo = 0x01,
                Bar = 0x02,
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
                        bytes.unpack_length::<u16>()?;
                        Ok(MyStruct {
                            field1: bytes.set_bits(12).unpack()?,
                            field2: bytes.set_bits(4).unpack()?,
                            field3: bytes.unpack_length::<u16>()?.unpack()?,
                        })
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        bytes.pack_length_with_offset::<u16>(2)?;
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
                        Ok(AnotherStruct(bytes.unpack()?, bytes.unpack()?))
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
                        if !bytes.next_if_eq(&[0x99]) {
                            return Err(PacketError::Unspecified(format!("No matching bytes found for {}", stringify!(ThirdStruct))));
                        }
                        Ok(ThirdStruct)
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            ThirdStruct => {
                                bytes.pack(&[0x99])?;
                            }
                        };
                        Ok(())
                    }
                }
                impl FromToPacket for MyEnum {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        if bytes.next_if_eq(&[0x01, 0x02]) {
                            bytes.unpack_length::<u8>()?;
                            return Ok(MyEnum::NamedVariant {
                                field: bytes.unpack()?,
                                field2: bytes.unpack()?,
                            });
                        }
                        if bytes.next_if_eq(&[0x03]) {
                            bytes.unpack_length::<u8>()?;
                            return Ok(MyEnum::UnnamedVariant(bytes.unpack()?, bytes.unpack()?));
                        }
                        if bytes.next_if_eq(&[0x04]) {
                            bytes.unpack_length::<u8>()?;
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
                                bytes.pack(&[0x01, 0x02])?;
                                bytes.pack_length::<u8>()?;
                                bytes.pack(field)?;
                                bytes.pack(field2)?;
                            },
                            MyEnum::UnnamedVariant(m0, m1) => {
                                bytes.pack(&[0x03])?;
                                bytes.pack_length::<u8>()?;
                                bytes.pack(m0)?;
                                bytes.pack(m1)?;
                            },
                            MyEnum::UnitVariant => {
                                bytes.pack(&[0x04])?;
                                bytes.pack_length::<u8>()?;
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
                                bytes.pack(&[0x01])?;
                            }
                            Status::Error(m0) => {
                                bytes.pack(m0)?;
                            }
                        };
                        Ok(())
                    }
                }


                impl FromToPacket for SomeDiscriminantEnum {
                    fn from_packet(bytes: &mut Packet) -> Result<Self, PacketError> {
                        if bytes.next_if_eq(&[0x01]) {
                            return Ok(SomeDiscriminantEnum::Foo);
                        }
                        if bytes.next_if_eq(&[0x02]) {
                            return Ok(SomeDiscriminantEnum::Bar);
                        }
                        Err(
                            PacketError::Unspecified(
                                format!(
                                    "No matching variant found for {}", stringify!(SomeDiscriminantEnum)
                                ),
                            ),
                        )
                    }
                    fn to_packet(&self, bytes: &mut Packet) -> Result<(), PacketError> {
                        match self {
                            SomeDiscriminantEnum::Foo => {
                                bytes.pack(&[0x01])?;
                            }
                            SomeDiscriminantEnum::Bar => {
                                bytes.pack(&[0x02])?;
                            }
                        };
                        Ok(())
                    }
                }
                    
            })
        );
    }
}
