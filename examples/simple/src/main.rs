use std::time::Duration;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use futures_util::SinkExt;
use wsdom::js_types::NullImmediate;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    use futures_util::StreamExt;
    let browser = wsdom::Browser::new();

    tokio::spawn({
        let browser = browser.clone();
        async move {
            let document = wsdom::dom::document(&browser);
            let body = document.get_body();
            body.set_inner_text(&"connected!");
            for i in 0..5 {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let elem = document.create_element(&"div", &NullImmediate);
                elem.set_inner_text(&&*format!("{i}"));
                body.append_child(&elem);
            }
            let number = wsdom::js::Math::exp(&browser, &2.0);
            let num = number.retrieve_float().await;
            assert_eq!(num, (2.0f64).exp());
            tokio::time::sleep(Duration::from_secs(1)).await;
            wsdom::dom::alert(&browser, &"done");
        }
    });

    let (mut tx, mut rx) = socket.split();
    let tx_fut = {
        let browser = browser.clone();
        async move {
            tx.send_all(
                &mut browser
                    .get_outgoing_stream()
                    .map(|msg| Ok(Message::Text(msg))),
            )
            .await
        }
    };
    let rx_fut = {
        let browser = browser.clone();
        async move {
            while let Some(Ok(Message::Text(msg))) = rx.next().await {
                browser.receive_incoming_message(msg);
            }
        }
    };
    let _todo = tokio::join!(tx_fut, rx_fut);
}
