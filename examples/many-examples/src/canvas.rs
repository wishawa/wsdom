use futures_util::StreamExt;
use wsdom::{js_types::JsValue, Browser, JsCast};

pub async fn app(browser: Browser) {
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
    // let rect_drawer: &wsdom::dom::CanvasRect = ctx.as_ref();
    // let line_drawer: &wsdom::dom::CanvasPath = ctx.as_ref();

    ctx.clear_rect(&0, &0, &200, &200);
    loop {
        btn.set_inner_text(&"draw");
        click_stream.next().await;

        ctx.begin_path();
        ctx.move_to(&75, &50);
        ctx.line_to(&100, &75);
        ctx.line_to(&100, &25);
        ctx.fill(&wsdom::undefined());

        btn.set_inner_text(&"clear");
        click_stream.next().await;
        ctx.clear_rect(&0, &0, &200, &200);
    }
}
