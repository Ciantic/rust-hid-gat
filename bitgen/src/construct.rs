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
    Enum(syn::ItemEnum),
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
    Unit {
        attrs: Vec<Attribute>,
        ctor_name: Ident,
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
                    let field_value = cb(&FieldDef::Unit {
                        attrs: top_level_attrs,
                        ctor_name: struct_name.clone(),
                    });
                    quote! {
                        #field_value
                    }
                }
            }
        }
        Constructor::Enum(ienum) => {
            for variant in &ienum.variants {
                let variant_name = &variant.ident;
                let variant_attrs = variant.attrs.clone();
                match &variant.fields {
                    Fields::Named(fields) => {
                        let field_defs = get_field_defs(fields);
                        let field_values = field_defs.iter().map(cb);
                        return quote! {
                            #variant_name {
                                #(#field_values),*
                            }
                        };
                    }
                    Fields::Unnamed(unnamed) => {
                        let field_defs = get_field_defs_unnamed(unnamed);
                        let field_values = field_defs.iter().map(cb);
                        return quote! {
                            #variant_name (
                                #(#field_values),*
                            )
                        };
                    }
                    Fields::Unit => {
                        return quote! {
                            #variant_name
                        };
                    }
                }
                return quote! { zzz };
            }
            quote! {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_callback(_field: &FieldDef) -> TokenStream {
        quote! {
            foo
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
                    field1: foo,
                    field2: foo
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
                MyStruct(foo, foo)
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
                foo
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
        let constructor = construct(&Constructor::Enum(item), dummy_callback);
        assert_eq!(
            constructor.to_string(),
            quote! {
                MyEnum::Variant1 { field1: foo, field2: foo }
            }
            .to_string()
        );
    }
}
