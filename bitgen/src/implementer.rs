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
use syn::Item;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::Lit;
use syn::{Expr, Meta, MetaNameValue};

use crate::common::FieldDef;
use crate::common::GenItem;
use crate::construct::construct;
use crate::construct::Constructor;
use crate::construct::ConstructorCbArg;
use crate::destruct::destruct;
use crate::destruct::Destructurer;
use crate::destruct::DestructurerCbArg;

fn construct_callback(arg: &ConstructorCbArg) -> TokenStream {
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

fn destruct_callback(args: &DestructurerCbArg) -> TokenStream {
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
        FieldDef::UnitStruct => quote! {
            my_destructor::<#type_name>()?;
        },
        FieldDef::UnitEnum { variant_name, .. } => quote! {
            my_destructor::<#type_name::#variant_name>()?;
        },
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
                        destructed.push(destruct(&Destructurer {
                            item: genitem.clone(),
                            destructrurer: destruct_callback,
                        }));
                        constructed.push(quote! {
                            if stars_are_aligned::<#enum_name::#variant_name>() {
                                return #constr;
                            }
                        });
                    }

                    (
                        enum_name,
                        quote! {
                            #(#destructed),*
                        },
                        quote! {
                            #(#constructed)*
                            panic!("No matching variant found")
                        },
                    )
                }
                _ => return None,
            };

            Some(quote! {
                impl Foo for #name {
                    fn from_foo(self) -> #name {
                        match self {
                            #destructed
                        }
                    }
                    fn to_foo(self) -> #name {
                        #constructed
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
        let some_things = quote! {

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
        let res = syn::parse2::<syn::File>(some_things).unwrap();
        let output_toks = implementer(&res.items);
        let output = quote! {
            #(#output_toks)*
        };
        assert_eq!(
            pretty_string(output),
            pretty_string(quote! {
                impl Foo for MyStruct {
                    fn from_foo(self) -> MyStruct {
                        match self {
                            MyStruct { field1, field2 } => {
                                my_destructor::<i32>(field1)?;
                                my_destructor::<String>(field2)?;
                            }
                        }
                    }
                    fn to_foo(self) -> MyStruct {
                        MyStruct {
                            field1: my_maker::<i32>(field1),
                            field2: my_maker::<String>(field2),
                        }
                    }
                }

                impl Foo for AnotherStruct {
                    fn from_foo(self) -> AnotherStruct {
                        match self {
                            AnotherStruct(m0, m1) => {
                                my_destructor::<u32>(m0)?;
                                my_destructor::<String>(m1)?;
                            }
                        }
                    }
                    fn to_foo(self) -> AnotherStruct {
                        AnotherStruct(my_maker::<u32>(0), my_maker::<String>(1))
                    }
                }

                impl Foo for ThirdStruct {
                    fn from_foo(self) -> ThirdStruct {
                        match self {
                            ThirdStruct => {
                                my_destructor::<ThirdStruct>()?;
                            }
                        }
                    }
                    fn to_foo(self) -> ThirdStruct {
                        my_maker::<ThirdStruct>()
                    }
                }

                impl Foo for MyEnum {
                    fn from_foo(self) -> MyEnum {
                        match self {
                            MyEnum::NamedVariant { field, field2 } => {
                                my_destructor::<u32>(field)?;
                                my_destructor::<String>(field2)?;
                            }
                            MyEnum::UnnamedVariant(m0, m1) => {
                                my_destructor::<u32>(m0)?;
                                my_destructor::<String>(m1)?;
                            }
                            MyEnum::UnitVariant => {
                                my_destructor::<MyEnum::UnitVariant>()?;
                            }
                        }
                    }
                    fn to_foo(self) -> MyEnum {
                        if stars_are_aligned::<MyEnum::NamedVariant>() {
                            return MyEnum::NamedVariant {
                                field: my_maker::<u32>(field),
                                field2: my_maker::<String>(field2),
                            };
                        }
                        if stars_are_aligned::<MyEnum::UnnamedVariant>() {
                            return MyEnum::UnnamedVariant(my_maker::<u32>(0), my_maker::<String>(1));
                        }
                        if stars_are_aligned::<MyEnum::UnitVariant>() {
                            return my_maker::<MyEnum::UnitVariant>();
                        }
                        panic!("No matching variant found")
                    }
                }
            })
        );
    }
}

// struct MyStruct {
//     field1: i32,
//     field2: String,
// }

// impl Foo for MyStruct {
//     fn from_foo(self) -> MyStruct {
//         match self {
//             MyStruct { field1, field2 } => {
//                 my_destructor::<i32>(field1)?;
//                 my_destructor::<String>(field2)?;
//             }
//         }
//     }
//     fn to_foo(self) -> MyStruct {
//         match self {
//             MyStruct { .. } => { field1 : my_maker :: < i32 > (field1) , field2 : my_maker :: < String > (field2) }
//         }
//     }
// }

// struct MyStruct {
//     field1: i32,
//     field2: String,
// }

// struct AnotherStruct(u32, String);

// struct ThirdStruct;

// enum MyEnum {
//     NamedVariant { field: u32, field2: String },
//     UnnamedVariant(u32, String),
//     UnitVariant,
// }
// trait Foo {
//     fn from_foo(self) -> Self;
//     fn to_foo(self) -> Self;
// }

// impl Foo for MyStruct {
//     fn from_foo(self) -> MyStruct {
//         match self {
//             MyStruct { field1, field2 } => {
//                 my_destructor::<i32>(field1)?;
//                 my_destructor::<String>(field2)?;
//             }
//         }
//     }
//     fn to_foo(self) -> MyStruct {
//         MyStruct {
//             field1: my_maker::<i32>(field1),
//             field2: my_maker::<String>(field2),
//         }
//     }
// }
// impl Foo for AnotherStruct {
//     fn from_foo(self) -> AnotherStruct {
//         match self {
//             AnotherStruct(m0, m1) => {
//                 my_destructor::<u32>(m0)?;
//                 my_destructor::<String>(m1)?;
//             }
//         }
//     }
//     fn to_foo(self) -> AnotherStruct {
//         AnotherStruct(my_maker::<u32>(0), my_maker::<String>(1))
//     }
// }
// impl Foo for ThirdStruct {
//     fn from_foo(self) -> ThirdStruct {
//         match self {
//             ThirdStruct => {
//                 my_destructor::<ThirdStruct>()?;
//             }
//         }
//     }
//     fn to_foo(self) -> ThirdStruct {
//         my_maker::<ThirdStruct>()
//     }
// }

// impl Foo for MyEnum { fn from_foo (self) -> MyEnum { match self { MyEnum :: NamedVariant { field , field2 } => { my_destructor :: < u32 > (field) ? ; my_destructor :: < String > (field2) ? ; } , MyEnum :: UnnamedVariant (m0 , m1) => { my_destructor :: < u32 > (m0) ? ; my_destructor :: < String > (m1) ? ; } , MyEnum :: UnitVariant => { my_destructor :: < MyEnum :: UnitVariant > () ? ; } } } fn to_foo (self) -> MyEnum { MyEnum :: NamedVariant { field : my_maker :: < u32 > (field) , field2 : my_maker :: < String > (field2) },  MyEnum :: UnnamedVariant (my_maker :: < u32 > (0) , my_maker :: < String > (1)) , my_maker :: < MyEnum :: UnitVariant > () } }
