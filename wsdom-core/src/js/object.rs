use crate::{js::value::JsValue, js_cast::JsCast, serialize::UseInJsCode};

include!("impl_basic.rs");

impl_basic!(JsObject);
