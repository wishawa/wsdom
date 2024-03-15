//! Integration code for conveniently using WSDOM with the Axum web framework.
//!
//! This library provides only one function: [socket_to_browser].

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{Future, Sink, Stream, StreamExt};
use pin_project_lite::pin_project;
use wsdom_core::Browser;

pin_project! {
    /// Future type returned from [socket_to_browser].
    pub struct ToBrowserFuture<Fut: Future> {
        #[pin] ws: WebSocket,
        #[pin] fut: Fut,
        browser: Browser,
        output: Option<Fut::Output>
    }
}

impl<Fut> Future for ToBrowserFuture<Fut>
where
    Fut: Future,
{
    type Output = Output<Fut::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        match this.ws.as_mut().poll_next(cx) {
            Poll::Ready(Some(Ok(Message::Text(message)))) => {
                this.browser.receive_incoming_message(message);
            }
            Poll::Ready(None | Some(Ok(Message::Close(_)))) => {
                return Poll::Ready(Output::ConnectionClosed);
            }
            Poll::Ready(Some(Ok(_))) => {}
            Poll::Ready(Some(Err(e))) => {
                return Poll::Ready(Output::AxumError(e));
            }
            Poll::Pending => {}
        }
        match this.ws.as_mut().poll_ready(cx) {
            Poll::Ready(Ok(_)) => {
                match this.browser.poll_next_unpin(cx) {
                    Poll::Ready(Some(message)) => {
                        match this.ws.as_mut().start_send(Message::Text(message)) {
                            Ok(_) => {}
                            Err(e) => return Poll::Ready(Output::AxumError(e)),
                        }
                    }
                    Poll::Ready(None) => {
                        if let Some(err) = this.browser.take_error() {
                            return Poll::Ready(Output::WsdomError(err));
                        } else {
                            return Poll::Pending;
                        }
                    }
                    Poll::Pending => {}
                }
                match this.ws.as_mut().poll_flush(cx) {
                    Poll::Ready(Err(e)) => {
                        return Poll::Ready(Output::AxumError(e));
                    }
                    Poll::Ready(Ok(_)) => {
                        if let Some(output) = this.output.take() {
                            return Poll::Ready(Output::Done(output));
                        }
                    }
                    _ => {}
                }
            }
            Poll::Ready(Err(e)) => {
                return Poll::Ready(Output::AxumError(e));
            }
            Poll::Pending => {}
        }
        if this.output.is_none() {
            if let Poll::Ready(t) = this.fut.poll(cx) {
                *this.output = Some(t);
            }
        }
        Poll::Pending
    }
}

/// Output type of [ToBrowserFuture].
pub enum Output<T> {
    /// The inner function (the second argument passed to `socket_to_browser`) completed with this result.
    Done(T),
    /// The WebSocket connection was closed, for whatever reason.
    ConnectionClosed,
    /// Axum raised an error.
    AxumError(axum::Error),
    /// WSDOM raised an error.
    WsdomError(wsdom_core::Error),
}

/// Get a [Browser] from an [axum::WebSocket] object.
///
/// You should first follow [axum's tutorial on how to obtain the WebSocket](https://docs.rs/axum/latest/axum/extract/ws/index.html).
/// Once you have the WebSocket object,
/// pass it to `socket_to_browser` along with an async function/closure that takes a Browser as argument,
/// then await the returned Future.
///
/// ```rust
/// # use wsdom_core::Browser;
/// use wsdom_axum::socket_to_browser;
/// use axum::extract::{WebSocketUpgrade, ws::WebSocket};
/// use axum::response::Response;
/// async fn axum_handler(wsu: WebSocketUpgrade) -> Response {
///     wsu.on_upgrade(|ws: WebSocket| async move {
///         socket_to_browser(ws, app).await;
///     })
/// }
/// async fn app(browser: Browser) {
///     // do things...
/// }
/// ````
#[must_use = "the return type is a Future and should be .awaited"]
pub fn socket_to_browser<Func, Fut>(ws: WebSocket, f: Func) -> ToBrowserFuture<Fut>
where
    Func: FnOnce(Browser) -> Fut,
    Fut: Future,
{
    let browser = Browser::new();
    ToBrowserFuture {
        fut: f(browser.clone()),
        ws,
        browser,
        output: None,
    }
}
