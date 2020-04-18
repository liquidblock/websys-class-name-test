
use wasm_bindgen::prelude::*;
use web_sys::console;

pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

pub fn test() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    let svg: web_sys::Element = document.create_element_ns(Some(SVG_NAMESPACE), "svg")?;

    body.append_child(&svg)?;

    console::log_1(&"Test Begin".into());
    svg.set_class_name("test");
    let _: String = svg.class_name();
    console::log_1(&"Test End".into());

    Ok(())
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use console_error_panic_hook;
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    test().expect("test returned error result");
    panic!("this is a test, if the panic hook works correctly")
}
