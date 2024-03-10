# WSDOM

WSDOM is a Rust → JavaScript *Remote Method Invocation* or *Distributed Objects* system.
It lets Rust code hold and manipulate JavaScript objects over the network.

WSDOM can be used to add network-dependent interactivity to webpages without writing JS code or making API endpoints. It can also be integrated into "LiveView"-style Rust web frameworks to expose access to the full Web API.

# Quick Example
```rust
// this Rust code runs on a Rust server
async fn app(browser: wsdom::Browser) {
    let document = wsdom::dom::document(&browser) // get hold of a Document object
    let body = document.get_body(); // get the <body /> of that document object
    let elem = document.create_element(&"div", &wsdom::undefined()); // create a <div />
    elem.set_inner_text(&"Hello World!"); // set the text
    body.append_child(&elem); // add the <div /> to the <body />
}
```
```js
// this JavaScript code runs on the browser
WSDOMConnectWebSocket("ws://example.com:3000/");
```

The full "Hello World!" code (using with Tokio's [Axum web framework](https://github.com/tokio-rs/axum/)) is available [here](/exampls/hello/).

# Features
-   WSDOM generates **strongly-typed** Rust stubs for JS classes/functions/methods based on `.d.ts` TypeScript definition.
    Stubs for the base JavaScript and DOM API are generated from the official TypeScript definitions [here](https://github.com/microsoft/TypeScript/tree/main/src/lib).
-   Calling JS code with WSDOM incurs **no network blocks**. This Rust code
    ```rust
    let mut val: JsNumber = browser.new_value(&1.0);
    for _ in 0..100 {
        val = wsdom::js::Math::cos(&browser, &val);
    }
    ```
    does not block at all; it will finish in microseconds.
-   WSDOM does **asynchronous retrieval**. Values are sent back to the Rust side only when explicitly requested.
    For example, to get the value computed by the loop above, one would do
    ```rust
    let val_retrieved: f64 = val.retrieve_f64().await;
    println!("the value of (cos^[100])(1.0) computed in JavaScript is {val_retrieved}");
    ```
    the `.await` will take one network roundtrip.
-   WSDOM is **memory safe & efficient**. JS objects are automatically freed when and only when they are no longer needed.
-   WSDOM is one-way, so JS code cannot directly call Rust code.
    To make event handling possible, WSDOM employs **Futures-based interactivity** with JS callbacks connected to Rust streams.
    ```rust
    async fn example(browser: Browser, button: &HTMLElement) {
        let (stream, callback) = wsdom::callback::new_callback::<MouseEvent>(&browser);
        button.add_event_listener(&"click", &callback, &NullImmediate);
        let _click_event = stream.next().await; // wait for the Stream to yield
        println!("button was clicked on the browser!");
    }
    ```
-   WSDOM has **pluggable transport**. Any text protocol can be substituted for the default WebSocket transport.
-   WSDOM is **framework-agnostic**. Any web framework with WebSocket or comparable two-way text protocol can be used.
    I provide a ready-made adapter for use with the [Axum web framework](https://github.com/tokio-rs/axum/) and its WebSocket implementation.
-   WSDOM is **executor-agnostic**. Any async Rust executor will work.

# How It Works
## Code Serialization
When you use WSDOM to call a JS function, WSDOM generates the equivalent JavaScript code and sends it over WebSocket.
The WSDOM client on the JS side receives the code and feeds it into [Function()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function) (`eval`'s little sibling) to execute it.

For example, if you run this Rust code
```rust
wsdom::dom::alert(&browser, &Some(&"hi"));
```
WSDOM would send something like
```js
alert("hi");
```
over the WebSocket.

The `eval`-like approach is chosen because WSDOM wants to expose the full Web API.
One can build a library similar to WSDOM but with more fine-grained permission on what JavaScript functions/classes could be accessed.
Such library would not need to rely on `Function()` or `eval()`.

## Memory Management
The memory management scheme of WSDOM is quite simple.

Each JS object held from the Rust side is associated with a unique ID
```rust
struct JsValue {
    // the ID of the object
    id: u32,
    // the connected web browser executing our JavaScript
    browser: Arc<...>,
}
```
and on the JavaScript side, we have a "heap" mapping each ID to the actual value
```typescript
var VALUES: Map<Id, any>;
```

Each time a `JsValue` needs to be created, an ID is allocated from a shared `u64` counter.
Here is a highly simplified example
```rust
fn cos(browser: &Browser, input: &JsNumber) -> JsNumber {
    let mut browser = browser.borrow_mut();
    let output_id = browser.counter;
    browser.counter += 1;
    browser.send_code(
        format!("VALUES.set({}, Math.cos({}))", output_id, input.id)
    );
    JsNumber::from(JsValue { id: output_id })
}
```

When a `JsValue` is dropped, the object it references can be freed by removing the repective map entry
```rust
impl Drop for JsValue {
    fn drop(&mut self) {
        self.browser.send_code(
            format!("VALUES.delete({});", self.id)
        );
    }
}
```

# Disclaimer

WSDOM is alpha-quality software at best. Use at your own risk.

