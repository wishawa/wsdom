use std::marker::PhantomData;

use crate::{JsCast, ToJs, UseInJsCode};

use super::{object::JsObject, value::JsValue};

#[derive(Clone, ref_cast::RefCast)]
#[repr(transparent)]
pub struct JsArray<T>(JsValue, PhantomData<T>);

impl<T> JsCast for JsArray<T> {
    fn unchecked_from_js(val: JsValue) -> Self {
        Self(val, PhantomData)
    }
    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        ref_cast::RefCast::ref_cast(val)
    }
}
impl<T> UseInJsCode for JsArray<T> {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.serialize_to(buf)
    }
}
impl<T> AsRef<JsValue> for JsArray<T> {
    fn as_ref(&self) -> &JsValue {
        JsCast::unchecked_from_js_ref(&self.0)
    }
}
impl<T> Into<JsValue> for JsArray<T> {
    fn into(self) -> JsValue {
        JsCast::unchecked_from_js(self.0)
    }
}
impl<T> AsRef<JsObject> for JsArray<T> {
    fn as_ref(&self) -> &JsObject {
        JsCast::unchecked_from_js_ref(&self.0)
    }
}
impl<T> Into<JsObject> for JsArray<T> {
    fn into(self) -> JsObject {
        JsCast::unchecked_from_js(self.0)
    }
}
impl<T> std::ops::Deref for JsArray<T> {
    type Target = JsObject;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T> JsArray<T> {}

impl<'a, T, U> ToJs<JsArray<T>> for &'a [U]
where
    T: JsCast,
    U: serde::Serialize + ToJs<T>,
{
}

impl<T, U> ToJs<JsArray<T>> for JsArray<U>
where
    T: ToJs<U>,
    U: JsCast,
{
}
