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
