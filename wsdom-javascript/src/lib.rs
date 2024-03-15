#![allow(non_snake_case)]

mod __wsdom_load_ts_macro {
    pub use ref_cast::RefCast;
    pub use wsdom_core::{
        for_macro::{RawCodeImmediate, UpcastWorkaround},
        js_types::*,
        Browser, JsCast, ToJs, UseInJsCode,
    };
    pub use wsdom_macros_decl::*;
}

wsdom_macros::load_ts!("es5-handpicked.d.ts");

mod misc_impl;
