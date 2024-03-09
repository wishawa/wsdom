mod interaction;
mod js;
mod js_cast;
mod link;
mod operations;
mod protocol;
mod retrieve;
mod serialize;
mod upcast_workaround;

pub use link::Browser;
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

pub mod advanced {
    pub use super::link::BrowserInternal;
    pub use super::serialize::RawCodeImmediate;
    pub use super::upcast_workaround::UpcastWorkaround;
}
