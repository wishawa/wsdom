use axum::{extract::ws::WebSocketUpgrade, response::Response, routing::get, Router};
use futures_util::StreamExt;
use wsdom::{js_types::JsValue, Browser};

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
    let btn = document.create_element(&"button", &wsdom::null());
    btn.set_inner_text(&"play t-rex sound (make sure you don't have autoplay blocked)");
    body.append_child(&btn);
    let audio = wsdom::dom::Audio(
        &browser,
        &Some(&"https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3"),
    );

    // set up the click listener
    let mut click_stream = {
        let (callback, func) = wsdom::callback::new_callback::<JsValue>(&browser);
        btn.add_event_listener(&"click", &func, &wsdom::null());
        callback
    };
    // wait for click
    loop {
        click_stream.next().await;
        audio.set_current_time(&0.0);
        audio.play();
    }
}
