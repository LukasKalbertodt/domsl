use std::fmt::Display;

use web_sys::{Document, Node};


pub struct Wrap<'a, T>(pub &'a T);

// ===== Implementation for `T: Into<Node>` ==================================
//
// This is the top priority: if a type implements `Into<Node>`, this "impl" is
// taken, regardless of whether it also implements `AsRef<str>`, `Display` or
// `IntoIterator`.

pub struct NodeTag;

impl NodeTag {
    pub fn into_node<T: AsRef<Node>>(self, t: T, _: &Document) -> Node {
        t.as_ref().clone()
    }
}

pub trait NodeKind {
    fn domsl_into_nodes(&self) -> NodeTag {
        NodeTag
    }
}

impl<T: AsRef<Node>> NodeKind for &&&&&&Wrap<'_, T> {}


// ===== Second priority: `T: AsRef<str>` ==================================

pub struct AsStrTag;

impl AsStrTag {
    pub fn into_node(self, x: impl AsRef<str>, document: &Document) -> Node {
        document.create_text_node(x.as_ref()).into()
    }
}

pub trait AsStrKind {
    fn domsl_into_nodes(&self) -> AsStrTag {
        AsStrTag
    }
}

impl<T: AsRef<str>> AsStrKind for &&&&Wrap<'_, T> {}


// ===== Third priority: `T: AsRef<str>` ==================================

pub struct DisplayTag;

impl DisplayTag {
    pub fn into_node(self, x: impl Display, document: &Document) -> Node {
        document.create_text_node(&x.to_string()).into()
    }
}

pub trait DisplayKind {
    fn domsl_into_nodes(&self) -> DisplayTag {
        DisplayTag
    }
}

// One `&` more for second priority
impl<T: Display> DisplayKind for &&Wrap<'_, T> {}




// ===== Third priority: `T: IntoIterator<Item: Into<Node>>` ===============

pub struct IterNodeTag;

impl IterNodeTag {
    pub fn into_node(
        self,
        iter: impl IntoIterator<Item: Into<Node>>,
        document: &Document,
    ) -> Node {
        let frag = document.create_document_fragment();
        for e in iter {
            frag.append_child(&e.into()).unwrap();
        }
        frag.into()
    }
}

pub trait IterNodeKind {
    fn domsl_into_nodes(&self) -> IterNodeTag {
        IterNodeTag
    }
}

impl<T: IntoIterator<Item: Into<Node>>> IterNodeKind for Wrap<'_, T> {}





// pub trait IntoNodes {
//     fn domsl_into_node(self, document: &Document) -> Node;
// }

// impl IntoNodes for &Node {
//     fn domsl_into_node(self, _: &Document) -> Node {
//         self.clone()
//     }
// }

// impl IntoNodes for &str {
//     fn domsl_into_node(self, document: &Document) -> Node {
//         document.create_text_node(self).into()
//     }
// }
