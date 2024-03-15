# WSDOM

WSDOM is a roundtrip-free Rust → JavaScript *Remote Method Invocation* or *Distributed Objects* system.
It lets Rust code hold and manipulate JavaScript objects over the network.

WSDOM can be used to add network-dependent functionalities to webpages without writing JS code or making API endpoints. It can also be integrated into "LiveView"-style Rust web frameworks to expose access to the full Web API.

# Quick Example
```rust
// this Rust code runs on a Rust server
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
WSDOMConnectWebSocket("ws://example.com:4000/");
```

Our full "Hello World!" code (using with Tokio's [Axum web framework](https://github.com/tokio-rs/axum/)) is available [here](/exampls/hello/).

# Key Features (and Anti-Features)
-   WSDOM generates **strongly-typed** Rust stubs for JS classes/functions/methods based on `.d.ts` TypeScript definitions.
    Stubs for the base JavaScript and DOM API are generated from the official TypeScript definitions [here](https://github.com/microsoft/TypeScript/tree/main/src/lib).
-   Calling JS code with WSDOM is **roundtrip-free**. This Rust code
    ```rust
    let mut val: JsNumber = browser.new_value(&1.0);
    for _ in 0..100 {
        val = wsdom::js::Math::cos(&browser, &val); // Math.cos on the JS side
    }
    ```
    does not block on the network at all; it will finish in microseconds.
    -   Roundtrip-free calling is possible because **WSDOM keeps values on the JS side**, sending them back to Rust only when explicitly requested.
        To get the value computed by the loop above, one would do
        ```rust
        let val_retrieved: f64 = val.retrieve_float().await;
        println!("the value of (cos^[100])(1.0) computed in JavaScript is {val_retrieved}");
        ```
        the `.await` will take one network roundtrip.
-   Due to the roundtrip-free design, **WSDOM fundamentally cannot handle JS exceptions**.
    -   If one of the `Math.cos` calls in our loop above throws,
        the Rust loop will still complete all 100 iterations without panic or any sort of warning (see [the Roundtrip Free Calls section](#roundtrip-free-calls) for why).
        As you might expect, this means code using WSDOM are **very painful to debug**.
-   WSDOM is **one-way**. Rust code can call JS code but not the other way around.
    -   To make event handling possible, WSDOM employs **Futures-based interactivity** with JS callbacks connected to Rust streams.
        ```rust
        async fn example(browser: Browser, button: &HTMLElement) {
            let (stream, callback) = wsdom::callback::new_callback::<MouseEvent>(&browser);
            button.add_event_listener(&"click", &callback, &wsdom::undefined());
            let _click_event: MouseEvent = stream.next().await; // wait for the Stream to yield
            println!("button was clicked on the browser!");
        }
        ```
-   WSDOM itself is **transport-agnostic**, **framework-agnostic**, and **executor-agnostic**,
    but I provide an integration library for easily getting started with WSDOM on
    [Axum web framework](https://github.com/tokio-rs/axum/) (which uses the Tokio executor) with WebSocket.

# Comparisons
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

You can think of a `JsValue` as a smart pointer that points to object in a heap,
only that the heap lives in JavaScript and is on a remote machine. A bump allocator running on the Rust side manages the heap.
<!-- **A bit of Rust evangelism**: In most other programming languages, a memory management scheme like this wouldn't be possible.
Rust's precise memory management means `Drop::drop` is called as soon as the object is no longer needed.
In a GC-ed language, it can take minutes (or even hours) until object destructors are called.
During this time, the JS-side `VALUES` map would fill up, wasting memory. -->

## Roundtrip-Free Calls
Our memory management approach enables roundtrip-free calls.
The way we do it is easily demonstrated with an example.
This Rust code
```rust
let val_a = browser.new_value::<JsNumber>(&0.5);
let val_b = wsdom::js::Math::cos(&browser, &val_a);
let val_c = wsdom::js::Math::cos(&browser, &val_b);
```
will produce JavaScript code that looks something like
```js
VALUES.set(1, 0.5); // browser.new_value(&0.5)
VALUES.set(2, Math.cos(VALUES.get(1))); // val_b = ...
VALUES.delete(1); // val_a dropped
VALUES.set(3, Math.cos(VALUES.get(2))); // val_c = ...
VALUES.delete(2); // val_b dropped
```
The serialized JS code are sent to the JS side in batches.
This means *the 3-lines Rust code probably finishes before the first line of JS code even reaches the JS side*.
You now see where our "fundamentally cannot handle JS exceptions" anti-feature came from.

# Disclaimer

Use WSDOM at your own risk. It is alpha-quality at best.

The Rust code produced by our `.d.ts` loader might change between WSDOM versions.

