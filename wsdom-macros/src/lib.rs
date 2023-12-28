// mod wasm_bindgen;
mod load_ts;
use syn::parse_macro_input;

// #[proc_macro_attribute]
// pub fn wasm_bindgen(
//     _attrs: proc_macro::TokenStream,
//     block: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     let block = parse_macro_input!(block as ItemForeignMod);
//     match wasm_bindgen::convert(block) {
//         Ok(res) => res.into(),
//         Err(e) => e.to_compile_error().into(),
//     }
// }

#[proc_macro]
pub fn load_ts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let filename = parse_macro_input!(input as syn::LitStr);
    match load_ts::load_ts_macro(filename) {
        Ok(res) => res.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
