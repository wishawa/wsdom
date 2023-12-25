pub use wrmi_macros::load_ts;

pub mod __wrmi_load_ts_macro {
    pub use ref_cast::RefCast;
    pub use wrmi_core::{js_types::*, Browser, JsCast, ToJs, UseInJsCode};
}

pub use wrmi_javascript::*;
wrmi_macros::load_ts!("../data-used/dom.generated.d.ts");
