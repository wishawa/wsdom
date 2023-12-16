use std::fmt::Write;

use crate::{
    js::{object::JsObject, value::JsValue},
    link::Browser,
    protocol::{GET, SET},
    retrieve::RetrieveFuture,
    serialize::{UseInJsCode, UseInJsCodeWriter},
};

impl Browser {
    pub fn call_function<'a>(
        &'a self,
        function: &'a str,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
    ) -> JsValue {
        self.call_function_inner(&format_args!("{}", function), args)
    }

    fn call_function_inner<'a>(
        &'a self,
        function: &std::fmt::Arguments<'_>,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
    ) -> JsValue {
        let id = {
            let link = self.0.lock();
            let mut link = link.borrow_mut();
            let out_id = link.get_new_id();
            write!(link.raw_commands_buf(), "{SET}({out_id},{function}(").unwrap();
            for arg in args.into_iter() {
                let arg = UseInJsCodeWriter(arg);
                if write!(link.raw_commands_buf(), "{arg},").is_err() {
                    link.kill(Box::new(CommandSerializeFailed));
                }
            }
            write!(link.raw_commands_buf(), "));\n").unwrap();
            out_id
        };
        JsValue {
            id,
            browser: self.clone(),
        }
    }

    pub(crate) fn new_value<'a>(&'a self, code: std::fmt::Arguments<'a>) -> JsValue {
        let link = self.0.lock();
        let mut link = link.borrow_mut();
        let out_id = link.get_new_id();
        write!(link.raw_commands_buf(), "{SET}({out_id},{code})").unwrap();
        JsValue {
            id: out_id,
            browser: self.to_owned(),
        }
    }
}

impl JsValue {
    pub(crate) fn retrieve_and_deserialize<U: serde::de::DeserializeOwned>(
        &self,
    ) -> RetrieveFuture<'_, U> {
        RetrieveFuture::new(self.id, &self.browser.0)
    }
    pub fn retrieve_json(&self) -> RetrieveFuture<'_, serde_json::Value> {
        self.retrieve_and_deserialize()
    }
}
impl JsObject {
    pub fn js_get_field(&self, property: &dyn UseInJsCode) -> JsValue {
        let browser = self.browser.clone();
        let id = {
            let link = browser.0.lock();
            let mut link = link.borrow_mut();
            let out_id = link.get_new_id();
            let self_id = self.id;
            let property = UseInJsCodeWriter(property);
            if write!(
                link.raw_commands_buf(),
                "{SET}({out_id},{GET}({self_id}).{property});\n"
            )
            .is_err()
            {
                link.kill(Box::new(CommandSerializeFailed));
            }
            out_id
        };
        JsValue { id, browser }
    }
    pub fn js_set_field(&self, property: &dyn UseInJsCode, value: &dyn UseInJsCode) {
        let self_id = self.id;
        let link = self.browser.0.lock();
        let mut link = link.borrow_mut();
        let (property, value) = (UseInJsCodeWriter(property), UseInJsCodeWriter(value));
        if write!(
            link.raw_commands_buf(),
            "{GET}({self_id})[{property}]={value};\n"
        )
        .is_err()
        {
            link.kill(Box::new(CommandSerializeFailed));
        }
    }
    pub fn js_call_method<'a>(
        &'a self,
        method_name: &'a str,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
    ) -> JsValue {
        let self_id = self.id;
        self.browser
            .call_function_inner(&format_args!("{GET}({self_id}).{method_name}"), args)
    }
    pub fn js_call_self<'a>(
        &'a self,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
    ) -> JsValue {
        let self_id = self.id;
        self.browser
            .call_function_inner(&format_args!("({GET}({self_id}))"), args)
    }
}

struct CommandSerializeFailed;

impl std::fmt::Display for CommandSerializeFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::fmt::Debug for CommandSerializeFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandSerializeFailed").finish()
    }
}

impl std::error::Error for CommandSerializeFailed {}
