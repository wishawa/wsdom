use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::parser::ts_type::TsType;

use super::Context;

impl<'a> Context<'a> {
    pub(super) fn make_global_var_getter(&self, name: &str, ty: &TsType<'a>) -> TokenStream {
        let function_name = Ident::new(name, Span::call_site());
        let ty = self.convert_type(ty.to_owned());
        quote! {
            pub fn #function_name (browser: &__wrmi_load_ts_macro::Browser) -> #ty {
                __wrmi_load_ts_macro::JsCast::unchecked_from_js(
                    browser.value_from_raw_code(core::format_args!(#name))
                )
            }
        }
    }
}
