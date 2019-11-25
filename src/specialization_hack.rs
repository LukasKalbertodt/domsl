//! The dirty details of the autoref-based specialization used for converting
//! values into a `Node`.
//!
//! If you are a user of `domsl`, you probably only care about the "easy"
//! explanation relevant for users. In that case, please refer to
//! [`IntoNode`][crate::IntoNode]. However, if you are here for the gory
//! details of this hack, feel free to poke around. The general technique was
//! first described by dtolnay in [this document][1]. To use it for more than
//! two impls and for more trait bounds, the technique needed some slight
//! adjuestments. You can read about this generalized technique in this blog
//! post of mine (TODO: actually write blog post!).
//!
//! [1]: https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

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
    fn domsl_into_node(&self) -> NodeTag {
        NodeTag
    }
}

impl<T: AsRef<Node>> NodeKind for &&&&&&&&&&Wrap<'_, T> {}


// ===== Second priority: `T: AsRef<str>` ==================================

pub struct AsStrTag;

impl AsStrTag {
    pub fn into_node(self, x: impl AsRef<str>, document: &Document) -> Node {
        document.create_text_node(x.as_ref()).into()
    }
}

pub trait AsStrKind {
    fn domsl_into_node(&self) -> AsStrTag {
        AsStrTag
    }
}

impl<T: AsRef<str>> AsStrKind for &&&&&&&&Wrap<'_, T> {}


// ===== Third priority: `T: Display` ==================================

pub struct DisplayTag;

impl DisplayTag {
    pub fn into_node(self, x: impl Display, document: &Document) -> Node {
        document.create_text_node(&x.to_string()).into()
    }
}

pub trait DisplayKind {
    fn domsl_into_node(&self) -> DisplayTag {
        DisplayTag
    }
}

// One `&` more for second priority
impl<T: Display> DisplayKind for &&&&&&Wrap<'_, T> {}




// ===== `T: IntoIterator<Item: AsRef<Node>>` ===============

pub struct IterNodeTag;

impl IterNodeTag {
    pub fn into_node(
        self,
        iter: impl IntoIterator<Item: AsRef<Node>>,
        document: &Document,
    ) -> Node {
        let frag = document.create_document_fragment();
        for e in iter {
            frag.append_child(&e.as_ref()).unwrap();
        }
        frag.into()
    }
}

pub trait IterNodeKind {
    fn domsl_into_node(&self) -> IterNodeTag {
        IterNodeTag
    }
}

impl<T: IntoIterator<Item: AsRef<Node>>> IterNodeKind for &&&&Wrap<'_, T> {}


// ===== `T: IntoIterator<Item: AsRef<Node>>` ===============
pub struct IterStrTag;

impl IterStrTag {
    pub fn into_node(
        self,
        iter: impl IntoIterator<Item: AsRef<str>>,
        document: &Document,
    ) -> Node {
        let frag = document.create_document_fragment();
        for e in iter {
            let t = document.create_text_node(e.as_ref());
            frag.append_child(&t).unwrap();
        }
        frag.into()
    }
}

pub trait IterStrKind {
    fn domsl_into_node(&self) -> IterStrTag {
        IterStrTag
    }
}

impl<T: IntoIterator<Item: AsRef<str>>> IterStrKind for &&Wrap<'_, T> {}


// ===== `T: IntoIterator<Item: Display>` ===============
pub struct IterDisplayTag;

impl IterDisplayTag {
    pub fn into_node(
        self,
        iter: impl IntoIterator<Item: Display>,
        document: &Document,
    ) -> Node {
        let frag = document.create_document_fragment();
        for e in iter {
            // TODO: maybe reuse string buffer
            let t = document.create_text_node(&e.to_string());
            frag.append_child(&t).unwrap();
        }
        frag.into()
    }
}

pub trait IterDisplayKind {
    fn domsl_into_node(&self) -> IterDisplayTag {
        IterDisplayTag
    }
}

impl<T: IntoIterator<Item: Display>> IterDisplayKind for Wrap<'_, T> {}





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
