use crate::{JsCast, UseInJsCode};

use super::{object::JsObject, value::JsValue};

include!("impl_basic.rs");

impl_basic!(JsArray; T; JsObject);
impl_basic!(@one_conversion; JsArray; T; JsValue);
