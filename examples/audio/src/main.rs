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
    let btn1 = document.create_element(&"button", &wsdom::undefined());
    let btn2 = document.create_element(&"button", &wsdom::undefined());
    btn1.set_inner_text(
        &"play t-rex sound with Audio() (make sure you don't have autoplay blocked)",
    );
    btn2.set_inner_text(&"play 440 Hz with WebAudio OscillatorNode");
    body.append_child(&btn1);
    body.append_child(&btn2);

    // prepare the audio
    let trex_audio = wsdom::dom::Audio(
        &browser,
        &Some(&"https://interactive-examples.mdn.mozilla.net/media/cc0-audio/t-rex-roar.mp3"),
    );
    let context = wsdom::dom::AudioContext::new(&browser, &wsdom::undefined());
    let oscillator = context.create_oscillator();
    oscillator.get_frequency().set_value(&440);
    oscillator.connect_with_destinationNode_output_input(
        &context.get_destination(),
        &wsdom::undefined(),
        &wsdom::undefined(),
    );
    context.suspend();
    oscillator.start(&wsdom::undefined());

    // set up the click listener
    let mut click_stream1 = {
        let (callback, func) = wsdom::callback::new_callback::<JsValue>(&browser);
        btn1.add_event_listener(&"click", &func, &wsdom::null());
        callback
    };
    let mut click_stream2 = {
        let (callback, func) = wsdom::callback::new_callback::<JsValue>(&browser);
        btn2.add_event_listener(&"click", &func, &wsdom::null());
        callback
    };

    // wait for click
    loop {
        tokio::select! {
            _ = click_stream1.next() => {
                trex_audio.set_current_time(&0.0);
                trex_audio.play();
            }
            _ = click_stream2.next() => {
                context.resume();
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                context.suspend();
            }
        }
    }
}
