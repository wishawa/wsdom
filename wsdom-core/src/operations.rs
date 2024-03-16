use std::fmt::Write;

use crate::{
    js::{object::JsObject, value::JsValue},
    js_cast::JsCast,
    link::{Browser, Error},
    protocol::{GET, SET},
    retrieve::RetrieveFuture,
    serialize::{ToJs, UseInJsCode, UseInJsCodeWriter},
};

impl Browser {
    /// Call a standalone JavaScript function.
    ///
    /// ```rust
    /// # use wsdom_core::Browser;
    /// fn example(browser: Browser) {
    ///     let _return_value = browser.call_function(
    ///         "alert",
    ///         [&"hello world" as &_],
    ///         false
    ///     );
    /// }
    /// ```
    ///
    /// This method is "low-level" and you shouldn't need to use it.
    /// Instead, use the `wsdom` crate which provides mostly type-safe wrappers to the Web API.
    ///
    /// If you still want to use `call_function`,
    /// be aware that the first argument (`function_name`) is NOT escaped.
    /// Do NOT allow user-supplied function name.
    pub fn call_function<'a>(
        &'a self,
        function_name: &'a str,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
        last_arg_variadic: bool,
    ) -> JsValue {
        self.call_function_inner(&format_args!("{}", function_name), args, last_arg_variadic)
    }

    /// Call constructor for a class.
    ///
    /// ```rust
    /// # use wsdom_core::Browser;
    /// fn example(browser: Browser) {
    ///     let _regexp_object = browser.call_constructor(
    ///         "RegExp",
    ///         [&"hello" as &_],
    ///         false
    ///     );
    /// }
    /// ```
    ///
    /// This method is "low-level" and you shouldn't need to use it.
    /// Instead, use the `wsdom` crate which provides mostly type-safe wrappers to the Web API.
    ///
    /// If you still want to use `call_constructor`,
    /// be aware that the first argument (`class_name`) is NOT escaped.
    /// Do NOT allow user-supplied class name.
    pub fn call_constructor<'a>(
        &'a self,
        class_name: &'a str,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
        last_arg_variadic: bool,
    ) -> JsValue {
        self.call_function_inner(&format_args!("new {}", class_name), args, last_arg_variadic)
    }

    fn call_function_inner<'a>(
        &'a self,
        function: &std::fmt::Arguments<'_>,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
        last_arg_variadic: bool,
    ) -> JsValue {
        let id = {
            let mut link = self.0.lock().unwrap();
            let out_id = link.get_new_id();
            write!(link.raw_commands_buf(), "{SET}({out_id},{function}(").unwrap();
            let mut iter = args.into_iter().peekable();
            while let Some(arg) = iter.next() {
                let arg = UseInJsCodeWriter(arg);
                let res = if last_arg_variadic && iter.peek().is_none() {
                    write!(link.raw_commands_buf(), "...{arg},")
                } else {
                    write!(link.raw_commands_buf(), "{arg},")
                };
                if let Err(e) = res {
                    link.kill(Error::CommandSerialize(e));
                }
            }
            write!(link.raw_commands_buf(), "));\n").unwrap();
            link.wake_outgoing();
            out_id
        };
        JsValue {
            id,
            browser: self.clone(),
        }
    }

    /// Get a field in an object.
    ///
    /// This returns the value of `base_obj[property]`.
    pub fn get_field(&self, base_obj: &dyn UseInJsCode, property: &dyn UseInJsCode) -> JsValue {
        let browser = self.clone();
        let id = {
            let mut link = browser.0.lock().unwrap();
            let out_id = link.get_new_id();
            let base_obj = UseInJsCodeWriter(base_obj);
            let property = UseInJsCodeWriter(property);
            if let Err(e) = writeln!(
                link.raw_commands_buf(),
                "{SET}({out_id},({base_obj})[{property}]);"
            ) {
                link.kill(Error::CommandSerialize(e));
            }
            link.wake_outgoing_lazy();
            out_id
        };
        JsValue { id, browser }
    }

    /// Set a field in an object.
    ///
    /// This executes the JavaScript code `base_obj[property]=value;`
    pub fn set_field(
        &self,
        base_obj: &dyn UseInJsCode,
        property: &dyn UseInJsCode,
        value: &dyn UseInJsCode,
    ) {
        let mut link = self.0.lock().unwrap();
        let (base_obj, property, value) = (
            UseInJsCodeWriter(base_obj),
            UseInJsCodeWriter(property),
            UseInJsCodeWriter(value),
        );
        if let Err(e) = writeln!(link.raw_commands_buf(), "({base_obj})[{property}]={value};") {
            link.kill(Error::CommandSerialize(e));
        }
        link.wake_outgoing();
    }

    /// Create a new value on the JavaScript side from a [ToJs] type.
    pub fn new_value<'a, T: JsCast>(&'a self, value: &'a dyn ToJs<T>) -> T {
        let val = self.value_from_raw_code(format_args!("{}", UseInJsCodeWriter(value)));
        JsCast::unchecked_from_js(val)
    }

    /// Executes arbitrary JavaScript code.
    ///
    /// Don't use this unless you really have to.
    pub fn run_raw_code<'a>(&'a self, code: std::fmt::Arguments<'a>) {
        let mut link = self.0.lock().unwrap();
        if let Err(e) = writeln!(link.raw_commands_buf(), "{{ {code} }}") {
            link.kill(Error::CommandSerialize(e));
        }
        link.wake_outgoing();
    }

    /// Executes arbitrary JavaScript expression and return the result.
    ///
    /// Don't use this unless you really have to.
    pub fn value_from_raw_code<'a>(&'a self, code: std::fmt::Arguments<'a>) -> JsValue {
        let mut link = self.0.lock().unwrap();
        let out_id = link.get_new_id();
        writeln!(link.raw_commands_buf(), "{SET}({out_id},{code});").unwrap();
        link.wake_outgoing();
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
    /// Retrive this value from the JS side to the Rust side.
    /// Returns Future whose output is a [serde_json::Value].
    ///
    /// ```rust
    /// # use wsdom::Browser;
    /// # use wsdom::dom::HTMLInputElement;
    /// async fn example(input: &HTMLInputElement) {
    ///     let _val = input.get_value().retrieve_json().await;
    /// }
    /// ```
    pub fn retrieve_json(&self) -> RetrieveFuture<'_, serde_json::Value> {
        self.retrieve_and_deserialize()
    }
}
impl JsObject {
    /// Get a field value of in this object.
    ///
    /// WSDOM provides built-in getters so you should use that instead when possible.
    ///
    /// Use `js_get_field` only when needed
    ///
    /// ```rust
    /// # use wsdom_core::Browser;
    /// # use wsdom_core::js_types::*;
    /// fn example(browser: Browser) {
    ///     // you can get `window["location"]["href"]` like this
    ///     let href: JsValue = wsdom::dom::location(&browser).js_get_field(&"href");
    ///
    ///     // but you should use built-in getters instead
    ///     let href: JsString = wsdom::dom::location(&browser).get_href();
    /// }
    /// ```
    pub fn js_get_field(&self, property: &dyn UseInJsCode) -> JsValue {
        let browser = self.browser.clone();
        let id = {
            let mut link = browser.0.lock().unwrap();
            let out_id = link.get_new_id();
            let self_id = self.id;
            let property = UseInJsCodeWriter(property);
            if let Err(e) = writeln!(
                link.raw_commands_buf(),
                "{SET}({out_id},{GET}({self_id})[{property}]);"
            ) {
                link.kill(Error::CommandSerialize(e));
            }
            link.wake_outgoing_lazy();
            out_id
        };
        JsValue { id, browser }
    }
    /// Set a field value of in this object.
    ///
    /// WSDOM provides built-in setters so you should use that instead when possible.
    ///
    /// Use `js_set_field` only when needed
    ///
    /// ```rust
    /// # use wsdom_core::Browser;
    /// # use wsdom_core::js_types::*;
    /// fn example(browser: Browser) {
    ///     // you can set `window["location"]["href"]` like this
    ///     wsdom::dom::location(&browser).js_set_field(&"href", &"https://example.com/");
    ///
    ///     // but you should use built-in setters instead
    ///     wsdom::dom::location(&browser).set_href(&"https://example.com");
    /// }
    /// ```
    pub fn js_set_field(&self, property: &dyn UseInJsCode, value: &dyn UseInJsCode) {
        let self_id = self.id;
        let mut link = self.browser.0.lock().unwrap();
        let (property, value) = (UseInJsCodeWriter(property), UseInJsCodeWriter(value));
        if let Err(e) = writeln!(
            link.raw_commands_buf(),
            "{GET}({self_id})[{property}]={value};"
        ) {
            link.kill(Error::CommandSerialize(e));
        }
        link.wake_outgoing();
    }

