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
use syn::FieldsUnnamed;
use syn::Ident;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::Lit;
use syn::Type;
use syn::{Expr, Meta, MetaNameValue};

use crate::common::build_field_defs_named;
use crate::common::build_field_defs_unnamed;
use crate::common::get_field_names;
use crate::common::FieldDef;
use crate::common::GenItem;

pub struct ConstructorCbArg {
    /// Name of the new type, e.g. `Foo` in `struct Foo` or `enum Foo`
    pub type_name: Ident,

    /// Top-level atttributes
    pub attrs: Vec<Attribute>,

    /// Definition of single field
    pub field: FieldDef,
}

pub struct Constructor {
    pub item: GenItem,
    pub constructer: fn(&ConstructorCbArg) -> TokenStream,
}
fn construct(args: &Constructor) -> TokenStream {
    let item = &args.item;
    let cb = &args.constructer;

    match item {
        GenItem::Struct(istruct) => {
            let top_level_attrs = istruct.attrs.clone();
            let struct_name = istruct.ident.clone();
            let map_cb = |field: &FieldDef| {
                cb(&ConstructorCbArg {
                    attrs: top_level_attrs.clone(),
                    type_name: struct_name.clone(),
                    field: field.clone(),
                })
            };

            match &istruct.fields {
                Fields::Named(fields) => {
                    let field_defs = build_field_defs_named(&fields);
                    let field_names = get_field_names(&field_defs);
                    let field_values = field_defs.iter().map(map_cb);

                    quote! {
                        #struct_name {
                            #(#field_names : #field_values),*
                        }
                    }
                }
                Fields::Unnamed(unnamed) => {
                    let field_defs = build_field_defs_unnamed(unnamed);
                    let field_values = field_defs.iter().map(map_cb);

                    quote! {
                        #struct_name (
                            #(#field_values),*
                        )
                    }
                }
                Fields::Unit => {
                    let field_value = cb(&ConstructorCbArg {
                        attrs: top_level_attrs,
                        type_name: struct_name.clone(),
                        field: FieldDef::UnitStruct,
                    });
                    quote! {
                        #field_value
                    }
                }
            }
        }
        GenItem::Enum(ienum, variant) => {
            let enum_name = ienum.ident.clone();
            let variant_name = variant.ident.clone();
            let map_cb = |field: &FieldDef| {
                cb(&ConstructorCbArg {
                    attrs: ienum.attrs.clone(),
                    type_name: enum_name.clone(),
                    field: field.clone(),
                })
            };
            match &variant.fields {
                Fields::Named(fields) => {
                    let field_defs = build_field_defs_named(fields);
                    let field_names = get_field_names(&field_defs);
                    let field_values = field_defs.iter().map(map_cb);
                    return quote! {
                        #enum_name::#variant_name {
                            #(#field_names : #field_values),*
                        }
                    };
                }
                Fields::Unnamed(unnamed) => {
                    let field_defs = build_field_defs_unnamed(unnamed);
                    let field_values = field_defs.iter().map(map_cb);
                    return quote! {
                        #enum_name::#variant_name (
                            #(#field_values),*
                        )
                    };
                }
                Fields::Unit => {
                    let field_value = map_cb(&FieldDef::UnitEnum {
                        attrs: ienum.attrs.clone(),
                        variant_name: variant_name.clone(),
                        discriminant: variant.discriminant.clone().map(|f| f.1),
                    });
                    return quote! {
                        #field_value
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_callback(arg: &ConstructorCbArg) -> TokenStream {
        let field = &arg.field;
        let type_name = &arg.type_name;
        match field {
            FieldDef::Named { name, ty, .. } => quote! {
                my_maker::<#ty>(#name)
            },
            FieldDef::Unnamed { index, ty, .. } => quote! {
                my_maker::<#ty>(#index)
            },
            FieldDef::UnitStruct => quote! {
                my_maker::<#type_name>()
            },
            FieldDef::UnitEnum {
                variant_name,
                discriminant,
                ..
            } => quote! {
                my_maker::<#type_name::#variant_name>(#discriminant)
            },
        }
    }

    #[test]
    fn construct_struct_named() {
        let input = quote! {
            #[derive(Debug)]
            struct MyStruct {
                field1: i32,
                field2: String,
            }
        }
        .to_string();
        let item: ItemStruct = syn::parse_str(&input).unwrap();
        let constructor = construct(&Constructor {
            item: GenItem::Struct(item),
            constructer: dummy_callback,
        });
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyStruct {
                    field1: my_maker::<i32>(field1),
                    field2: my_maker::<String>(field2)
                }
            }
            .to_string()
        );
    }

    #[test]
    fn construct_struct_unnamed() {
        let input = quote! {
            #[derive(Debug)]
            struct MyStruct(i32, String);
        }
        .to_string();
        let item: ItemStruct = syn::parse_str(&input).unwrap();
        let constructor = construct(&Constructor {
            item: GenItem::Struct(item),
            constructer: dummy_callback,
        });
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyStruct(
                    my_maker::<i32>(0),
                    my_maker::<String>(1)
                )
            }
            .to_string()
        );
    }

    #[test]
    fn construct_struct_unit() {
        let input = quote! {
            #[derive(Debug)]
            struct MyStruct;
        }
        .to_string();
        let item: ItemStruct = syn::parse_str(&input).unwrap();
        let constructor = construct(&Constructor {
            item: GenItem::Struct(item),
            constructer: dummy_callback,
        });
        assert_eq!(
            constructor.to_string(),
            quote! {
                my_maker::<MyStruct>()
            }
            .to_string()
        );
    }

    #[test]
    fn construct_enum_named() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Variant1 { field1: i32, field2: String },
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let constructor = construct(&Constructor {
            item: GenItem::Enum(item.clone(), item.variants.first().unwrap().clone()),
            constructer: dummy_callback,
        });
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyEnum::Variant1 {
                    field1: my_maker::<i32>(field1),
                    field2: my_maker::<String>(field2)
                }
            }
            .to_string()
        );
    }

    #[test]
    fn construct_enum_unnamed() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Variant2(u32, String),
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let constructor = construct(&Constructor {
            item: GenItem::Enum(item.clone(), item.variants.last().unwrap().clone()),
            constructer: dummy_callback,
        });
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyEnum::Variant2(
                    my_maker::<u32>(0),
                    my_maker::<String>(1)
                )
            }
            .to_string()
        );
    }

    #[test]
    fn construct_enum_unit() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Variant3
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let constructor = construct(&Constructor {
            item: GenItem::Enum(item.clone(), item.variants.last().unwrap().clone()),
            constructer: dummy_callback,
        });
        assert_eq!(
            constructor.to_string(),
            quote! {
                my_maker::<MyEnum::Variant3>()
            }
            .to_string()
        );
    }
    #[test]
    fn construct_enum_unit_discriminants() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Choice1 = 1,
                Choice2 = 2,
                Choice255 = 0xff
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let constructor = construct(&Constructor {
            item: GenItem::Enum(item.clone(), item.variants.last().unwrap().clone()),
            constructer: dummy_callback,
        });
        assert_eq!(
            constructor.to_string(),
            quote! {
                my_maker::<MyEnum::Choice255>(0xff)
            }
            .to_string()
        );
    }
}
