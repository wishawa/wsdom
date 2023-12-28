use crate::parser::{
    generic::GenericArgs,
    ts_type::{NamedType, TsType},
};
use proc_macro2::TokenStream;
use quote::quote;

use super::{util::new_ident_safe, Context};

pub(super) mod known_types {
    use super::SimplifiedType;

    pub(crate) const fn simple_named_type(name: &'static str) -> SimplifiedType<'static> {
        SimplifiedType {
            name,
            args: Vec::new(),
        }
    }

    pub(crate) const NUMBER: SimplifiedType = simple_named_type("number");
    pub(crate) const BOOLEAN: SimplifiedType = simple_named_type("boolean");
    pub(crate) const UNKNOWN: SimplifiedType = simple_named_type("unknown");
    pub(crate) const OBJECT: SimplifiedType = simple_named_type("object");
    pub(crate) const STRING: SimplifiedType = simple_named_type("string");
    pub(crate) const NULL: SimplifiedType = simple_named_type("__translate_nullish");
}

#[derive(Clone, PartialEq)]
pub(crate) struct SimplifiedType<'a> {
    pub name: &'a str,
    pub args: Vec<SimplifiedType<'a>>,
}

mod fold {
    use super::SimplifiedType;

    pub(crate) fn fold_type<'a>(
        t: SimplifiedType<'a>,
        replace: &mut impl FnMut(&str) -> Option<SimplifiedType<'a>>,
    ) -> SimplifiedType<'a> {
        if let Some(rep) = replace(t.name) {
            rep
        } else {
            SimplifiedType {
                name: t.name,
                args: t
                    .args
                    .into_iter()
                    .map(|arg| fold_type(arg, replace))
                    .collect(),
            }
        }
    }
}

impl<'a> Context<'a> {
    pub(super) fn visit_all_ancestors<'s, R>(
        &'s self,
        ty: &SimplifiedType<'a>,
        visitor: &mut impl FnMut(&SimplifiedType<'a>) -> Option<R>,
    ) -> Option<R>
    where
        'a: 's,
    {
        use std::collections::HashMap;

        // Ty<GD1, GD2, ...> extends OldAnc1<GDX>, OldAnc2<...>, ...
        let (gd, old_ancs) = self.inhr_graph.get(ty.name)?;
        let replacements = gd
            .args
            .iter()
            .zip(ty.args.iter())
            .map(|(target, replace)| (target.name, replace))
            .collect::<HashMap<_, _>>();
        let new_ancs = old_ancs.iter().map(|anc| {
            let t = self.simplify_type(TsType::Named { ty: anc.to_owned() });
            fold::fold_type(t, &mut |name| replacements.get(name).cloned().cloned())
        });
        // TODO: should this be BFS instead?
        for anc in new_ancs {
            if let Some(x) = visitor(&anc) {
                return Some(x);
            }
            self.visit_all_ancestors(&anc, visitor);
        }
        None
    }

