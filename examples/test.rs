#![feature(proc_macro_hygiene)]

use domsl::{jsx, Component};
use web_sys::{Document, Node};

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let node = jsx!(document => {
        <Foo color="red">"Hello"</Foo>
    });
}

struct Foo {
    color: &'static str,
}
impl Component for Foo {
    type Node = web_sys::HtmlParagraphElement;
    fn render(self, document: &Document, children: Vec<Node>) -> Self::Node {
        jsx!(document => {
            <p style={ format!("color: {}", self.color) }>{ children }</p>
        })
    }
}
