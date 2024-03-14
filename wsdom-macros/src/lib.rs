mod load_ts;
use syn::parse_macro_input;

#[proc_macro]
pub fn load_ts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let filename = parse_macro_input!(input as syn::LitStr);
    match load_ts::load_ts_macro(filename) {
        Ok(res) => res.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn load_custom_ts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let filename = parse_macro_input!(input as syn::LitStr);
    match load_ts::load_custom_ts_macro(filename) {
        Ok(res) => res.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
