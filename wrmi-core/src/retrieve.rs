use std::cell::RefCell;
use std::marker::PhantomData;
use std::{fmt::Write, future::Future, pin::Pin, task::Poll};

use parking_lot::ReentrantMutex;
use serde::de::DeserializeOwned;

use crate::connection::WrmiLink;
use crate::protocol::{GET, REP};

pub struct RetrieveFuture<'a, T: DeserializeOwned> {
    pub(crate) id: u64,
    pub(crate) ret_id: Option<u64>,
    pub(crate) remote: &'a ReentrantMutex<RefCell<WrmiLink>>,
    _phantom: PhantomData<Pin<Box<T>>>,
}

impl<'a, T: DeserializeOwned> RetrieveFuture<'a, T> {
    pub fn new(id: u64, remote: &'a ReentrantMutex<RefCell<WrmiLink>>) -> Self {
        Self {
            id,
            ret_id: None,
            remote,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: DeserializeOwned> Future for RetrieveFuture<'a, T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let remote = this.remote.lock();
        let mut remote = remote.borrow_mut();
        if let Some(ret_id) = this.ret_id {
            if let Some(v) = remote.retrieve_values.remove(&ret_id) {
                let v = v.split_once(':').unwrap().1;
                match serde_json::from_str(v) {
                    Ok(v) => return Poll::Ready(v),
                    Err(e) => remote.kill(Box::new(e)),
                }
            }
        } else {
            let ret_id = remote.get_new_id();
            this.ret_id = Some(ret_id);
            let this_id = this.id;
            write!(
                remote.raw_commands_buf(),
                "{REP}({ret_id},{GET}({this_id}));"
            )
            .unwrap();
        }
        let waker = cx.waker();
        match remote.retrieve_wakers.entry(this.id) {
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
