use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::util::new_ident_safe,
    parser::{method::MethodArg, ts_type::TsType},
};

use super::Context;

impl<'a> Context<'a> {
    pub(super) fn make_global_var_getter(&self, name: &str, ty: &TsType<'a>) -> TokenStream {
        let function_name = new_ident_safe(name);
        let ty = self.convert_type(self.simplify_type(ty.to_owned()));
        quote! {
            pub fn #function_name (browser: &__wsdom_load_ts_macro::Browser) -> #ty {
                __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                    browser.value_from_raw_code(core::format_args!(#name))
                )
            }
        }
    }
    pub(super) fn make_custom_constructor(
        &self,
        class_name: &str,
        args: &[MethodArg<'a>],
        ret_ty: &TsType<'a>,
    ) -> TokenStream {
        let (arg_types, arg_names_sig, variadic) = self.make_sig_args(args);
        let arg_names_body = arg_names_sig.clone();
        let ret_ty = self.convert_type(self.simplify_type(ret_ty.to_owned()));
        let name = new_ident_safe(class_name);
        quote! {
            pub fn #name (browser: &__wsdom_load_ts_macro::Browser, #(#arg_names_sig : #arg_types,)*) -> #ret_ty {
                __wsdom_load_ts_macro::JsCast::unchecked_from_js(
                    browser.call_constructor(
                        #class_name,
                        [
                            #(  __wsdom_load_ts_macro::UpcastWorkaround::new( #arg_names_body ).cast(), )*
                        ],
                        #variadic
                    )
                )
            }
        }
    }
}
