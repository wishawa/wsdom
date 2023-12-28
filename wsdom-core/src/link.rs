use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

/// A WRMI client.
///
/// You can use this to call JS functions on the client.
/// Every JsValue holds a Browser object which they internally use for calling methods, etc.
///
/// Browser uses Arc internally, so cloning is cheap and a cloned Browser points to the same client.
#[derive(Clone, Debug)]
pub struct Browser(pub(crate) Arc<Mutex<BrowserInternal>>);

impl Browser {
    pub fn new() -> Self {
        let link = BrowserInternal {
            retrieve_values: HashMap::new(),
            retrieve_wakers: HashMap::new(),
            last_id: 1,
            commands_buf: String::new(),
            outgoing_waker: None,
            dead: None,
        };
        Self(Arc::new(Mutex::new(link)))
    }
    pub fn receive_incoming_message(&self, message: String) {
        self.0.lock().unwrap().receive(message);
    }
    pub fn get_outgoing_stream(&self) -> OutgoingMessages {
        OutgoingMessages {
            link: self.0.clone(),
        }
    }
}

pub struct OutgoingMessages {
    link: Arc<Mutex<BrowserInternal>>,
}

impl futures_core::Stream for OutgoingMessages {
    type Item = String;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let mut link = this.link.lock().unwrap();

        if link.dead.is_some() {
            return Poll::Ready(None);
        }

        let new_waker = cx.waker();
        if !link
            .outgoing_waker
            .as_ref()
            .is_some_and(|w| w.will_wake(new_waker))
        {
            link.outgoing_waker = Some(new_waker.to_owned());
        }
        if !link.commands_buf.is_empty() {
            Poll::Ready(Some(std::mem::take(&mut link.commands_buf)))
        } else {
            Poll::Pending
        }
    }
}

#[derive(Debug)]
pub struct BrowserInternal {
    pub(crate) retrieve_wakers: HashMap<u64, Waker>,
    pub(crate) retrieve_values: HashMap<u64, String>,
    last_id: u64,
    commands_buf: String,
    outgoing_waker: Option<Waker>,
    dead: Option<Box<dyn Error + Send>>,
}

impl BrowserInternal {
    pub fn receive(&mut self, message: String) {
        match message
            .split_once(':')
            .and_then(|(id, _)| id.parse::<u64>().ok())
        {
            Some(id) => {
                if let Some(waker) = self.retrieve_wakers.remove(&id) {
                    self.retrieve_values.insert(id, message);
                    waker.wake();
                }
            }
            None => self.kill(Box::new(InvalidReturn)),
        }
    }
    pub fn raw_commands_buf(&mut self) -> &mut String {
        &mut self.commands_buf
    }
    pub(crate) fn get_new_id(&mut self) -> u64 {
        self.last_id += 1;
        self.last_id
    }
    pub(crate) fn kill(&mut self, err: Box<dyn Error + Send>) {
        if self.dead.is_none() {
            self.dead = Some(err);
        }
    }
    pub(crate) fn wake_outgoing(&mut self) {
        if let Some(waker) = self.outgoing_waker.as_ref() {
            waker.wake_by_ref();
        }
    }
}

struct InvalidReturn;
impl std::fmt::Debug for InvalidReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InvalidReturn").finish()
    }
}
impl std::fmt::Display for InvalidReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::error::Error for InvalidReturn {}
