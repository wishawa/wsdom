The "LiveView" design has been gaining popularity
(we have [Phoenix LiveView](https://github.com/phoenixframework/phoenix_live_view), [LiveViewJS](https://www.liveviewjs.com/), [Dioxus LiveView](https://dioxuslabs.com/learn/0.5/reference/liveview)).
But why limit ourselves to *View*? Why not Live*Everything*?

# WSDOM
WSDOM is a no-roundtrip Rust → JavaScript *Remote Method Invocation* or *Distributed Objects* system.
It lets Rust code hold JavaScript objects and call methods/functions over the network.

WSDOM can be used to add network-dependent functionalities to webpages without writing JS code or making API endpoints. It can also be integrated into "LiveView"-style Rust web frameworks to expose access to the full Web API.

## Quick Example
Here is an example using WSDOM to put `<div>Hello World!</div>` on a webpage.
```rust
// this Rust code runs on your web server
fn hello(browser: wsdom::Browser) {
    let document = wsdom::dom::document(&browser) // get hold of a Document object
    let body = document.get_body(); // get the <body /> of that document object
    let elem = document.create_element(&"div", &wsdom::undefined()); // create a <div />
    elem.set_inner_text(&"Hello World!"); // set the text
    body.append_child(&elem); // add the <div /> to the <body />
}
```
```js
// this JavaScript code runs on the browser
WSDOMConnectWebSocket("ws://my-website.domain:4000/");
```

Our full "Hello World!" code is available [here](/examples/hello/src/main.rs).

## Key Features (and Anti-Features)
-   WSDOM generates **strongly-typed** Rust stubs for JS classes/functions/methods based on `.d.ts` TypeScript definitions.
-   Calling JS code with WSDOM is **roundtrip-free**. This Rust code
    ```rust
    let mut val: JsNumber = browser.new_value(&1.0);
    for _ in 0..100 {
        val = wsdom::js::Math::cos(&browser, &val); // Math.cos on the JS side
    }
    ```
    does not block on the network at all; it will finish in microseconds.
    -   Roundtrip-free calling is possible because WSDOM **keeps values on the JS side**, sending them back to Rust only when explicitly requested.
        To get the value computed by the loop above, one would do
        ```rust
        let val_retrieved: f64 = val.retrieve_float().await;
        println!("the value of (cos^[100])(1.0) computed in JavaScript is {val_retrieved}");
        ```
        the `.await` will take one network roundtrip.
-   Due to the roundtrip-free design, WSDOM fundamentally **cannot handle JS exceptions**.
    -   If one of the `Math.cos` calls in our loop above throws,
        the Rust loop will still complete all 100 iterations without panic or any sort of warning (see [How It Works](how-it-works.md) for why).
        As you might expect, this means code using WSDOM are **very painful to debug**.
-   WSDOM is **one-way**. Rust code can call JS code but not the other way around.
    -   To make event handling possible, we have **Futures-based interactivity**;
        we connect JS callbacks to streams that can be awaited on the Rust side.
        ```rust
        async fn example(browser: Browser, button: &HTMLElement) {
            let (stream, callback) = wsdom::callback::new_callback::<MouseEvent>(&browser);
            button.add_event_listener(&"click", &callback, &wsdom::undefined());
            let _click_event: MouseEvent = stream.next().await; // wait for the Stream to yield
            println!("button was clicked on the browser!");
        }
        ```
-   WSDOM is **transport-agnostic**, **framework-agnostic**, and **executor-agnostic**.
    That said, we provide an integration library for easily getting started with WSDOM on
    [Axum web framework](https://github.com/tokio-rs/axum/) (which uses the Tokio executor) with WebSocket.

## Examples
Hosted examples are available.
When viewing them, I recommend opening your browser's network devtool to see the WebSocket traffic.
- Counter example: [code](/examples/many-examples/src/counter.rs) [demo](http://141.145.215.129:4000/counter)
- Audio API example: [code](/examples/many-examples/src/audio.rs) [demo](http://141.145.215.129:4000/audio)
- Canvas API example: [code](/examples/many-examples/src/canvas.rs) [demo](http://141.145.215.129:4000/canvas)

## Comparisons
### [web-sys](https://docs.rs/web-sys/latest/web_sys/)
WSDOM serves a similar role as web-sys (and a bit of [js-sys](https://docs.rs/js-sys/latest/js_sys/) too),
but instead of running your Rust in WebAssembly in the same browser,
we let you run your Rust code away across a WebSocket connection.

WSDOM's translation of JS API to Rust is different from web-sys.
We translate from TypeScript declarations, rather than directly from WebIDLs.
The network gap also means our optional types take the form of `JsNullable<_>` (compared to the `Option<_>` of web-sys).

### [jsdom](https://github.com/jsdom/jsdom)
WSDOM and jsdom are similar in that we both expose the web browser's API outside a web browser.
jsdom does so by implementing the API themselves.
WSDOM does so by forwarding calls to a real web browser running across a WebSocket connection.

## Details
The [How It Works](how-it-works.md) document describes how WSDOM works in more details.

The crate is [on crates.io](https://crates.io/crates/wsdom) and the documentation is [on docs.rs](https://docs.rs/wsdom/latest).

## Disclaimer

Use WSDOM at your own risk. It is alpha-quality at best.

The Rust code produced by our `.d.ts` loader might change between WSDOM versions.

