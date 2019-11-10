use web_sys::{Node, Text};

pub use domsl_macro::jsx;

pub trait IntoNode {
    fn domsl_into_node(self) -> Node;
}

impl IntoNode for Node {
    fn domsl_into_node(self) -> Node {
        self
    }
}

impl IntoNode for &str {
    fn domsl_into_node(self) -> Node {
        // I wasn't able to find a reason why this JS function would throw an
        // exception. So for now we assume it won't happen.
        Text::new_with_data(self)
            .expect("`Text::new_with_data` returned an error (this is a `domsl` bug)")
            .into()
    }
}
