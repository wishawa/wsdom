use axum::{extract::ws::WebSocketUpgrade, response::Response, routing::get, Router};
use wsdom::Browser;

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
    let document = wsdom::dom::document(&browser);
    let body = document.get_body();
    let elem = document.create_element(&"div", &wsdom::null());
    elem.set_inner_text(&"Hello World!");
    body.append_child(&elem);
}
