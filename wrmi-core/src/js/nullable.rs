use std::marker::PhantomData;

use crate::{js::value::JsValue, js_cast::JsCast, serialize::UseInJsCode};

#[derive(Clone, ref_cast::RefCast)]
#[repr(transparent)]
pub struct Nullable<T>(JsValue, PhantomData<T>);

impl<T: AsRef<JsValue>> AsRef<JsValue> for Nullable<T> {
    fn as_ref(&self) -> &JsValue {
        self.0.as_ref()
    }
}

impl<T: Into<JsValue>> Into<JsValue> for Nullable<T> {
    fn into(self) -> JsValue {
        self.0.into()
    }
}

impl<T: JsCast> JsCast for Nullable<T> {
    fn unchecked_from_js(val: JsValue) -> Self {
        Self(val, PhantomData)
    }
    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        ref_cast::RefCast::ref_cast(val)
    }
}

impl<T: UseInJsCode> UseInJsCode for Nullable<T> {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.serialize_to(buf)
    }
}
