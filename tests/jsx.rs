#![feature(proc_macro_hygiene)]

use domsl::jsx;
use wasm_bindgen_test::*;
use web_sys::Document;

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

#[wasm_bindgen_test]
fn simple_div() {
    let d = doc();
    let n: web_sys::HtmlDivElement = jsx!(d => {<div></div>});
    assert_eq!(n.inner_text(), "");
    assert_eq!(n.inner_html(), "");
    assert_eq!(n.children().length(), 0);
}
