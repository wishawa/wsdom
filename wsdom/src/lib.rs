/*!
# WSDOM

WSDOM is a roundtrip-free Rust → JavaScript *Remote Method Invocation* or *Distributed Objects* system.
It lets Rust code hold and manipulate JavaScript objects over the network.

For an overview of what this library does, see the [README on GitHub](https://github.com/wishawa/wsdom).

# Documentation
`wsdom::dom` is the module you will be working with most of the time.
This module contains stubs for the Web API (`HTMLElement`, `Canvas`, etc.).
The stubs were auto-generated so they don't have documentation attached to them,
but you can always look for the item with the corresponding name on [MDN](https://developer.mozilla.org/en-US/docs/Web/API).

The `wsdom::js` module contains stubs for the base JavaScript functionalities (`ArrayBuffer`, `RegExp`, etc.).
Note that this module is **incomplete**. I only auto-generated a small subset of the JS API
(because the TypeScript typings for these stuff are quite complicated so auto-translating them to Rust is hard).

The `js_types` module contains JavaScript primitives such as `number`, `string`, and `object`.
*/

#[doc(hidden)]
pub mod __wsdom_load_ts_macro {
    //! Private module for our macro. Don't use this.
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
pub use wsdom_core::immediates::*;
pub use wsdom_core::{js_types, Browser, JsCast, ToJs};
pub use wsdom_dom as dom;
pub use wsdom_javascript as js;
pub use wsdom_macros::load_custom_ts;

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
