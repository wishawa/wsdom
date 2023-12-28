use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::parser::{generic::GenericsDeclarationArg, method::MethodArg};

use super::{util::new_ident_safe, Context};

impl<'a> Context<'a> {
    pub(super) fn make_sig_args<'b>(
        &'b self,
        args: &'b [MethodArg<'a>],
    ) -> (
        impl Iterator<Item = TokenStream> + Clone + 'b,
        impl Iterator<Item = Ident> + Clone + 'b,
        bool,
    ) {
        let types = args.iter().map(move |arg| {
            let ty = self.simplify_type(arg.ty.to_owned());
            let (is_class, ty_tokens) = if arg.optional {
                (
                    false,
                    self.convert_type(self.unify_types(super::types::known_types::NULL, ty)),
                )
            } else {
                (self.classes.contains(ty.name), self.convert_type(ty))
            };
            if is_class {
                quote! {& #ty_tokens }
            } else {
                quote! {& dyn __wsdom_load_ts_macro::ToJs< #ty_tokens >}
            }
        });
        let names = args.iter().map(|arg| new_ident_safe(arg.name));
        let last_variadic = args.last().is_some_and(|a| a.variadic);
        (types, names, last_variadic)
    }
    pub(super) fn make_sig_generics<'b>(
        &'b self,
        args: &'b [GenericsDeclarationArg<'b>],
    ) -> Option<TokenStream> {
        let generics =args.iter().map(|gen| {
                    let name = new_ident_safe(gen.name);
                    let bounds = gen.extends.clone().map(|ty| self.convert_type(self.simplify_type(ty))).into_iter();
                    quote! {
                        #name: __wsdom_load_ts_macro::JsCast #(+ ::core::convert::AsRef<#bounds> + ::core::convert::Into<#bounds>)*
                    }
                });
        if args.is_empty() {
            None
        } else {
            Some(quote! {
                <#(#generics,)*>
            })
        }
    }
}
