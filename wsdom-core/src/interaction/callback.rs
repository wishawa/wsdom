use std::{marker::PhantomData, pin::Pin, task::Poll};

use futures_core::Future;

use crate::{js::value::JsValue, js_cast::JsCast, link::Browser};

pub struct Callback<E> {
    val_id: u64,
    ret_id: u64,
    browser: Browser,
    _phantom: PhantomData<Pin<Box<E>>>,
}

impl<E: JsCast> Future for Callback<E> {
    type Output = E;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut link = this.browser.0.lock().unwrap();
        if link.retrieve_values.remove(&this.ret_id).is_some() {
            return Poll::Ready(E::unchecked_from_js(JsValue {
                browser: this.browser.clone(),
                id: this.val_id,
            }));
        }
        let waker = cx.waker();
        match link.retrieve_wakers.entry(this.ret_id) {
            std::collections::hash_map::Entry::Occupied(mut occ) => {
                if !occ.get().will_wake(waker) {
                    occ.insert(waker.to_owned());
                }
            }
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(waker.to_owned());
            }
        }
        Poll::Pending
    }
}

fn new_callback<E>() -> (Callback<E>, JsValue) {
    todo!()
}
