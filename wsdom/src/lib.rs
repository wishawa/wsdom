pub use wsdom_macros::load_ts;

#[doc(hidden)]
pub mod __wsdom_load_ts_macro {
    pub use ref_cast::RefCast;
    pub use wsdom_core::{
        for_macro::{RawCodeImmediate, UpcastWorkaround},
        js_types::*,
        Browser, JsCast, ToJs, UseInJsCode,
    };
    pub use wsdom_javascript::Array;
    pub use wsdom_macros_decl::*;
}

pub use wsdom_core::callback;
pub use wsdom_core::{js_types, Browser, JsCast, ToJs};
pub use wsdom_dom as dom;
pub use wsdom_javascript as js;

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    #[test]
    fn dummy() {
        use super::__wsdom_load_ts_macro;
        wsdom_macros::load_ts!("../typescript-defs/test/dummy.d.ts");
    }
    #[test]
    fn history() {
        use super::__wsdom_load_ts_macro;
        wsdom_macros::load_ts!("../typescript-defs/test/history.d.ts");
    }

    #[test]
    #[allow(non_snake_case, non_camel_case_types)]
    fn console() {
        use super::__wsdom_load_ts_macro;
        use wsdom_javascript::*;
        wsdom_macros::load_ts!("../typescript-defs/test/console.d.ts");
    }

    #[test]
    fn math() {
        use super::__wsdom_load_ts_macro;
        use wsdom_javascript::*;
        wsdom_macros::load_ts!("../typescript-defs/test/math.d.ts");
    }
    #[test]
    fn unify_nullable() {
        use super::__wsdom_load_ts_macro;
        wsdom_macros::load_ts!("../typescript-defs/test/unify-null.d.ts");
    }

    #[test]
    fn generic() {
        use super::__wsdom_load_ts_macro;
        wsdom_macros::load_ts!("../typescript-defs/test/generic.d.ts");
    }

    #[test]
    fn unify() {
        use super::__wsdom_load_ts_macro;
        wsdom_macros::load_ts!("../typescript-defs/test/unify.d.ts");
    }
}
