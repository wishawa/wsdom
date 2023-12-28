use crate::{
    js::nullable::{JsNullable, JsNullish},
    js_cast::JsCast,
    serialize::{RawCodeImmediate, ToJs, UseInJsCode},
};

pub struct NullImmediate;

impl UseInJsCode for NullImmediate {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        RawCodeImmediate("null").serialize_to(buf)
    }
}

impl ToJs<JsNullish> for NullImmediate {}
impl<T> ToJs<JsNullable<T>> for NullImmediate {}

impl<'a, T: UseInJsCode> UseInJsCode for Option<&'a T> {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Some(t) => t.serialize_to(buf),
            None => NullImmediate.serialize_to(buf),
        }
    }
}

impl<'a, T, U> ToJs<JsNullable<T>> for Option<&'a U>
where
    T: JsCast,
    U: UseInJsCode + ToJs<T>,
{
}
