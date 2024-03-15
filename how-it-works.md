# How WSDOM Works
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
