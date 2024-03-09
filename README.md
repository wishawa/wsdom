# WSDOM

WSDOM is a Rust → JavaScript Remote Method Invocation (RMI) system. It lets you hold and use JavaScript objects from Rust code running on a remote server.

WSDOM can be used to add interactivity to webpages without writing JS code or making API endpoints. It can also be integrated into "LiveView"-style Rust web frameworks to expose access to the full Web API.

# Quick Example
```rust
```

# Features
-   WSDOM is **strongly-typed**. Rust stub types are generated from the [TypeScript definitions for the DOM API](https://github.com/microsoft/TypeScript/tree/main/src/lib). You can also bring your own `.d.ts`; see [here](/examples/custom-def/).
-   WSDOM is **roudtrip-free**. This code
    ```rust
    async fn example(browser: Browser) {
        let mut val: JsNumber = browser.new_value(&1.0);
        for _ in 0..1000 {
            val = wsdom::js::Math::cos(&val);
        }
        let val_retrieved = val.retrieve_f64().await;
        println!("{val_retrieved}");
    }
    ```
    takes only one roundtrip latency to execute because code in the loop body does not block on network. See [how this works](/guides/how-it-works.md).
-   WSDOM is **memory safe & efficient**. JS objects are automatically freed when and only when they are no longer needed. See [how this works](/guides/how-it-works.md).
-   WSDOM has **pluggable transport**. Any text protocol can be substituted for the default WebSocket transport. See [this example](/examples/custom-transport/).
-   WSDOM is one-way, so JS code cannot directly call Rust code. To make interactivity (events handling, etc.) possible, WSDOM employs **Futures-based interactivity** with JS callbacks connected to Rust streams.
    ```rust
    async fn example(browser: Browser, button: &HtmlElement) {
        let (stream, callback) = wsdom::callback::new_callback::<JsValue>(&browser);
        button.add_event_listener(&"click", &callback, &NullImmediate);
        let _click_event = stream.next().await; // wait for the Stream to yield
        println!("button was clicked!");
    }
    ```

# Disclaimer

This is alpha-quality software at best. Use at your own risk.

