use parking_lot::ReentrantMutex;

use crate::connection::WrmiLink;
use crate::js_cast::JsCast;
use crate::protocol::{DEL, GET, SET};
use std::cell::RefCell;
use std::{fmt::Write, sync::Arc};

pub struct JsValue {
    pub(crate) id: u64,
    pub(crate) connection: Arc<ReentrantMutex<RefCell<WrmiLink>>>,
}

impl Drop for JsValue {
    fn drop(&mut self) {
        let self_id = self.id;
        write!(
            self.connection.lock().borrow_mut().raw_commands_buf(),
            "{DEL}({self_id});\n",
        )
        .unwrap();
    }
}

impl Clone for JsValue {
    fn clone(&self) -> Self {
        let self_id = self.id;
        let remote = self.connection.lock();
        let mut remote = remote.borrow_mut();
        let out_id = remote.get_new_id();
        write!(
            remote.raw_commands_buf(),
            "{SET}({out_id},{GET}({self_id}));\n"
        )
        .unwrap();
        Self {
            id: out_id,
            connection: Arc::clone(&self.connection),
        }
    }
}

impl JsValue {
    // const MAX_ID: u64 = (1 << 53) - 1;
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
