mod interaction;
mod internal;
mod js;
mod js_cast;
mod link;
mod operations;
mod protocol;
mod retrieve;
mod serialize;

pub use link::{Browser, Error};

pub mod js_types {
    pub use super::js::{
        immediates::NullImmediate,
        nullable::{JsNullable, JsNullish},
        object::JsObject,
        primitives::*,
        value::JsValue,
    };
}
pub use interaction::callback;
pub use js_cast::JsCast;
pub use serialize::{ToJs, UseInJsCode};

pub mod for_macro {
    pub use super::internal::upcast_workaround::UpcastWorkaround;
    pub use super::link::BrowserInternal;
    pub use super::serialize::RawCodeImmediate;
}
