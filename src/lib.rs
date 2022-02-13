use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn draw_text_on_canvas(canvas: web_sys::HtmlCanvasElement) {
    console_error_panic_hook::set_once();
    let width = canvas.client_width();
    let height = canvas.client_height();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Did not return CanvasRenderingContext2d");

    context.set_global_alpha(1.0);

    context.set_fill_style(&JsValue::from_str("rgb(255, 255, 255)"));
    context.fill_rect(0.0, 0.0, width.into(), height.into());

    context.set_fill_style(&JsValue::from_str("rgb(0, 0, 0)"));
    context
        .fill_text_with_max_width("Drawn in WASM", 0.into(), (height / 2).into(), width.into())
        .unwrap();
}
