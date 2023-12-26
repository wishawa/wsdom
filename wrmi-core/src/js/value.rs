use crate::js_cast::JsCast;
use crate::link::Browser;
use crate::protocol::{DEL, GET, SET};
use std::fmt::Write;

/// Represents a value that exists on the JavaScript side.
/// Value can be anything - number, string, object, undefined, null, ...
pub struct JsValue {
    pub(crate) id: u64,
    pub(crate) browser: Browser,
}

impl Drop for JsValue {
    fn drop(&mut self) {
        let self_id = self.id;
        write!(
            self.browser.0.lock().unwrap().raw_commands_buf(),
            "{DEL}({self_id});\n",
        )
        .unwrap();
    }
}

impl Clone for JsValue {
    fn clone(&self) -> Self {
        let self_id = self.id;
        let out_id = {
            let mut link = self.browser.0.lock().unwrap();
            let out_id = link.get_new_id();
            write!(
                link.raw_commands_buf(),
                "{SET}({out_id},{GET}({self_id}));\n"
            )
            .unwrap();
            out_id
        };
        Self {
            id: out_id,
            browser: self.browser.clone(),
        }
    }
}

impl JsValue {
    // const MAX_ID: u64 = (1 << 53) - 1;
    pub fn browser(&self) -> &Browser {
        &self.browser
    }
}

impl AsRef<JsValue> for JsValue {
    fn as_ref(&self) -> &JsValue {
        self
    }
}

impl JsCast for JsValue {
    fn unchecked_from_js(val: JsValue) -> Self {
        val
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        val
    }
}
