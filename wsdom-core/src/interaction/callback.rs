use std::{fmt::Write, marker::PhantomData, pin::Pin, task::Poll};

use crate::{
    js::value::JsValue,
    js_cast::JsCast,
    link::{Browser, RetrievalState},
    protocol::{DEL, GET, REP, SET},
};

pub struct Callback<E> {
    arr_id: u64,
    ret_id: u64,
    browser: Browser,
    consumed: usize,
    _phantom: PhantomData<Pin<Box<E>>>,
}

impl<E: JsCast> futures_core::Stream for Callback<E> {
    type Item = E;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let mut link = this.browser.0.lock().unwrap();
        let ret_id = this.ret_id;
        match link.retrievals.entry(ret_id) {
            std::collections::hash_map::Entry::Occupied(mut occ) => {
                let state = occ.get_mut();

                let new_waker = cx.waker();
                if !state.waker.will_wake(new_waker) {
                    state.waker = new_waker.to_owned();
                }

                if state.times > this.consumed {
                    this.consumed += 1;
                    let val_id = link.get_new_id();
                    let arr_id = this.arr_id;
                    writeln!(
                        link.raw_commands_buf(),
                        "{SET}({val_id}, {GET}({arr_id}).shift());"
                    )
                    .unwrap();
                    link.wake_outgoing_lazy();
                    Poll::Ready(Some(JsCast::unchecked_from_js(JsValue {
                        id: val_id,
                        browser: this.browser.to_owned(),
                    })))
                } else {
                    Poll::Pending
                }
            }
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(RetrievalState {
                    waker: cx.waker().to_owned(),
                    last_value: String::new(),
                    times: 0,
                });
                Poll::Pending
            }
        }
    }
}
impl<E> Drop for Callback<E> {
    fn drop(&mut self) {
        let mut link = self.browser.0.lock().unwrap();
        let ret_id = self.ret_id;
        link.retrievals.remove(&ret_id);
        let arr_id = self.arr_id;
        writeln!(link.raw_commands_buf(), "{DEL}({arr_id});").unwrap();
    }
}

pub fn new_callback<E>(browser: &Browser) -> (Callback<E>, JsValue) {
    let mut link = browser.0.lock().unwrap();
    let arr_id = link.get_new_id();
    let ret_id = link.get_new_id();
    let func_id = link.get_new_id();
    let func = JsValue {
        browser: browser.to_owned(),
        id: func_id,
    };
    writeln!(link.raw_commands_buf(),
"{SET}({arr_id}, []); {SET}({func_id}, function(e) {{ {GET}({arr_id}).push(e); {REP}({ret_id}, 0) }});").unwrap();
    link.wake_outgoing_lazy();
    let callback = Callback {
        browser: browser.to_owned(),
        ret_id,
        arr_id,
        consumed: 0,
        _phantom: PhantomData,
    };
    (callback, func)
}
