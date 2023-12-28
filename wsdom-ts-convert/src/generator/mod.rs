mod alias;
mod class;
mod function;
mod signature;
mod types;
mod util;
mod var;

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::util::iter_dedupe_consecutive,
    parser::{
        comment::WithComment,
        declare_var::DeclareVar,
        generic::GenericsDeclaration,
        interface::Interface,
        item::Item,
        member::Member,
        method::{Method, MethodName},
        ts_type::{NamedType, TsType},
    },
};

#[derive(Default)]
struct Context<'a> {
    interfaces: HashMap<&'a str, Interface<'a>>,
    classes: HashSet<&'a str>,
    inhr_graph: HashMap<&'a str, (GenericsDeclaration<'a>, Vec<NamedType<'a>>)>,
}

pub(crate) fn generate_all<'a>(dts: &[WithComment<'a, Item<'a>>]) -> TokenStream {
    let mut generated_code = Vec::<TokenStream>::new();
    let mut interfaces = HashMap::new();
    let mut inhr_graph = HashMap::new();
    for item in dts {
        match &item.data {
            Item::Interface(interface) => {
                let interface = interface.clone();
                let (_, extends) = inhr_graph
                    .entry(interface.name)
                    .or_insert_with(|| (interface.generics.clone(), Vec::new()));
                for extend in &interface.extends {
                    match extend {
                        TsType::Named { ty } => {
                            extends.push(ty.to_owned());
                        }
                        _ => {}
                    }
                }
                interfaces.insert(interface.name, interface);
            }
            _ => {}
        }
    }
    let mut ctx = Context {
        interfaces,
        classes: HashSet::new(),
        inhr_graph,
    };

    let declare_vars = dts.iter().filter_map(|item| match &item.data {
        Item::DeclareVar(dv) => Some(dv),
        _ => None,
    });
    let declare_classes = declare_vars.clone().filter(|dv| starts_with_cap(dv.name));
    let declare_global_vars = declare_vars.filter(|dv| !starts_with_cap(dv.name));
    ctx.classes
        .extend(declare_classes.clone().map(|dv| dv.name));
    for DeclareVar { name, ty } in declare_classes {
        let Some((decl_members, direct_decl)) = (match ty {
            TsType::Named {
                ty: NamedType { name, .. },
            } => ctx
                .interfaces
                .get(name)
                .map(|interface| (&*interface.members, false)),
            TsType::Interface { members } => Some((&**members, true)),
            _ => None,
        }) else {
            continue;
        };

        let constructor_ty = decl_members.iter().find_map(|member| match &member.data {
            Member::Method(Method {
                name: MethodName::Constructor,
                ret:
                    TsType::Named {
                        ty: NamedType { name, .. },
                    },
                ..
            }) => Some(name),
            _ => None,
        });
        if constructor_ty.is_some_and(|t| t != name) {
            continue;
        }
        let iface = constructor_ty
            .and_then(|s| ctx.interfaces.get(s))
            .or_else(|| direct_decl.then(|| ctx.interfaces.get(name)).flatten())
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
    for DeclareVar { name, ty } in declare_global_vars {
        generated_code.push(ctx.make_global_var_getter(name, ty));
    }
    for ta in dts.iter().filter_map(|item| match &item.data {
        Item::TypeAlias(ta) => Some(ta),
        _ => None,
    }) {
        generated_code.push(ctx.make_type_alias(ta));
    }
    for df in iter_dedupe_consecutive(
        dts.iter().filter_map(|item| match &item.data {
            Item::DeclareFunction(df) => Some(df),
            _ => None,
        }),
        |df| Some(df.name),
    ) {
        generated_code.push(ctx.make_function(df));
    }

    for iface in dts.iter().filter_map(|item| match &item.data {
        Item::Interface(iface) if !ctx.classes.contains(iface.name) => Some(iface),
        _ => None,
    }) {
        generated_code.push(ctx.make_class(iface, &[]));
    }
    quote! {
        #(#generated_code)*
    }
}

fn starts_with_cap(s: &str) -> bool {
    s.chars()
        .next()
        .as_ref()
        .is_some_and(char::is_ascii_uppercase)
}
