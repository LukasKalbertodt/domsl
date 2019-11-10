use web_sys::{Document, Node};

pub use domsl_macro::jsx;

pub trait IntoNode {
    fn domsl_into_node(self, document: &Document) -> Node;
}

impl IntoNode for &Node {
    fn domsl_into_node(self, _: &Document) -> Node {
        self.clone()
    }
}

impl IntoNode for &str {
    fn domsl_into_node(self, document: &Document) -> Node {
        document.create_text_node(self).into()
    }
}
