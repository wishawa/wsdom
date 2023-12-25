use proc_macro2::TokenStream;
use quote::quote;

use crate::parser::type_alias::TypeAlias;

use super::{util::new_ident_safe, Context};

impl<'a> Context<'a> {
    pub(super) fn make_type_alias(&self, ta: &TypeAlias<'a>) -> TokenStream {
        let name = new_ident_safe(ta.name);
        let ty = self.convert_type(ta.ty.to_owned());
        if ta.generics.args.is_empty() {
            quote! {
                type #name = #ty;
            }
        } else {
            let generic_args = ta.generics.args.iter().map(|arg| {
                let name = new_ident_safe(arg.name);
                let extends = arg.extends.clone().map(|b| {
                    let t = self.convert_type(b);
                    quote! {
                        : ::core::convert::AsRef<#t> + ::core::convert::Into<#t>
                    }
                });
                quote! {
                    #name #extends
                }
            });
            quote! {
                pub type #name < #(#generic_args,)* > = #ty ;
            }
        }
    }
}
