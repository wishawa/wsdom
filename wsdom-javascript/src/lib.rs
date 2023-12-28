#![allow(non_snake_case)]

mod __wsdom_load_ts_macro {
    pub use ref_cast::RefCast;
    pub use wsdom_core::{js_types::*, Browser, JsCast, ToJs, UseInJsCode};
}

wsdom_macros::load_ts!("../typescript-defs/used/es5-handpicked.d.ts");

mod misc_impl;
