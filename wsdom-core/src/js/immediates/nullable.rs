use crate::{
    js::nullable::{JsNullable, JsNullish},
    JsCast, RawCodeImmediate, ToJs, UseInJsCode,
};

pub struct NullImmediate;

impl UseInJsCode for NullImmediate {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        RawCodeImmediate("null").serialize_to(buf)
    }
}

impl ToJs<JsNullish> for NullImmediate {}
impl<T> ToJs<JsNullable<T>> for NullImmediate {}

impl<T: UseInJsCode> UseInJsCode for Option<T> {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Some(t) => t.serialize_to(buf),
            None => NullImmediate.serialize_to(buf),
        }
    }
}

impl<T, U> ToJs<JsNullable<T>> for Option<U>
where
    T: JsCast,
    U: UseInJsCode + ToJs<T>,
{
}
