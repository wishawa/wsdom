use std::{cell::RefCell, fmt::Write, sync::Arc};

use parking_lot::ReentrantMutex;

use crate::{
    connection::WrmiLink,
    js::value::JsValue,
    protocol::{GET, SET},
    retrieve::RetrieveFuture,
    serialize::{UseInJsCode, UseInJsCodeWriter},
};

pub fn call_function<'a>(
    remote_arc: &'a Arc<ReentrantMutex<RefCell<WrmiLink>>>,
    function: &'a str,
    args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
) -> JsValue {
    call_function_inner(remote_arc, &format_args!("{}", function), args)
}

pub fn new_value<'a>(
    remote_arc: &'a Arc<ReentrantMutex<RefCell<WrmiLink>>>,
    code: std::fmt::Arguments<'a>,
) -> JsValue {
    let remote = remote_arc.lock();
    let mut remote = remote.borrow_mut();
    let out_id = remote.get_new_id();
    write!(remote.raw_commands_buf(), "{SET}({out_id},{code})").unwrap();
    JsValue {
        id: out_id,
        connection: remote_arc.clone(),
    }
}

impl JsValue {
    pub(crate) fn retrieve<U: serde::de::DeserializeOwned>(&self) -> RetrieveFuture<'_, U> {
        RetrieveFuture::new(self.id, &self.connection)
    }
    pub fn retrieve_json(&self) -> RetrieveFuture<'_, serde_json::Value> {
        self.retrieve()
    }
    pub fn js_get_property(&self, property: &dyn UseInJsCode) -> JsValue {
        let remote = Arc::clone(&self.connection);
        let id = {
            let remote = remote.lock();
            let mut remote = remote.borrow_mut();
            let out_id = remote.get_new_id();
            let self_id = self.id;
            let property = UseInJsCodeWriter(property);
            if write!(
                remote.raw_commands_buf(),
                "{SET}({out_id},{GET}({self_id}).{property});\n"
            )
            .is_err()
            {
                remote.kill(Box::new(CommandSerializeFailed));
            }
            out_id
        };
        JsValue {
            id,
            connection: remote,
        }
    }
    pub fn js_set_property(&self, property: &dyn UseInJsCode, value: &dyn UseInJsCode) {
        let self_id = self.id;
        let remote = self.connection.lock();
        let mut remote = remote.borrow_mut();
        let (property, value) = (UseInJsCodeWriter(property), UseInJsCodeWriter(value));
        if write!(
            remote.raw_commands_buf(),
            "{GET}({self_id})[{property}]={value};\n"
        )
        .is_err()
        {
            remote.kill(Box::new(CommandSerializeFailed));
        }
    }
    pub fn js_call_method<'a>(
        &'a self,
        method_name: &'a str,
        args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
    ) -> JsValue {
        let self_id = self.id;
        call_function_inner(
            &self.connection,
            &format_args!("{GET}({self_id}).{method_name}"),
            args,
        )
    }
}

fn call_function_inner<'a>(
    remote_arc: &'a Arc<ReentrantMutex<RefCell<WrmiLink>>>,
    function: &std::fmt::Arguments<'_>,
    args: impl IntoIterator<Item = &'a dyn UseInJsCode>,
) -> JsValue {
    let remote = remote_arc.lock();
    let mut remote = remote.borrow_mut();
    let out_id = remote.get_new_id();
    write!(remote.raw_commands_buf(), "{SET}({out_id},{function}(").unwrap();
    for arg in args.into_iter() {
        let arg = UseInJsCodeWriter(arg);
        if write!(remote.raw_commands_buf(), "{arg},").is_err() {
            remote.kill(Box::new(CommandSerializeFailed));
        }
    }
    write!(remote.raw_commands_buf(), "));\n").unwrap();
    JsValue {
        id: out_id,
        connection: Arc::clone(remote_arc),
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
