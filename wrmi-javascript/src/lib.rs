#![allow(non_snake_case)]

pub use wrmi_macros::load_ts;

mod __wrmi_load_ts_macro {
    pub use super::Array;
    pub use ref_cast::RefCast;
    pub use wrmi_core::{js_types::*, Browser, JsCast, ToJs, UseInJsCode};
}

wrmi_macros::load_ts!("../data-used/es5-handpicked.d.ts");

mod misc_impl;
