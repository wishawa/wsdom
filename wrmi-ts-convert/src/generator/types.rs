use crate::parser::{
    generic::GenericArgs,
    ts_type::{NamedType, TsType},
};
use proc_macro2::TokenStream;
use quote::quote;

use super::{util::new_ident_safe, Context};

pub(super) mod known_types {
    use crate::parser::{
        generic::GenericArgs,
        ts_type::{NamedType, TsType},
    };

    pub(crate) const fn simple_named_type(name: &'static str) -> TsType<'static> {
        TsType::Named {
            ty: NamedType {
                name,
                generic: GenericArgs { args: Vec::new() },
            },
        }
    }

    pub(crate) const NUMBER: TsType = simple_named_type("number");
    pub(crate) const BOOLEAN: TsType = simple_named_type("boolean");
    pub(crate) const UNKNOWN: TsType = simple_named_type("unknown");
    pub(crate) const OBJECT: TsType = simple_named_type("object");
    pub(crate) const STRING: TsType = simple_named_type("string");
    pub(crate) const NULL: TsType = simple_named_type("null");
}

impl<'a> Context<'a> {
    fn simplify_type(&self, t: TsType<'a>) -> TsType<'a> {
        match t {
            TsType::Named {
                ty:
                    NamedType {
                        name: "null" | "void" | "undefined",
                        ..
                    },
            } => TsType::Named {
                ty: NamedType {
                    name: "__translate_nullish",
                    generic: Default::default(),
                },
            },
            TsType::Named {
                ty:
                    NamedType {
                        name: "Exclude",
                        generic: GenericArgs { args },
                    },
            } => args.into_iter().rev().next().unwrap_or(known_types::OBJECT),
            TsType::Named { .. } => t,
            TsType::Union { pair } => {
                self.unify_types(self.simplify_type(pair.0), self.simplify_type(pair.1))
            }
            TsType::StringLit { .. } => known_types::STRING,
            TsType::IntLit { .. } => known_types::NUMBER,
            TsType::BoolLit { .. } => known_types::BOOLEAN,
            TsType::Array { item } => TsType::Array {
                item: Box::new(self.simplify_type(*item)),
            },
            TsType::FixedArray { types } => TsType::Array {
                item: Box::new(
                    types
                        .into_iter()
                        .map(|ty| self.simplify_type(ty))
                        .reduce(|acc, item| self.unify_types(acc, item))
                        .unwrap_or(known_types::OBJECT),
                ),
            },
            TsType::PatternString { .. } => known_types::STRING,
            _ => known_types::OBJECT,
        }
    }
    fn unify_types(&self, t1: TsType<'a>, t2: TsType<'a>) -> TsType<'a> {
        macro_rules! tuple_and_rev {
            ($a:pat, $b:pat) => {
                ($a, $b) | ($b, $a)
            };
        }
        match (t1, t2) {
            (u1, u2) if u1 == u2 => u1,
            (
                TsType::Named {
                    ty:
                        NamedType {
                            name: n1,
                            generic: mut g1,
                        },
                },
                TsType::Named {
                    ty:
                        NamedType {
                            name: n2,
                            generic: g2,
                        },
                },
            ) if n1 == n2 => {
                g1.args = g1
                    .args
                    .into_iter()
                    .zip(g2.args.into_iter())
                    .map(|(a1, a2)| self.unify_types(a1, a2))
                    .collect();
                TsType::Named {
                    ty: NamedType {
                        name: n1,
                        generic: g1,
                    },
                }
            }
            tuple_and_rev!(
                TsType::Named {
                    ty:
                        NamedType {
                            name: "__translate_nullish",
                            ..
                        },
                },
                u1 @ TsType::Named {
                    ty:
                        NamedType {
                            name: "__translate_nullable",
                            ..
                        },
                }
            ) => u1,
            tuple_and_rev!(
                u1,
                TsType::Named {
                    ty: NamedType {
                        name: "__translate_nullish",
                        ..
                    },
                }
            ) => TsType::Named {
                ty: NamedType {
                    name: "__translate_nullable",
                    generic: GenericArgs { args: vec![u1] },
                },
            },

            (TsType::Array { item: item1 }, TsType::Array { item: item2 }) => TsType::Array {
                item: Box::new(self.unify_types(*item1, *item2)),
            },
            _ => known_types::OBJECT,
        }
    }

    pub(super) fn convert_type(&self, t: TsType<'_>) -> TokenStream {
        match self.simplify_type(t) {
            TsType::Named { ty } => {
                let (name, common) = match ty.name {
                    "unknown" | "any" => ("JsValue", true),
                    "object" => ("JsObject", true),
                    "number" => ("JsNumber", true),
                    "string" => ("JsString", true),
                    "boolean" => ("JsBoolean", true),
                    "__translate_nullish" => ("JsNullish", true),
                    "__translate_nullable" => ("JsNullable", true),
                    name => (name, false),
                };
                let ident = new_ident_safe(name);
                let prefix = common.then(|| quote! {__wrmi_load_ts_macro::});
                if ty.generic.args.is_empty() {
                    quote! {
                        #prefix #ident
                    }
                } else {
                    let ga = ty.generic.args.into_iter().map(|a| self.convert_type(a));
                    quote! {
                        #prefix #ident<#(#ga,)*>
                    }
                }
            }
            TsType::Array { item } => {
                let inner = self.convert_type(*item);
                quote! { __wrmi_load_ts_macro::JsArray<#inner> }
            }
            TsType::KeyOf { .. } => quote! { __wrmi_load_ts_macro::JsString },
            _ => quote! { __wrmi_load_ts_macro::JsObject },
        }
    }
}
