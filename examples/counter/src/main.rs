use axum::{extract::ws::WebSocketUpgrade, response::Response, routing::get, Router};
use futures_util::StreamExt;
use wsdom::{
    js_types::{JsValue, NullImmediate},
    Browser,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| async {
        wsdom_axum::socket_to_browser(socket, app).await;
    })
}

async fn app(browser: Browser) {
    // create the UI
    let document = wsdom::dom::document(&browser);
    let body = document.get_body();
    let btn_add = document.create_element(&"button", &NullImmediate);
    btn_add.set_inner_text(&"+");
    let btn_sub = document.create_element(&"button", &NullImmediate);
    btn_sub.set_inner_text(&"-");
    let label = document.create_element(&"span", &NullImmediate);
    body.append_child(&btn_sub);
    body.append_child(&label);
    body.append_child(&btn_add);

    // start with count = 0
    let mut value = 0;

    // set up the click listeners
    let mut click_add_stream = {
        let (callback, func) = wsdom::callback::new_callback::<JsValue>(&browser);
        btn_add.add_event_listener(&"click", &func, &NullImmediate);
        callback
    };
    let mut click_sub_stream = {
        let (callback, func) = wsdom::callback::new_callback::<JsValue>(&browser);
        btn_sub.add_event_listener(&"click", &func, &NullImmediate);
        callback
    };

    // wait for event and update count
    loop {
        label.set_inner_text(&&*format!("{value}"));
        tokio::select! {
            _ = click_add_stream.next() => {
                value += 1;
            }
            _ = click_sub_stream.next() => {
                value -= 1;
            }
        }
    }
}
