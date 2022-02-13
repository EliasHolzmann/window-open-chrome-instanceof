use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn append_div_to_document_containing(element: web_sys::HtmlElement) {
    console_error_panic_hook::set_once();

    let document = element
        .get_root_node()
        .dyn_into::<web_sys::HtmlDocument>()
        .expect("get_root_node() did not return Document");

    let div = document
        .create_element("div")
        .expect("create_element() failed");
    div.set_text_content(Some("Created by WASM"));

    document
        .body()
        .expect("root_node.body() returned null")
        .append_with_node_1(&div)
        .expect("Appending failed");
}
