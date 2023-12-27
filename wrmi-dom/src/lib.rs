#![allow(non_snake_case)]

mod __wrmi_load_ts_macro {
    pub use ref_cast::RefCast;
    pub use wrmi_core::{js_types::*, Browser, JsCast, ToJs, UseInJsCode};
}

use wrmi_javascript::*;
wrmi_macros::load_ts!("../data-used/dom.generated.d.ts");
