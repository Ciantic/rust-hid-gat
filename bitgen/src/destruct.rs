use proc_macro2::TokenStream;
use quote::quote;
use syn;
use syn::Attribute;
use syn::Fields;
use syn::Type;
use syn::TypePath;

use crate::common::build_field_defs_named;
use crate::common::build_field_defs_unnamed;
use crate::common::get_field_matchers;
use crate::common::get_field_names;
use crate::common::FieldDef;
use crate::common::GenItem;

pub struct DestructurerCbArg {
    // var_name: Ident,
    pub type_name: Type,
    pub top_level_attrs: Vec<Attribute>,
    pub field: FieldDef,
}

pub struct Destructurer<
    W: Fn(Vec<TokenStream>) -> TokenStream,
    D: Fn(&DestructurerCbArg) -> TokenStream,
> {
    pub item: GenItem,
    pub wrapper: W,
    pub destructrurer: D,
}

/// Returns destructuring syntax for pattern matching
///
/// Result does not contain `match foo {}` only single arm of the match case.
pub fn destruct<T, D>(args: &Destructurer<T, D>) -> TokenStream
where
    T: Fn(Vec<TokenStream>) -> TokenStream,
    D: Fn(&DestructurerCbArg) -> TokenStream,
{
    let cb = &args.destructrurer;
    let wrapper = &args.wrapper;
    match &args.item {
        GenItem::Struct(item_struct) => {
            let struct_name = Type::Path(TypePath {
                qself: None,
                path: item_struct.ident.clone().into(),
            });
            let my_cb = |field: &FieldDef| {
                cb(&DestructurerCbArg {
                    top_level_attrs: item_struct.attrs.clone(),
                    type_name: struct_name.clone(),
                    // var_name: var_name.clone(),
                    field: field.clone(),
                })
            };
            match &item_struct.fields {
                Fields::Named(fields) => {
                    let field_defs = build_field_defs_named(fields);
                    let field_names = get_field_names(&field_defs);
                    let field_values = field_defs.iter().map(my_cb);
                    let body = wrapper(field_values.collect());
                    quote! {
                        #struct_name {
                            #(#field_names),*
                        } => {
                            #body
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_defs = build_field_defs_unnamed(fields);
                    let field_values = field_defs.iter().map(my_cb);
                    let field_matchers = get_field_matchers(&field_defs);
                    let body = wrapper(field_values.collect());
                    quote! {
                        #struct_name(
                            #(#field_matchers),*
                        ) => {
                            #body
                        }
                    }
                }
                Fields::Unit => {
                    let field_value = cb(&&DestructurerCbArg {
                        top_level_attrs: item_struct.attrs.clone(),
                        type_name: struct_name.clone(),
                        // var_name: var_name.clone(),
                        field: FieldDef::UnitStruct {
                            attrs: item_struct.attrs.clone(),
                        },
                    });
                    let body = wrapper(vec![field_value]);
                    quote! {
                        #struct_name => {
                            #body
                        }
                    }
                }
            }
        }
        GenItem::Enum(ienum, variant) => {
            let enum_name = Type::Path(TypePath {
                qself: None,
                path: ienum.ident.clone().into(),
            });
            let variant_name = &variant.ident;
            let my_cb = |field: &FieldDef| {
                cb(&DestructurerCbArg {
                    top_level_attrs: ienum.attrs.clone(),
                    type_name: enum_name.clone(),
                    // var_name: var_name.clone(),
                    field: field.clone(),
                })
            };

            match &variant.fields {
                Fields::Named(fields) => {
                    let field_defs = build_field_defs_named(fields);
                    let field_names = get_field_names(&field_defs);
                    let field_values = field_defs.iter().map(my_cb);
                    let body = wrapper(field_values.collect());
                    quote! {
                        #enum_name::#variant_name {
                            #(#field_names),*
                        } => {
                            #body
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_defs = build_field_defs_unnamed(fields);
                    let field_values = field_defs.iter().map(my_cb);
                    let field_matchers = get_field_matchers(&field_defs);
                    let body = wrapper(field_values.collect());
                    quote! {
                        #enum_name::#variant_name(
                            #(#field_matchers),*
                        ) => {
                            #body
                        }
                    }
                }
                Fields::Unit => {
                    let field_value = cb(&DestructurerCbArg {
                        top_level_attrs: ienum.attrs.clone(),
                        type_name: enum_name.clone(),
                        // var_name: var_name.clone(),
                        field: FieldDef::UnitEnum {
                            attrs: vec![],
                            variant_name: variant.ident.clone(),
                            discriminant: variant.discriminant.clone().map(|d| d.1),
                        },
                    });
                    let body = wrapper(vec![field_value]);
                    quote! {
                        #enum_name::#variant_name => {
                            #body
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::{ItemEnum, ItemStruct};

    use super::*;

    fn dummy_callback(args: &DestructurerCbArg) -> TokenStream {
        // let var_name = &args.var_name;
        let type_name = &args.type_name;
        match &args.field {
            FieldDef::Named { name, ty, .. } => {
                quote! {
                    my_destructor::<#ty>(#name)?;
                }
            }
            FieldDef::Unnamed { var_match, ty, .. } => {
                quote! {
                    my_destructor::<#ty>(#var_match)?;
                }
            }
            FieldDef::UnitStruct { .. } => quote! {
                my_destructor::<#type_name>()?;
            },
            FieldDef::UnitEnum { variant_name, .. } => quote! {
                my_destructor::<#type_name::#variant_name>()?;
            },
        }
    }

    #[test]
    fn test_destruct_struct_named() {
        let input = quote! {
            #[derive(Debug)]
            struct MyStruct {
                field1: i32,
                field2: String,
            }
        }
        .to_string();
        let item: ItemStruct = syn::parse_str(&input).unwrap();
        let destructed = destruct(&Destructurer {
            item: GenItem::Struct(item),
            wrapper: |fields| {
                quote! {
                    my_destructor.init()?;
                    #(#fields)*
                    my_destructor.finish()?;
                }
            },
            destructrurer: dummy_callback,
        });

        assert_eq!(
            quote! {
                MyStruct { field1, field2 } => {
                    my_destructor.init()?;
                    my_destructor::<i32>(field1)?;
                    my_destructor::<String>(field2)?;
                    my_destructor.finish()?;
                }
            }
            .to_string(),
            destructed.to_string()
        )
    }

    #[test]
    fn test_destruct_struct_unnamed() {
        let input = quote! {
            #[derive(Debug)]
            struct MyStruct(i32, String);
        }
        .to_string();
        let item: ItemStruct = syn::parse_str(&input).unwrap();

        let destructed = destruct(&Destructurer {
            item: GenItem::Struct(item),
            wrapper: |fields| {
                quote! {
                    my_destructor.init()?;
                    #(#fields)*
                    my_destructor.finish()?;
                }
            },
            destructrurer: dummy_callback,
        });

        assert_eq!(
            quote! {
                MyStruct(m0, m1) => {
                    my_destructor.init()?;
                    my_destructor::<i32>(m0)?;
                    my_destructor::<String>(m1)?;
                    my_destructor.finish()?;
                }
            }
            .to_string(),
            destructed.to_string()
        )
    }

    #[test]
    fn test_destruct_struct_unit() {
        let input = quote! {
            #[derive(Debug)]
            struct MyStruct;
        }
        .to_string();
        let item: ItemStruct = syn::parse_str(&input).unwrap();

        let destructed = destruct(&Destructurer {
            item: GenItem::Struct(item),
            wrapper: |fields| {
                quote! {
                    my_destructor.init()?;
                    #(#fields)*
                    my_destructor.finish()?;
                }
            },
            destructrurer: dummy_callback,
        });

        assert_eq!(
            quote! {
                MyStruct => {
                    my_destructor.init()?;
                    my_destructor::<MyStruct>()?;
                    my_destructor.finish()?;
                }
            }
            .to_string(),
            destructed.to_string()
        )
    }

    #[test]
    fn test_destruct_enum_named() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Variant1 { field1: i32, field2: String },
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let destructured = destruct(&Destructurer {
            item: GenItem::Enum(item.clone(), item.variants.last().unwrap().clone()),
            wrapper: |fields| {
                quote! {
                    my_destructor.init()?;
                    #(#fields)*
                    my_destructor.finish()?;
                }
            },
            destructrurer: dummy_callback,
        });
        assert_eq!(
            destructured.to_string(),
            quote! {
                MyEnum::Variant1 {
                    field1,
                    field2
                } => {
                    my_destructor.init()?;
                    my_destructor::<i32>(field1)?;
                    my_destructor::<String>(field2)?;
                    my_destructor.finish()?;
                }
            }
            .to_string()
        );
    }

    #[test]
    fn test_destruct_enum_unnamed() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Variant1(i32, String),
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let destructured = destruct(&Destructurer {
            item: GenItem::Enum(item.clone(), item.variants.last().unwrap().clone()),
            wrapper: |fields| {
                quote! {
                    my_destructor.init()?;
                    #(#fields)*
                    my_destructor.finish()?;
                }
            },
            destructrurer: dummy_callback,
        });
        assert_eq!(
            destructured.to_string(),
            quote! {
                MyEnum::Variant1(m0, m1) => {
                    my_destructor.init()?;
                    my_destructor::<i32>(m0)?;
                    my_destructor::<String>(m1)?;
                    my_destructor.finish()?;
                }
            }
            .to_string()
        );
    }

    #[test]
    fn test_destruct_enum_unit() {
        let input = quote! {
            #[derive(Debug)]
            enum MyEnum {
                Variant1,
            }
        }
        .to_string();
        let item: ItemEnum = syn::parse_str(&input).unwrap();
        let destructured = destruct(&Destructurer {
            item: GenItem::Enum(item.clone(), item.variants.last().unwrap().clone()),
            wrapper: |fields| {
                quote! {
                    my_destructor.init()?;
                    #(#fields)*
                    my_destructor.finish()?;
                }
            },
            destructrurer: dummy_callback,
        });
        assert_eq!(
            destructured.to_string(),
            quote! {
                MyEnum::Variant1 => {
                    my_destructor.init()?;
                    my_destructor::<MyEnum::Variant1>()?;
                    my_destructor.finish()?;
                }
            }
            .to_string()
        );
    }
}
