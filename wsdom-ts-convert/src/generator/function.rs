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
        let (arg_types, arg_names_sig, last_arg_variadic) = self.make_sig_args(&df.args);
        let arg_names_body = arg_names_sig.clone();
        let ret = self.convert_type(self.simplify_type(df.ret.to_owned()));
        let function_generics = self.make_sig_generics(&df.generics.args);
        let function = df.name;
        quote! {
            pub fn #function_name_ident #function_generics (browser: &__wsdom_load_ts_macro::Browser, #(#arg_names_sig: #arg_types,)*) -> #ret {
                __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                    browser.call_function(#function, [
                        #(  __wsdom_load_ts_macro::UpcastWorkaround::new(#arg_names_body).cast(), )*
                    ], #last_arg_variadic)
                )
            }
        }
    }
}
