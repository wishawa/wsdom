use crate::{
    js::value::JsValue, js_cast::JsCast, retrieve::RetrieveFuture, serialize::UseInJsCode,
};

include!("impl_basic.rs");

impl_basic!(JsBoolean);
impl_basic!(JsString);
impl_basic!(JsNumber);
impl_basic!(JsSymbol);

impl JsBoolean {
    pub fn retrieve(&self) -> RetrieveFuture<'_, bool> {
        self.0.retrieve_and_deserialize()
    }
}

impl JsString {
    pub fn retrieve(&self) -> RetrieveFuture<'_, String> {
        self.0.retrieve_and_deserialize()
    }
}

impl JsNumber {
    pub fn retrieve_float(&self) -> RetrieveFuture<'_, f64> {
        self.0.retrieve_and_deserialize()
    }
    pub fn retrieve_int(&self) -> RetrieveFuture<'_, i64> {
        self.0.retrieve_and_deserialize()
    }
}
