use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{Future, Sink, Stream, StreamExt};
use pin_project_lite::pin_project;
use wsdom_core::Browser;

pin_project! {
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

pub enum Output<T> {
    Done(T),
    ConnectionClosed,
    AxumError(axum::Error),
    WsdomError(wsdom_core::Error),
}

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
