use crate::{
    generator::util::{new_ident_safe, to_snake_case},
    parser::declare_function::DeclareFunction,
};
use proc_macro2::TokenStream;
use quote::quote;

use super::Context;

impl<'a> Context<'a> {
    pub(super) fn make_function(&self, df: &DeclareFunction<'a>) -> TokenStream {
        let function_name_ident = new_ident_safe(&to_snake_case(df.name));
        let arg_names_sig = df.args.iter().map(|arg| new_ident_safe(arg.name));
        let arg_names_body = arg_names_sig.clone();
        let arg_types = df.args.iter().map(|arg| {
            let arg_type = self.convert_type(self.simplify_type(arg.ty.to_owned()));
            quote! {&impl __wsdom_load_ts_macro::ToJs<#arg_type>}
        });
        let last_arg_variadic = df.args.iter().any(|arg| arg.variadic);
        let ret = self.convert_type(self.simplify_type(df.ret.to_owned()));
        let function_generics = df.generics.args.iter().map(|gen| {
            let name = new_ident_safe(gen.name);
            let extends = gen
                .extends
                .clone()
                .map(|ty| self.convert_type(self.simplify_type(ty)))
                .into_iter();
            quote! {
                #name: __wsdom_load_ts_macro::JsCast #(+ ::core::convert::AsRef<#extends> + ::core::convert::Into<#extends>)*
            }
        });
        let function_generics = if df.generics.args.is_empty() {
            None
        } else {
            Some(quote! {
                <#(#function_generics,)*>
            })
        };
        let function = df.name;
        quote! {
            pub fn #function_name_ident #function_generics (browser: &__wsdom_load_ts_macro::Browser, #(#arg_names_sig: #arg_types,)*) -> #ret {
                __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                    browser.call_function(#function, [
                        #( #arg_names_body as &dyn __wsdom_load_ts_macro::UseInJsCode,)*
                    ], #last_arg_variadic)
                )
            }
        }
    }
}
