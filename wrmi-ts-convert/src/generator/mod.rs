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
    types: Vec<HashMap<&'a str, (GenericsDeclaration<'a>, TsType<'a>)>>,
    declares: Vec<HashMap<&'a str, &'a [WithComment<'a, Member<'a>>]>>,
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
                quote! { JsArray<#inner> }
            }
            TsType::KeyOf { .. } => quote! { JsString },
            _ => quote! { JsObject },
        }
    }

    fn find_decl(&self, name: &str) -> Option<&'a [WithComment<'a, Member<'a>>]> {
        self.declares
            .iter()
            .rev()
            .flat_map(|hm| hm.get(name).cloned())
            .next()
    }
}
pub(crate) fn make_types<'a>(dts: &[WithComment<'a, Item<'a>>]) -> TokenStream {
    fn make_rust_type(
        ctx: &Context<'_>,
        interface: &Interface<'_>,
        decl_var: &[WithComment<'_, Member<'_>>],
    ) -> TokenStream {
        let name = Ident::new(interface.name, Span::call_site());

        let (generics_with_bound, generics_without_bound) = if interface.generics.args.is_empty() {
            (None, None)
        } else {
            let with_bounds = interface.generics.args.iter().map(|arg| {
                let name = Ident::new(arg.name, Span::call_site());
                match &arg.extends {
                    Some(t) => {
                        let bound = ctx.ts_type_to_rust(t.to_owned());
                        quote! {
                            #name: AsRef<#bound>
                        }
                    }
                    None => {
                        quote! {
                            #name
                        }
                    }
                }
            });
            let without_bounds = interface.generics.args.iter().map(|arg| {
                let name = Ident::new(arg.name, Span::call_site());
                quote! {
                    #name
                }
            });
            (
                Some(quote! { <#(#with_bounds,)*> }),
                Some(quote! { <#(#without_bounds,)*> }),
            )
        };

        let generics_for_phantom = generics_without_bound
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(quote! {<()>}));

        let tokens = quote! {
            #[derive(::core::clone::Clone, __wrmi_load_ts_macro::RefCast)]
            #[repr(transparent)]
            struct #name #generics_with_bound (__wrmi_load_ts_macro::JsValue, ::core::marker::PhantomData #generics_for_phantom );
        };

        let extends = interface
            .extends
            .iter()
            .map(ToOwned::to_owned)
            .chain([known_types::UNKNOWN, known_types::OBJECT].into_iter())
            .map(|iface| ctx.ts_type_to_rust(iface.to_owned()));
        let first_extend = ctx.ts_type_to_rust(
            interface
                .extends
                .first()
                .unwrap_or(&known_types::OBJECT)
                .to_owned(),
        );

        let tokens = quote! {
            #tokens

            impl #generics_with_bound __wrmi_load_ts_macro::JsCast for #name #generics_without_bound {
                fn unchecked_from_js(val: __wrmi_load_ts_macro::JsValue) -> Self {
                    Self(val, ::core::marker::PhantomData)
                }
                fn unchecked_from_js_ref(val: &__wrmi_load_ts_macro::JsValue) -> &Self {
                    __wrmi_load_ts_macro::RefCast::ref_cast(val)
                }
            }
            impl #generics_with_bound __wrmi_load_ts_macro::UseInJsCode for #name #generics_without_bound {
                fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.serialize_to(buf)
                }
            }

            #(
                impl #generics_with_bound ::core::convert::AsRef<#extends> for #name #generics_without_bound {
                    fn as_ref(&self) -> &#extends {
                        __wrmi_load_ts_macro::JsCast::unchecked_from_js_ref(&self.0)
                    }
                }
                impl #generics_with_bound ::core::convert::Into<#extends> for #name #generics_without_bound {
                    fn into(self) -> #extends {
                        __wrmi_load_ts_macro::JsCast::unchecked_from_js(self.0)
                    }
                }
            )*

            impl #generics_with_bound std::ops::Deref for #name #generics_with_bound {
                type Target = #first_extend;
                fn deref(&self) -> &Self::Target {
                    self.as_ref()
                }
            }
        };

        let tokens = {
            fn make_member_code(
                ctx: &Context<'_>,
                interface_name: &'_ str,
                member: &Member<'_>,
                is_constructor: bool,
            ) -> Option<TokenStream> {
                match member {
                    Member::Method(method) => {
                        let method_name_str = match method.name {
                            crate::parser::method::MethodName::Nothing => "call_self",
                            crate::parser::method::MethodName::Constructor => "new",
                            crate::parser::method::MethodName::Iterator => return None,
                            crate::parser::method::MethodName::Name(name) => name,
                        };
                        let method_name_ident = Ident::new(method_name_str, Span::call_site());
                        let arg_names = method
                            .args
                            .iter()
                            .map(|arg| Ident::new(arg.name, Span::call_site()));
                        let arg_names_cloned = arg_names.clone();
                        let arg_types = method.args.iter().map(|arg| {
                            let arg_type = ctx.ts_type_to_rust(arg.ty.to_owned());
                            quote! {impl __wrmi_load_ts_macro::ToJs<#arg_type>}
                        });
                        let ret = ctx.ts_type_to_rust(method.ret.to_owned());
                        if is_constructor {
                            let function = &*format!("{}.{}", interface_name, method_name_str);
                            Some(quote! {
                                fn #method_name_ident (browser: &__wrmi_load_ts_macro::Browser, #(#arg_names: #arg_types,)*) -> #ret {
                                    __wrmi_load_ts_macro::JsCast::unchecked_from_js(browser.call_function(#function, [#(#arg_names_cloned,)*]))
                                }
                            })
                        } else {
                            Some(quote! {
                                fn #method_name_ident (&self, #(#arg_names: #arg_types,)*) -> #ret {
                                    __wrmi_load_ts_macro::JsCast::unchecked_from_js(self.0.js_call_method(method_name_str, [#(#arg_names_cloned,)*]))
                                }
                            })
                        }
                    }
                    _ => None
                    // Member::Field(_) => todo!(),
                    // Member::Getter(_) => todo!(),
                    // Member::Setter(_) => todo!(),
                }
            }

            let member_code =
                interface
                    .members
                    .iter()
                    .filter_map(|member| make_member_code(ctx, interface.name, &member.data, false))
                    .chain(decl_var.iter().flat_map(|member| {
                        make_member_code(ctx, interface.name, &member.data, true)
                    }));

            quote! {
                #tokens
                impl #generics_with_bound #name #generics_without_bound {
                    #(#member_code)*
                }
            }
        };

        tokens
    }

    let mut ctx = Context::default();
    let mut generated_code = Vec::new();
    let mut root_types_layer = HashMap::new();
    let mut root_decls_layer = HashMap::new();
    for item in dts {
        match &item.data {
            Item::Interface(interface) => {
                let interface = interface.clone();
                root_types_layer.insert(
                    interface.name,
                    (
                        interface.generics,
                        TsType::Interface {
                            members: interface.members,
                        },
                    ),
                );
            }
            Item::TypeAlias(alias) => {
                let alias = alias.clone();
                root_types_layer.insert(alias.name, (alias.generics, alias.ty));
            }
            Item::DeclareVar(DeclareVar {
                name,
                ty: TsType::Interface { members },
            }) => {
                root_decls_layer.insert(*name, &**members);
            }
            _ => {}
        }
    }
    ctx.types.push(root_types_layer);
    ctx.declares.push(root_decls_layer);
    for item in dts {
        match &item.data {
            Item::Interface(interface) => {
                generated_code.push(make_rust_type(
                    &ctx,
                    interface,
                    ctx.find_decl(interface.name).unwrap_or(&[]),
                ));
            }
            _ => {}
        }
    }
    quote! {
        #(#generated_code)*
    }
}