    pub(super) fn simplify_type(&self, t: TsType<'a>) -> SimplifiedType<'a> {
        match t {
            TsType::Named {
                ty:
                    NamedType {
                        name: "null" | "void" | "undefined",
                        ..
                    },
            } => SimplifiedType {
                name: "__translate_nullish",
                args: Vec::new(),
            },
            TsType::Named {
                ty:
                    NamedType {
                        name: "Exclude" | "Readonly",
                        generic: GenericArgs { args },
                    },
            } => args
                .into_iter()
                .next()
                .map(|arg| self.simplify_type(arg))
                .unwrap_or(known_types::OBJECT),
            TsType::Named {
                ty: NamedType { name: "Record", .. },
            } => known_types::OBJECT,
            TsType::Named {
                ty:
                    NamedType {
                        name,
                        generic: GenericArgs { args },
                    },
            } => SimplifiedType {
                name,
                args: args.into_iter().map(|t| self.simplify_type(t)).collect(),
            },
            TsType::Union { pair } => {
                self.unify_types(self.simplify_type(pair.0), self.simplify_type(pair.1))
            }
            TsType::StringLit { .. } => known_types::STRING,
            TsType::IntLit { .. } => known_types::NUMBER,
            TsType::BoolLit { .. } => known_types::BOOLEAN,
            TsType::Array { item } => SimplifiedType {
                name: "__translate_array",
                args: vec![self.simplify_type(*item)],
            },
            TsType::FixedArray { types } => SimplifiedType {
                name: "__translate_array",
                args: vec![types
                    .into_iter()
                    .map(|ty| self.simplify_type(ty))
                    .reduce(|acc, item| self.unify_types(acc, item))
                    .unwrap_or(known_types::UNKNOWN)],
            },
            TsType::PatternString { .. } => known_types::STRING,
            TsType::TsIs { .. } => known_types::BOOLEAN,
            TsType::KeyOf { .. } => known_types::STRING,
            _ => known_types::UNKNOWN,
        }
    }
    pub(super) fn unify_types(
        &self,
        t1: SimplifiedType<'a>,
        t2: SimplifiedType<'a>,
    ) -> SimplifiedType<'a> {
        macro_rules! tuple_and_rev {
            ($a:pat, $b:pat) => {
                ($a, $b) | ($b, $a)
            };
        }
        match (t1, t2) {
            (u1, u2) if u1 == u2 => u1,
            (SimplifiedType { name: n1, args: a1 }, SimplifiedType { name: n2, args: a2 })
                if n1 == n2 =>
            {
                SimplifiedType {
                    name: n1,
                    args: a1
                        .into_iter()
                        .zip(a2.into_iter())
                        .map(|(a1, a2)| self.unify_types(a1, a2))
                        .collect(),
                }
            }
            tuple_and_rev!(
                SimplifiedType {
                    name: "__translate_nullish",
                    ..
                },
                u1@SimplifiedType {
                    name: "__translate_nullable",
                    ..
                }
            ) => u1,
            tuple_and_rev!(
                ty,
                SimplifiedType {
                    name: "__translate_nullish",
                    ..
                }
            ) => SimplifiedType {
                name: "__translate_nullable",
                args: vec![ty],
            },
            (ty1, ty2) => {
                use std::collections::HashMap;
                let mut ancs1 = HashMap::new();
                self.visit_all_ancestors(&ty1, &mut |anc| {
                    ancs1.insert(anc.name, anc.to_owned());
                    None::<()>
                });
                ancs1.insert(ty1.name, ty1);
                if let Some(x) = ancs1.get(ty2.name) {
                    self.unify_types(x.to_owned(), ty2.to_owned())
                } else {
                    match self.visit_all_ancestors(&ty2, &mut |anc| {
                        ancs1.remove(anc.name).map(|got| (anc.to_owned(), got))
                    }) {
                        Some((from_1, from_2)) => self.unify_types(from_1, from_2),
                        None => known_types::UNKNOWN,
                    }
                }
            }
        }
    }

    pub(super) fn convert_type(&self, ty: SimplifiedType<'_>) -> TokenStream {
        let (name, common) = match ty.name {
            "unknown" | "any" => ("JsValue", true),
            "object" => ("JsObject", true),
            "number" => ("JsNumber", true),
            "string" => ("JsString", true),
            "boolean" => ("JsBoolean", true),
            "symbol" => ("JsSymbol", true),
            "this" => return quote! { Self },
            "__translate_nullish" => ("JsNullish", true),
            "__translate_nullable" => ("JsNullable", true),
            "__translate_array" | "ReadonlyArray" => ("Array", false),
            name => (name, false),
        };
        let ident = new_ident_safe(name);
        let prefix = common.then(|| quote! {__wsdom_load_ts_macro::});
        if ty.args.is_empty() {
            quote! {
                #prefix #ident
            }
        } else {
            let ga = ty.args.into_iter().map(|a| self.convert_type(a));
            quote! {
                #prefix #ident<#(#ga,)*>
            }
        }
    }
}
