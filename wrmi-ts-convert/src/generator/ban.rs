static BANNED_DECLARE_VARS: &[&str] = &["NodeFilter"];

pub(super) fn check_declare_var(name: &str) -> bool {
    BANNED_DECLARE_VARS.binary_search(&name).is_err()
}
