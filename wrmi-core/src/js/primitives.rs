use crate::{
    js::value::JsValue,
    js_cast::JsCast,
    retrieve::RetrieveFuture,
    serialize::{ToJs, UseInJsCode},
};

// number, boolean, and string

macro_rules! impl_primitive {
    ($name:ident) => {
        #[derive(Clone, ref_cast::RefCast)]
        #[repr(transparent)]
        pub struct $name(JsValue);

        impl AsRef<JsValue> for $name {
            fn as_ref(&self) -> &JsValue {
                JsCast::unchecked_from_js_ref(&self.0)
            }
        }

        impl std::ops::Deref for $name {
            type Target = JsValue;
            fn deref(&self) -> &JsValue {
                self.as_ref()
            }
        }

        impl Into<JsValue> for $name {
            fn into(self) -> JsValue {
                JsCast::unchecked_from_js(self.0)
            }
        }

        impl JsCast for $name {
            fn unchecked_from_js(val: JsValue) -> Self {
                Self(val)
            }
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                ref_cast::RefCast::ref_cast(val)
            }
        }

        impl UseInJsCode for $name {
            fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.serialize_to(buf)
            }
        }
    };
}

impl_primitive!(JsBoolean);
impl_primitive!(JsString);
impl_primitive!(JsNumber);
impl_primitive!(JsSymbol);

impl JsBoolean {
    pub fn retrieve(&self) -> RetrieveFuture<'_, bool> {
        self.0.retrieve_and_deserialize()
    }
}
impl ToJs<JsBoolean> for bool {}
impl JsString {
    pub fn retrieve(&self) -> RetrieveFuture<'_, String> {
        self.0.retrieve_and_deserialize()
    }
}
impl ToJs<JsString> for str {}
impl JsNumber {
    pub fn retrieve_float(&self) -> RetrieveFuture<'_, f64> {
        self.0.retrieve_and_deserialize()
    }
    pub fn retrieve_int(&self) -> RetrieveFuture<'_, i64> {
        self.0.retrieve_and_deserialize()
    }
}
impl ToJs<JsNumber> for i8 {}
impl ToJs<JsNumber> for i16 {}
impl ToJs<JsNumber> for i32 {}
impl ToJs<JsNumber> for i64 {}
impl ToJs<JsNumber> for isize {}
impl ToJs<JsNumber> for u8 {}
impl ToJs<JsNumber> for u16 {}
impl ToJs<JsNumber> for u32 {}
impl ToJs<JsNumber> for u64 {}
impl ToJs<JsNumber> for usize {}
impl ToJs<JsNumber> for f32 {}
impl ToJs<JsNumber> for f64 {}

// null and undefined

impl_primitive!(JsNull);
impl_primitive!(JsUndefined);

// impl<T: Into<JsValue> + JsCast> ToJs<Nullable<T>> for JsNull {}
