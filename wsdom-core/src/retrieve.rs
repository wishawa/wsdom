use std::marker::PhantomData;
use std::sync::Mutex;
use std::{fmt::Write, future::Future, pin::Pin, task::Poll};

use serde::de::DeserializeOwned;

use crate::link::{BrowserInternal, Error, RetrievalState};
use crate::protocol::{GET, REP};

/// A [Future] for retrieving value from the JS side to the Rust side.
///
/// If something goes wrong (for example if the network disconnects), this Future will simply pend forever.
pub struct RetrieveFuture<'a, T: DeserializeOwned> {
    pub(crate) id: u64,
    pub(crate) ret_id: u64,
    pub(crate) link: &'a Mutex<BrowserInternal>,
    _phantom: PhantomData<Pin<Box<T>>>,
}

impl<'a, T: DeserializeOwned> RetrieveFuture<'a, T> {
    pub(crate) fn new(id: u64, link: &'a Mutex<BrowserInternal>) -> Self {
        Self {
            id,
            ret_id: 0,
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
        match this.ret_id {
            0 => {
                let ret_id = link.get_new_id();
                this.ret_id = ret_id;
                let this_id = this.id;
                writeln!(link.raw_commands_buf(), "{REP}({ret_id},{GET}({this_id}));").unwrap();
                link.wake_outgoing();
                link.retrievals.insert(
                    ret_id,
                    RetrievalState {
                        waker: cx.waker().to_owned(),
                        last_value: String::new(),
                        times: 0,
                    },
                );
                Poll::Pending
            }
            ret_id => match link.retrievals.entry(ret_id) {
                std::collections::hash_map::Entry::Occupied(mut occ) => match occ.get_mut() {
                    RetrievalState {
                        waker, times: 0, ..
                    } => {
                        let new_waker = cx.waker();
                        if !waker.will_wake(new_waker) {
                            *waker = new_waker.to_owned();
                        }
                        Poll::Pending
                    }
                    RetrievalState { last_value, .. } => {
                        let v = std::mem::take(last_value);
                        occ.remove();
                        let v = v.split_once(':').unwrap().1;
                        match serde_json::from_str(v) {
                            Ok(v) => {
                                this.ret_id = 0;
                                Poll::Ready(v)
                            }
                            Err(e) => {
                                link.kill(Error::DataDeserialize(e));
                                Poll::Pending
                            }
                        }
                    }
                },
                std::collections::hash_map::Entry::Vacant(_) => Poll::Pending,
            },
        }
    }
}

impl<'a, T: DeserializeOwned> Drop for RetrieveFuture<'a, T> {
    fn drop(&mut self) {
        match self.ret_id {
            0 => {
                // NO-OP
            }
            ret_id => {
                let mut link = self.link.lock().unwrap();
                link.retrievals.remove(&ret_id);
            }
        }
    }
}
