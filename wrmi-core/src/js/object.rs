use crate::{js::value::JsValue, js_cast::JsCast};

/// Represents a JsValue that is an object.
/// I don't actually know what constitute an object in JavaScript,
/// but I take it to be something with methods, fields, etc.
#[derive(Clone, ref_cast::RefCast)]
#[repr(transparent)]
pub struct JsObject(pub(crate) JsValue);

impl std::ops::Deref for JsObject {
    type Target = JsValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<JsValue> for JsObject {
    fn as_ref(&self) -> &JsValue {
        self
    }
}

impl Into<JsValue> for JsObject {
    fn into(self) -> JsValue {
        self.0
    }
}

impl JsCast for JsObject {
    fn unchecked_from_js(val: JsValue) -> Self {
        Self(val)
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        ref_cast::RefCast::ref_cast(val)
    }
}
