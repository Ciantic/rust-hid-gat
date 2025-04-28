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

pub enum Constructor {
    Struct(syn::ItemStruct),
    Enum(syn::ItemEnum, syn::Variant),
}

pub enum FieldDef {
    Named {
        attrs: Vec<Attribute>,
        index: usize,
        name: Ident,
        ty: Type,
    },
    Unnamed {
        attrs: Vec<Attribute>,
        index: usize,
        ty: Type,
    },
    UnitStruct {
        attrs: Vec<Attribute>,
        struct_name: Ident,
    },
    UnitEnum {
        attrs: Vec<Attribute>,
        enum_name: Ident,
        variant_name: Ident,
    },
}

pub fn construct(val: &Constructor, cb: fn(&FieldDef) -> TokenStream) -> TokenStream {
    fn get_field_names(fields: &FieldsNamed) -> Vec<&Ident> {
        fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().expect("Expected named fields"))
            .collect::<Vec<_>>()
    }

    fn get_field_defs(fields: &FieldsNamed) -> Vec<FieldDef> {
        fields
            .named
            .iter()
            .enumerate()
            .map(|(index, field)| FieldDef::Named {
                attrs: field.attrs.clone(),
                index,
                name: field.ident.clone().expect("Expected named fields"),
                ty: field.ty.clone(),
            })
            .collect::<Vec<_>>()
    }

    fn get_field_defs_unnamed(fields: &FieldsUnnamed) -> Vec<FieldDef> {
        fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| FieldDef::Unnamed {
                attrs: field.attrs.clone(),
                index,
                ty: field.ty.clone(),
            })
            .collect::<Vec<_>>()
    }

    match val {
        Constructor::Struct(istruct) => {
            let top_level_attrs = istruct.attrs.clone();
            let struct_name = istruct.ident.clone();

            match &istruct.fields {
                Fields::Named(fields) => {
                    let field_names = get_field_names(&fields);
                    let field_defs = get_field_defs(&fields);
                    let field_values = field_defs.iter().map(cb);

                    quote! {
                        #struct_name {
                            #(#field_names : #field_values),*
                        }
                    }
                }
                Fields::Unnamed(unnamed) => {
                    let field_defs = get_field_defs_unnamed(unnamed);
                    let field_values = field_defs.iter().map(cb);

                    quote! {
                        #struct_name (
                            #(#field_values),*
                        )
                    }
                }
                Fields::Unit => {
                    let field_value = cb(&FieldDef::UnitStruct {
                        attrs: top_level_attrs,
                        struct_name: struct_name.clone(),
                    });
                    quote! {
                        #field_value
                    }
                }
            }
        }
        Constructor::Enum(ienum, variant) => {
            let enum_name = ienum.ident.clone();
            let variant_name = variant.ident.clone();
            match &variant.fields {
                Fields::Named(fields) => {
                    let field_names = get_field_names(&fields);
                    let field_defs = get_field_defs(fields);
                    let field_values = field_defs.iter().map(cb);
                    return quote! {
                        #enum_name::#variant_name {
                            #(#field_names : #field_values),*
                        }
                    };
                }
                Fields::Unnamed(unnamed) => {
                    let field_defs = get_field_defs_unnamed(unnamed);
                    let field_values = field_defs.iter().map(cb);
                    return quote! {
                        #enum_name::#variant_name (
                            #(#field_values),*
                        )
                    };
                }
                Fields::Unit => {
                    let field_value = cb(&FieldDef::UnitEnum {
                        attrs: ienum.attrs.clone(),
                        enum_name,
                        variant_name: variant_name.clone(),
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

    fn dummy_callback(field: &FieldDef) -> TokenStream {
        match field {
            FieldDef::Named { .. } => quote! {
                named_field
            },
            FieldDef::Unnamed { .. } => quote! {
                unnamed_field
            },
            FieldDef::UnitStruct { struct_name, .. } => quote! {
                #struct_name
            },
            FieldDef::UnitEnum {
                enum_name,
                variant_name,
                ..
            } => quote! {
                #enum_name::#variant_name
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
        let constructor = construct(&Constructor::Struct(item), dummy_callback);
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyStruct {
                    field1: named_field,
                    field2: named_field
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
        let constructor = construct(&Constructor::Struct(item), dummy_callback);
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyStruct(unnamed_field, unnamed_field)
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
        let constructor = construct(&Constructor::Struct(item), dummy_callback);
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyStruct
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
                Variant2(u32, String),
                Variant3
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let constructor = construct(
            &Constructor::Enum(item.clone(), item.variants.first().unwrap().clone()),
            dummy_callback,
        );
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyEnum::Variant1 { field1: named_field, field2: named_field }
            }
            .to_string()
        );
    }

    #[test]
    fn construct_enum_unnamed() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Variant1 { field1: i32, field2: String },
                Variant2(u32, String),
                Variant3
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let constructor = construct(
            &Constructor::Enum(item.clone(), item.variants.get(1).unwrap().clone()),
            dummy_callback,
        );
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyEnum::Variant2(unnamed_field, unnamed_field)
            }
            .to_string()
        );
    }

    #[test]
    fn construct_enum_unit() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Variant1 { field1: i32, field2: String },
                Variant2(u32, String),
                Variant3
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let constructor = construct(
            &Constructor::Enum(item.clone(), item.variants.last().unwrap().clone()),
            dummy_callback,
        );
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyEnum::Variant3
            }
            .to_string()
        );
    }
}
