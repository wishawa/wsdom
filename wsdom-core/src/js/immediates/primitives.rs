use crate::{
    js::primitives::*,
    js::value::JsValue,
    serialize::{SerdeToJs, ToJs, UseInJsCode},
};

macro_rules! impl_use_in_js {
    ($name:ident) => {
        impl UseInJsCode for $name {
            fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                SerdeToJs(self).serialize_to(buf)
            }
        }
        impl ToJs<JsValue> for $name {}
    };
}

impl ToJs<JsBoolean> for bool {}
impl_use_in_js!(bool);
impl ToJs<JsString> for str {}
impl_use_in_js!(str);
impl<'a> UseInJsCode for &'a str {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        SerdeToJs(self).serialize_to(buf)
    }
}
impl<'a> ToJs<JsString> for &'a str {}
impl<'a> ToJs<JsValue> for &'a str {}

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

impl_use_in_js!(f64);
impl_use_in_js!(f32);
impl_use_in_js!(usize);
impl_use_in_js!(u64);
impl_use_in_js!(u32);
impl_use_in_js!(u16);
impl_use_in_js!(u8);
impl_use_in_js!(isize);
impl_use_in_js!(i64);
impl_use_in_js!(i32);
impl_use_in_js!(i16);
impl_use_in_js!(i8);
