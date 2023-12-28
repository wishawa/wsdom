use std::marker::PhantomData;
use std::sync::Mutex;
use std::{fmt::Write, future::Future, pin::Pin, task::Poll};

use serde::de::DeserializeOwned;

use crate::link::BrowserInternal;
use crate::protocol::{GET, REP};

pub struct RetrieveFuture<'a, T: DeserializeOwned> {
    pub(crate) id: u64,
    pub(crate) ret_id: Option<u64>,
    pub(crate) link: &'a Mutex<BrowserInternal>,
    _phantom: PhantomData<Pin<Box<T>>>,
}

impl<'a, T: DeserializeOwned> RetrieveFuture<'a, T> {
    pub(crate) fn new(id: u64, link: &'a Mutex<BrowserInternal>) -> Self {
        Self {
            id,
            ret_id: None,
            link,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: DeserializeOwned> Future for RetrieveFuture<'a, T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut link = this.link.lock().unwrap();
        let ret_id = if let Some(ret_id) = this.ret_id {
            if let Some(v) = link.retrieve_values.remove(&ret_id) {
                let v = v.split_once(':').unwrap().1;
                match serde_json::from_str(v) {
                    Ok(v) => return Poll::Ready(v),
                    Err(e) => link.kill(Box::new(e)),
                }
            }
            ret_id
        } else {
            let ret_id = link.get_new_id();
            this.ret_id = Some(ret_id);
            let this_id = this.id;
            write!(link.raw_commands_buf(), "{REP}({ret_id},{GET}({this_id}));").unwrap();
            link.wake_outgoing();
            ret_id
        };
        let waker = cx.waker();
        match link.retrieve_wakers.entry(ret_id) {
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
