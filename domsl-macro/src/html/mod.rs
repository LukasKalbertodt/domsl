mod tags;

use proc_macro2::Ident;

use crate::{
    error::Error,
};
pub(crate) use tags::TAG_INFOS;


/// Information about a specific tag.
#[derive(Debug)]
pub(crate) struct TagInfo {
    /// The name of the tag. E.g. `a`, `br` or `img`.
    pub(crate) name: &'static str,

    /// The name of the type in `web_sys`. This closely correlates with the
    /// "interface" in the HTML standard (only the casing is different).
    pub(crate) ty: &'static str,

    /// What content models the tag belongs to.
    pub(crate) categories: &'static [ContentModel],

    /// What kind of children are allowed in this tag.
    pub(crate) children: &'static [Child],

    /// What attributes are allowed on this tag. This only lists non-global
    /// attributes. Every tag allows global attributes.
    pub(crate) attributes: &'static [&'static str],
}

impl TagInfo {
    /// Returns
    pub(crate) fn from_name(name: &Ident) -> Result<&'static Self, Error> {
        // Yes, we actually do check if an array is sorted before doing a
        // binary search. BUT, we only do it once and only in debug mode. I
        // still like to have this assert here as otherwise, a binary search on
        // an unsorted array might lead to really strange errors.
        #[cfg(debug_assertions)]
        {
            use std::sync::Once;

            static CHECK_SORTED: Once = Once::new();
            CHECK_SORTED.call_once(|| {
                let is_sorted = TAG_INFOS.iter()
                    .zip(&TAG_INFOS[1..])
                    .all(|(a, b)| a.name <= b.name);

                if !is_sorted {
                    panic!("TAG_INFOS array is not sorted! This is a bug!");
                }
            })
        }

        TAG_INFOS
            .binary_search_by_key(&name.to_string().as_str(), |info| info.name)
            .map(|pos| &TAG_INFOS[pos])
            .map_err(|_| Error::unknown_tag(name))
    }
}

/// The main content models of HTML.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ContentModel {
    Metadata,
    Flow,
    Sectioning,
    Heading,
    Phrasing,
    Embedded,
    Interactive,
}

/// Specifies what kind of child is allowed for another tag.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Child {
    /// The same children are allowed in this tag as in the parent tag.
    Transparent,

    /// Text is allowed.
    Text,

    /// Elements of a specific content model are allowed
    Model(ContentModel),

    /// A specific tag is allowed.
    Tag(&'static str),
}
