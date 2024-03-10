use axum::{extract::ws::WebSocketUpgrade, response::Response, routing::get, Router};
use futures_util::StreamExt;
use wsdom::{js_types::JsValue, Browser, JsCast};

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
    let canvas: wsdom::dom::HTMLCanvasElement = document
        .create_element(&"canvas", &wsdom::null())
        .unchecked_into();
    canvas.set_width(&200);
    canvas.set_height(&200);
    body.append_child(&canvas);

    let btn = document.create_element(&"button", &wsdom::null());
    body.append_child(&btn);

    // set up the click listener
    let mut click_stream = {
        let (callback, func) = wsdom::callback::new_callback::<JsValue>(&browser);
        btn.add_event_listener(&"click", &func, &wsdom::null());
        callback
    };

    let ctx: wsdom::dom::CanvasRenderingContext2D = canvas
        .get_context(&"2d", &wsdom::null())
        .unwrap()
        .unchecked_into();
    let rect_drawer: &wsdom::dom::CanvasRect = ctx.as_ref();
    let line_drawer: &wsdom::dom::CanvasPath = ctx.as_ref();

    rect_drawer.clear_rect(&0, &0, &200, &200);
    loop {
        btn.set_inner_text(&"draw");
        click_stream.next().await;

        wsdom::dom::CanvasDrawPath::begin_path(ctx.as_ref());
        line_drawer.move_to(&75, &50);
        line_drawer.line_to(&100, &75);
        line_drawer.line_to(&100, &25);
        wsdom::dom::CanvasDrawPath::fill(ctx.as_ref(), &wsdom::undefined());

        btn.set_inner_text(&"clear");
        click_stream.next().await;
        rect_drawer.clear_rect(&0, &0, &200, &200);
    }
}
