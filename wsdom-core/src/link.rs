use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

/// A WSDOM client.
///
/// You can use this to call JS functions on the JS client (the web browser).
/// Every JsValue holds a Browser object which they internally use for calling methods, etc.
///
/// Browser uses Arc internally, so cloning is cheap and a cloned Browser points to the same client.
///
/// ## Use with Integration Library
///
/// You can obtain Browser from the WSDOM integration library (for example, `wsdom-axum`).
///
/// ## Manual Usage
///
/// If there is no WSDOM integration library for your framework,
/// you can instead create Browser manually with the `new()` method.
///
/// Manually created Browsers need to be "driven"
/// -   Browser implements the [Stream][futures_core::Stream] trait with [String].
///     You must take items from the stream and send it to the WSDOM JS client
///     over WebSocket or other transport of your choice.
/// -   Browser has a `receive_incoming_message(msg: String)` method.
///     Everything sent by the WSDOM JS client must be fed into this method.
///
/// The `counter-manual` example in our repo shows manual usage with Tokio.
#[derive(Clone, Debug)]
pub struct Browser(pub(crate) Arc<Mutex<BrowserInternal>>);

impl Browser {
    /// Create a new Browser object.
    ///
    /// This is only needed if you intend to go the "manual" route described above.
    pub fn new() -> Self {
        let link = BrowserInternal {
            retrievals: HashMap::new(),
            last_id: 1,
            commands_buf: String::new(),
            outgoing_waker: None,
            dead: ErrorState::NoError,
        };
        Self(Arc::new(Mutex::new(link)))
    }
    /// Receive a message sent from the WSDOM JS client.
    ///
    /// This is only needed if you intend to go the "manual" route described above.
    /// If you use an integration library, messages are handled automatically.
    pub fn receive_incoming_message(&self, message: String) {
        self.0.lock().unwrap().receive(message);
    }
    /// If the Browser has errored, this will return the error.
    ///
    /// The [Error] type is not [Clone], so after the first call returning `Some(_)`,
    /// this method will return `None`.
    pub fn take_error(&self) -> Option<Error> {
        let mut link = self.0.lock().unwrap();
        match std::mem::replace(&mut link.dead, ErrorState::ErrorTaken) {
            ErrorState::NoError => {
                link.dead = ErrorState::NoError;
                None
            }
            ErrorState::Error(e) => Some(e),
            ErrorState::ErrorTaken => None,
        }
    }
}

/// The stream of messages that should be sent over WebSocket (or your transport of choice) to the JavaScript WSDOM client.
impl futures_core::Stream for Browser {
    type Item = String;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let mut link = this.0.lock().unwrap();

        if !matches!(&link.dead, ErrorState::NoError) {
            return Poll::Ready(None);
        }

        let new_waker = cx.waker();
        if !link
            .outgoing_waker
            .as_ref()
            .is_some_and(|w| new_waker.will_wake(w))
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
    pub(crate) retrievals: HashMap<u64, RetrievalState>,
    last_id: u64,
    commands_buf: String,
    outgoing_waker: Option<Waker>,
    dead: ErrorState,
}

/// Error that could happen in WSDOM.
///
/// Currently, the only errors that could happen are from [serde] serialization and deserialization.
#[derive(Debug)]
pub enum Error {
    CommandSerialize(std::fmt::Error),
    DataDeserialize(serde_json::Error),
}
#[derive(Debug)]
enum ErrorState {
    NoError,
    Error(Error),
    ErrorTaken,
}

#[derive(Debug)]
pub(crate) struct RetrievalState {
    pub(crate) waker: Waker,
    pub(crate) last_value: String,
    pub(crate) times: usize,
}

impl BrowserInternal {
    pub fn receive(&mut self, message: String) {
        match message
            .split_once(':')
            .and_then(|(id, _)| id.parse::<u64>().ok())
        {
            Some(id) => match self.retrievals.get_mut(&id) {
                Some(s) => {
                    s.times += 1;
                    s.last_value = message;
                    s.waker.wake_by_ref();
                }
                _ => {}
            },
            None => {}
        }
    }
    pub fn raw_commands_buf(&mut self) -> &mut String {
        &mut self.commands_buf
    }
    pub(crate) fn get_new_id(&mut self) -> u64 {
        self.last_id += 1;
        self.last_id
    }
    pub(crate) fn kill(&mut self, err: Error) {
        if matches!(self.dead, ErrorState::NoError) {
            self.dead = ErrorState::Error(err);
        }
    }
    pub(crate) fn wake_outgoing(&mut self) {
        if let Some(waker) = self.outgoing_waker.as_ref() {
            waker.wake_by_ref();
        }
    }
    pub(crate) fn wake_outgoing_lazy(&mut self) {
        self.wake_outgoing();
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