    /// Call a method on this object.
    ///
    /// Most types in WSDOM already come with safe Rust wrappers for their methods, so you should use those instead.
    ///
    /// ```rust
    /// # use wsdom_core::Browser;
    /// fn example(browser: &Browser) {
    ///     let console = wsdom::dom::console(browser);
    ///     // you can call console.log like this
    ///     console.js_call_method("log", [&"hello" as &_], false);
    ///     
    ///     // but the better way is to use
    ///     wsdom::dom::console(&browser).log(&[&"Hello" as &_]);
    /// }
    /// ```
    ///
    /// Be aware that the first argument (`method_name`) is NOT escaped.
    ///
    /// Set `last_arg_variadic` to `true` if you want to "spread" the last argument as `obj.method(arg1, arg2, ...arg3)`.
    pub fn js_call_method<'a>(
        &'a self,
        method_name: &'a str,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
        last_arg_variadic: bool,
    ) -> JsValue {
        let self_id = self.id;
        self.browser.call_function_inner(
            &format_args!("{GET}({self_id}).{method_name}"),
            args,
            last_arg_variadic,
        )
    }
    /// Call this object: `obj()`.
    ///
    /// Most types in WSDOM already come with safe Rust wrappers for their methods, so you should use those instead.
    pub fn js_call_self<'a>(
        &'a self,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
        last_arg_variadic: bool,
    ) -> JsValue {
        let self_id = self.id;
        self.browser.call_function_inner(
            &format_args!("({GET}({self_id}))"),
            args,
            last_arg_variadic,
        )
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
