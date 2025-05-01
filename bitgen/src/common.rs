use proc_macro2::Span;
use syn;
use syn::Attribute;
use syn::FieldsNamed;
use syn::FieldsUnnamed;
use syn::Ident;
use syn::LitInt;
use syn::Type;
use syn::{Expr, Meta, MetaNameValue};

#[derive(Clone)]
pub enum GenItem {
    Struct(syn::ItemStruct),
    Enum(syn::ItemEnum, syn::Variant),
}

#[derive(Clone)]
pub enum FieldDef {
    Named {
        attrs: Vec<Attribute>,
        index: LitInt,
        name: Ident,
        ty: Type,
    },
    Unnamed {
        attrs: Vec<Attribute>,
        index: LitInt,
        /// Variable name for the match, e.g. `m0` in `SomeValue(m0, m1)`, used only during destructuring
        var_match: Ident,
        ty: Type,
    },
    UnitStruct {
        /// Attributes of the struct
        attrs: Vec<Attribute>,
    },
    UnitEnum {
        attrs: Vec<Attribute>,
        discriminant: Option<Expr>,
        variant_name: Ident,
    },
}

// Maybe I should create collection struct and methods to it?

/// Build field definitions for named fields
pub fn build_field_defs_named(fields: &FieldsNamed) -> Vec<FieldDef> {
    fields
        .named
        .iter()
        .enumerate()
        .map(|(index, field)| FieldDef::Named {
            attrs: field.attrs.clone(),
            index: LitInt::new(&index.to_string(), Span::call_site()),
            name: field.ident.clone().expect("Expected named fields"),
            ty: field.ty.clone(),
        })
        .collect::<Vec<_>>()
}

/// Build field definitions for unnamed fields
pub fn build_field_defs_unnamed(fields: &FieldsUnnamed) -> Vec<FieldDef> {
    fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, field)| FieldDef::Unnamed {
            attrs: field.attrs.clone(),
            index: LitInt::new(&index.to_string(), Span::call_site()),
            var_match: Ident::new(&format!("m{}", index), Span::call_site()),
            ty: field.ty.clone(),
        })
        .collect::<Vec<_>>()
}

/// Get the field names for named fields. This is used for destructuring.
pub fn get_field_names(fields: &Vec<FieldDef>) -> Vec<Ident> {
    fields
        .iter()
        .filter_map(|field| {
            if let FieldDef::Named { name, .. } = field {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

/// Get the field matchers for unnamed fields. This is used for destructuring,
/// e.g. `m0` in `SomeValue(m0, m1)`.
pub fn get_field_matchers(fields: &Vec<FieldDef>) -> Vec<Ident> {
    fields
        .iter()
        .filter_map(|field| {
            if let FieldDef::Unnamed { var_match, .. } = field {
                Some(var_match.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn find_attr_by_name(attrs: &Vec<Attribute>, name: &str) -> Option<Expr> {
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
