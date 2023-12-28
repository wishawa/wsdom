use proc_macro2::TokenStream;
use quote::quote;

use crate::{generator::util::new_ident_safe, parser::ts_type::TsType};

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
}
