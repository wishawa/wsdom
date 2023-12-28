use crate::{js::value::JsValue, js_cast::JsCast, serialize::UseInJsCode};

include!("impl_basic.rs");

impl_basic!(JsNullish);

impl_basic!(JsNullable; T; JsValue);

impl JsNullish {
    pub fn into_nullable_ref<T>(&self) -> &JsNullable<T> {
        JsCast::unchecked_ref(self)
    }
    pub fn into_nullable<T>(self) -> JsNullable<T> {
        JsCast::unchecked_into(self)
    }
}

impl<T> JsNullable<T>
where
    T: JsCast,
{
    pub fn from_nonnull(value: T) -> Self {
        JsCast::unchecked_into(value)
    }
    pub fn from_nonnull_ref(value: &T) -> &Self {
        JsCast::unchecked_ref(value)
    }
    pub fn unwrap(self) -> T {
        JsCast::unchecked_into(self)
    }
    pub fn unwrap_ref(&self) -> &T {
        JsCast::unchecked_ref(self)
    }
}
