use proc_macro2::{Ident, Span};

pub fn to_snake_case(camel_case: &str) -> String {
    let orig_len = camel_case.len();
    let mut out = String::with_capacity(orig_len + orig_len / 2);
    let mut was_lower = false;
    for ch in camel_case.chars() {
        let is_upper = ch.is_ascii_uppercase();
        let ch_lower = ch.to_ascii_lowercase();
        if was_lower && is_upper {
            out.push('_');
            out.push(ch_lower);
        } else {
            out.push(ch_lower);
        }
        was_lower = !is_upper;
    }
    out
}

pub fn new_ident_safe(name: &str) -> Ident {
    let name = match name {
        "self" => "self_value",
        "type" => "ty",
        n => n,
    };
    Ident::new_raw(name, Span::call_site())
}
