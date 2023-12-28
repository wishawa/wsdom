use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use futures_util::SinkExt;
use wsdom::js_types::{JsValue, NullImmediate};

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
            let btn_add = document.create_element(&"button", &NullImmediate);
            btn_add.set_inner_text(&"+");
            let btn_sub = document.create_element(&"button", &NullImmediate);
            btn_sub.set_inner_text(&"-");
            let label = document.create_element(&"span", &NullImmediate);
            let mut value = 0;
            let mut click_add = {
                let (callback, func) = wsdom::callback::new_callback::<JsValue>(&browser);
                btn_add.add_event_listener(&"click", &func, &NullImmediate);
                callback
            };
            let mut click_sub = {
                let (callback, func) = wsdom::callback::new_callback::<JsValue>(&browser);
                btn_sub.add_event_listener(&"click", &func, &NullImmediate);
                callback
            };
            body.append_child(&btn_sub);
            body.append_child(&label);
            body.append_child(&btn_add);
            loop {
                label.set_inner_text(&&*format!("{value}"));
                tokio::select! {
                    _ = click_add.next() => {
                        value += 1;
                    }
                    _ = click_sub.next() => {
                        value -= 1;

                    }
                }
            }
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
