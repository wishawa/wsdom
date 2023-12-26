pub use wrmi_macros::load_ts;

#[doc(hidden)]
pub mod __wrmi_load_ts_macro {
    pub use ref_cast::RefCast;
    pub use wrmi_core::{js_types::*, Browser, JsCast, ToJs, UseInJsCode};
}

pub use wrmi_core::{js_types, Browser, JsCast, ToJs};
pub use wrmi_dom as dom;
pub use wrmi_javascript as js;

#[cfg(test)]
mod tests {
    #[test]
    fn dummy() {
        use super::__wrmi_load_ts_macro;
        use wrmi_javascript::*;
        wrmi_macros::load_ts!("../data-test/dummy.d.ts");
    }
    #[test]
    fn history() {
        use super::__wrmi_load_ts_macro;
        use wrmi_javascript::*;
        wrmi_macros::load_ts!("../data-test/history.d.ts");
    }

    #[test]
    #[allow(non_snake_case, non_camel_case_types)]
    fn console() {
        use super::__wrmi_load_ts_macro;
        use wrmi_javascript::*;
        wrmi_macros::load_ts!("../data-test/console.d.ts");
    }

    #[test]
    fn math() {
        use super::__wrmi_load_ts_macro;
        use wrmi_javascript::*;
        wrmi_macros::load_ts!("../data-test/math.d.ts");
    }
    #[test]
    fn unify_nullable() {
        use super::__wrmi_load_ts_macro;
        use wrmi_javascript::*;
        wrmi_macros::load_ts!("../data-test/unify-null.d.ts");
    }

    #[test]
    fn generic() {
        use super::__wrmi_load_ts_macro;
        wrmi_macros::load_ts!("../data-test/generic.d.ts");
    }

    #[test]
    fn unify() {
        use super::__wrmi_load_ts_macro;
        wrmi_macros::load_ts!("../data-test/unify.d.ts");
    }
}
