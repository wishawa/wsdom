mod class;
mod utils;

use std::{borrow::Cow, collections::HashMap};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::parser::{
    comment::WithComment,
    declare_var::DeclareVar,
    generic::GenericsDeclaration,
    interface::Interface,
    item::Item,
    member::Member,
    method::{Method, MethodName},
    ts_type::{NamedType, TsType},
};

mod known_types {
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
    pub(crate) const UNKNOWN: TsType = simple_named_type("unknown");
    pub(crate) const OBJECT: TsType = simple_named_type("object");
    pub(crate) const STRING: TsType = simple_named_type("string");
}

#[derive(Default)]
struct Context<'a> {
    types: HashMap<&'a str, Interface<'a>>,
    aliases: HashMap<&'a str, (GenericsDeclaration<'a>, TsType<'a>)>,
    declare_globals: HashMap<&'a str, &'a TsType<'a>>,
}

impl<'a> Context<'a> {
    fn simplify_type(&self, t: TsType<'a>) -> TsType<'a> {
        match t {
            TsType::Named { .. } => t,
            TsType::Union { pair } => {
                self.unify_types(self.simplify_type(pair.0), self.simplify_type(pair.1))
            }
            TsType::StringLit { .. } => known_types::STRING,
            TsType::IntLit { .. } => known_types::NUMBER,
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
            (TsType::Array { item: item1 }, TsType::Array { item: item2 }) => TsType::Array {
                item: Box::new(self.unify_types(*item1, *item2)),
            },
            _ => known_types::OBJECT,
        }
    }

    fn ts_type_to_rust(&self, t: TsType<'_>) -> TokenStream {
        match self.simplify_type(t) {
            TsType::Named { ty } => {
                let (name, common) = match ty.name {
                    "unknown" | "any" => ("JsValue", true),
                    "object" => ("JsObject", true),
                    "number" => ("JsNumber", true),
                    "string" => ("JsString", true),
                    "boolean" => ("JsBoolean", true),
                    "void" => ("JsUndefined", true),
                    name => (name, false),
                };
                let ident = Ident::new(name, Span::call_site());
                let prefix = common.then(|| quote! {__wrmi_load_ts_macro::});
                if ty.generic.args.is_empty() {
                    quote! {
                        #prefix #ident
                    }
                } else {
                    let ga = ty.generic.args.into_iter().map(|a| self.ts_type_to_rust(a));
                    quote! {
                        #prefix #ident<#(#ga,)*>
                    }
                }
            }
            TsType::Array { item } => {
                let inner = self.ts_type_to_rust(*item);
                quote! { __wrmi_load_ts_macro::JsArray<#inner> }
            }
            TsType::KeyOf { .. } => quote! { __wrmi_load_ts_macro::JsString },
            _ => quote! { __wrmi_load_ts_macro::JsObject },
        }
    }
    // fn find_decl(&self, name: &str) -> Option<&'a [WithComment<'a, Member<'a>>]> {
    //     self.declare_classes
    //         .iter()
    //         .rev()
    //         .flat_map(|hm| hm.get(name).cloned())
    //         .next()
    // }
}
pub(crate) fn make_types<'a>(dts: &[WithComment<'a, Item<'a>>]) -> TokenStream {
    let mut generated_code = Vec::<TokenStream>::new();
    let mut types = HashMap::new();
    let mut aliases = HashMap::new();
    let mut declare_globals = HashMap::new();
    for item in dts {
        match &item.data {
            Item::Interface(interface) => {
                let interface = interface.clone();
                types.insert(interface.name, interface);
            }
            Item::TypeAlias(alias) => {
                let alias = alias.clone();
                aliases.insert(alias.name, (alias.generics, alias.ty));
            }
            Item::DeclareVar(DeclareVar { name, ty }) => {
                declare_globals.insert(*name, ty);
            }
            _ => {}
        }
    }
    let ctx = Context {
        types,
        aliases,
        declare_globals,
    };
    for item in dts {
        match &item.data {
            Item::DeclareVar(DeclareVar { name, ty }) => {
                let Some((decl_members, direct_decl)) = (match ty {
                    TsType::Named {
                        ty: NamedType { name, .. },
                    } => ctx
                        .types
                        .get(name)
                        .map(|interface| (&*interface.members, false)),
                    TsType::Interface { members } => Some((&**members, true)),
                    _ => None,
                }) else {
                    continue;
                };

                let iface = decl_members
                    .iter()
                    .find_map(|member| match &member.data {
                        Member::Method(Method {
                            name: MethodName::Constructor,
                            ret:
                                TsType::Named {
                                    ty: NamedType { name, .. },
                                },
                            ..
                        }) => ctx.types.get(name),
                        _ => None,
                    })
                    .or_else(|| direct_decl.then(|| ctx.types.get(name)).flatten())
                    .map(Cow::Borrowed)
                    .unwrap_or_else(|| {
                        Cow::Owned(Interface {
                            extends: Default::default(),
                            generics: Default::default(),
                            members: Default::default(),
                            name,
                        })
                    });

                generated_code.push(ctx.make_class(&*iface, decl_members));
            }
            _ => {}
        }
    }
    quote! {
        #(#generated_code)*
    }
}
