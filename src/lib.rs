#![feature(associated_type_bounds)]

use web_sys::{Document, Node};


pub mod specialization_hack;

pub use domsl_macro::jsx;


pub trait Component {
    type Node: Into<Node>;
    fn render(self, document: &Document, children: Vec<Node>) -> Self::Node;
}



/// Types that can be represented by a DOM node. This trait is actually fake.
///
///
/// # Background
///
/// To interpolate values into your markup, these values have to be converted
/// into a DOM node. So if you write `<div>{ foo }</div>` then the generated
/// code has to call `outer_div.append_child(&foo_as_node)` somehow. This is
/// simple if `foo` is already a `web_sys::Node`, but we also want to make it
/// possible to interpolate other types (like strings) into your markup.
///
/// One would think that we can easily write a trait (like this one) to
/// abstract over all types that can be converted into a node. But it's not
/// that easy, due to coherence rules: trait impls cannot overlap. We would
/// like to, for example, implement `IntoNode` for `web_sys::Node` *and* for
/// all `T: Display` (we can convert those to a string and then to a text
/// node). But what if `web_sys::Node` would impement `Display` in the next
/// release? Then the impls would overlap and we would be in trouble. That's
/// why we can't write out these two impls.
///
/// What we want is "specialization": one impl specializes another, more
/// general one. However, specialization is not stable yet, so we can only use
/// it with nightly compilers.
///
/// However, it turns out that it is actually possible to fake specialization
/// in some situations by (ab)using other features of Rust. You can find all
/// the details in [`specialization_hack`], but I wouldn't recommend you to
/// look at it. Rather, the description of this trait should give you all you
/// need to know about what values you can interpolate into your markup.
///
/// **Keep in mind**: this trait is fake. If you attempt to import it, you will
/// get an error message. It purely exists for documentation purposes.
///
///
/// # Types "implementing" this trait
///
/// This section describes what you mainly want to know: which types can you
/// put into the `jsx!` macro and which not. So if you you write
/// `<div>{ v }</div>` where `v` has the type `T`, the following things are
/// tried and the first successful one is used:
///
/// - **`T: AsRef<web_sys::Node>`** (*node-like*): this is implemented for lots
///   of types from `web_sys`, in particular for all types that represent an
///   HTML element, for `DocumentFragment` and for `Node` itself.
/// - **`T: AsRef<str>`** (*string-like*): this is implemented for `&str`,
///   `String`, `&String` and a few others. This is just a special case of the
///   `Display` case below to avoid one heap allocation in these cases. This
///   results in a text node (`web_sys::Text`).
/// - **`T: Display`**: for all types that have a reasonable standard way of
///   being represented as text. This also results in a text node
///   (`web_sys::Text`).
/// - **`T: IntoIterator`**: things that can be turned into iterators (like
///   `Vec`) will results in a `DocumentFragment` where all items of the
///   iterator are added as children. Unfortunately, this "specialization
///   trick" does not allow "recursion", so to speak. So we can't simply say
///   `IntoItertor<Item: IntoNode>`. Instead, one level of recursion is
///   manually implemented:
///   - **`T: IntoIterator<Item: AsRef<web_sys::Node>>`**
///   - **`T: IntoIterator<Item: AsRef<str>>`**
///   - **`T: IntoIterator<Item: Display>`**
///   - ... but no nested iterators. In these cases, `Iterator::flat_map` can
///     help you out!
///
#[cfg(doc)]
pub trait IntoNode {
    /// Creates a DOM node representing `self`. See this trait's documentation
    /// for more information.
    fn domsl_into_node(&self) -> web_sys::Node;
}
