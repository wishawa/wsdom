use wsdom::{
    js_types::{JsNumber, JsValue},
    Browser, JsCast,
};

pub async fn app(browser: Browser) {
    // Newton-Raphson method
    let func = wsdom::js::Function::new(
        &browser,
        &[&"x", &"return 5.0 * Math.pow(x, 2) - 3.0 * x - 4.2"],
    );
    let mut x = browser.new_value::<JsNumber>(&0.0);
    for _ in 0..200 {
        const DELTA: f64 = 0.00001;
        let fx: JsNumber = func.js_call_self([&x as _], false).unchecked_into();
        let fxdelta: JsNumber = func
            .js_call_self([&(&x + &DELTA) as _], false)
            .unchecked_into();
        let dfx = &(&fxdelta - &fx) / &DELTA;
        x = &x - &(&fx / &dfx);
    }
    wsdom::dom::console(&browser).log(&[&"The root of 5x^2 - 3x + 4.2 is:"]);
    wsdom::dom::console(&browser).log(&[AsRef::<JsValue>::as_ref(&x)]);
    let document = wsdom::dom::document(&browser);
    let text_node = document.create_text_node(&"check your browser console");
    document.get_body().append_child(&text_node);
}
