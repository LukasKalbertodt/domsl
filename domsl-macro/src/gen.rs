use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use snax::{SnaxAttribute, SnaxItem};

use crate::{
    error::Error,
    html::TagInfo,
};


pub(crate) fn gen(root: &SnaxItem, document: &Ident) -> Result<TokenStream, Error> {
    // We need to cast the outer most element appropriately.
    let ty_cast = match &root {
        SnaxItem::Tag(tag) => {
            let ty = TagInfo::from_name(&tag.name)?.ty;
            quote! { .dyn_into::<web_sys::#ty>().unwrap() }
        }
        SnaxItem::SelfClosingTag(tag) => {
            let ty = TagInfo::from_name(&tag.name)?.ty;
            quote! { .dyn_into::<web_sys::#ty>().unwrap() }
        }

        // The expressions already evaluates to the correct type.
        SnaxItem::Fragment(_) => quote! {},
        SnaxItem::Content(_) => quote! {},
    };

    let gen_code = gen_item(&root)?;
    let out = quote! {{
        use wasm_bindgen::{prelude::*, JsCast};
        use web_sys::{Document};
        use domsl::specialization_hack::{
            AsStrKind, NodeKind, DisplayKind, IterDisplayKind, IterNodeKind, IterStrKind, Wrap,
        };

        let #DOCUMENT_IDENT: &Document = &*&#document;

        #gen_code #ty_cast
    }};

    Ok(out)
}

/// Generates an expression that creates a `web_sys::Node` representing the
/// given `item`.
fn gen_item(item: &SnaxItem) -> Result<TokenStream, Error> {
    let tokens = match item {
        SnaxItem::Tag(tag) => gen_tag(&tag.name, &tag.attributes, &tag.children)?,
        SnaxItem::SelfClosingTag(tag) => gen_tag(&tag.name, &tag.attributes, &[])?,
        SnaxItem::Fragment(fragment) => {
            let add_children = add_children(&fragment.children)?;

            quote! {{
                let #NODE_IDENT = #DOCUMENT_IDENT.create_document_fragment();
                #add_children
                web_sys::Node::from(#NODE_IDENT)
            }}
        }
        SnaxItem::Content(tt) => {
            quote! {{
                let #TMP_IDENT = (#tt);
                (&&&&&&&&&&&&Wrap(&#TMP_IDENT)).domsl_into_node()
                    .into_node(#TMP_IDENT, &#DOCUMENT_IDENT)
            }}
        }
    };

    Ok(tokens)
}

fn gen_tag(
    name: &Ident,
    attributes: &[SnaxAttribute],
    children: &[SnaxItem],
) -> Result<TokenStream, Error> {
    let info = TagInfo::from_name(&name)?;
    let name_string = name.to_string();
    let set_attrs = set_attributes(&attributes)?;
    let add_children = add_children(&children)?;

    Ok(quote! {{
        // This only fails if we pass in a name with incorrect characters, like
        // spaces. We assure that this is not the case in `TagInfo::from_name`.
        let #NODE_IDENT = #DOCUMENT_IDENT.create_element(#name_string).unwrap();
        #set_attrs
        #add_children
        web_sys::Node::from(#NODE_IDENT)
    }})
}

fn set_attributes(attrs: &[SnaxAttribute]) -> Result<TokenStream, Error> {
    attrs.iter().map(|attr| {
        match attr {
            SnaxAttribute::Simple { name, value } => {
                check_attribute_name(name)?;

                let name = name.to_string();
                let value = match value {
                    // If the token tree is a string literal, we don't need to
                    // call `to_string()`.
                    //
                    // TODO: make this check a bit more robust.
                    TokenTree::Literal(lit) if lit.to_string().starts_with("\"") => {
                        quote ! { #lit }
                    }
                    other => {
                        // We generate this helper function to make sure
                        // `other` implements `Display`. By using its span, the
                        // user gets good error message if `Display` is not
                        // implemented.
                        let helper = quote_spanned!(other.span()=>
                            fn #HELPER_IDENT() -> impl std::fmt::Display { #other }
                        );
                        quote! {
                            &{
                                #helper
                                #HELPER_IDENT().to_string()
                            }
                        }
                    }
                };

                Ok(quote! {
                    // This only errors if 'name' contains illegal characters
                    // which we check in `check_attribute_name`.
                    #NODE_IDENT.set_attribute(#name, #value).unwrap();
                })
            }
        }
    }).collect()
}

fn add_children(children: &[SnaxItem]) -> Result<TokenStream, Error> {
    children.iter().map(|c| {
        let child = gen_item(c)?;
        Ok(quote! { #NODE_IDENT.append_child(&#child).unwrap(); })
    }).collect()
}

fn check_attribute_name(_name: &Ident) -> Result<(), Error> {
    // TODO
    Ok(())
}


const NODE_IDENT: DomslIdent = DomslIdent("__domsl_node");
const DOCUMENT_IDENT: DomslIdent = DomslIdent("__domsl_document");
const HELPER_IDENT: DomslIdent = DomslIdent("__domsl_helper");
const TMP_IDENT: DomslIdent = DomslIdent("__domsl_tmp");

/// This is a small helper type that can be constructed as const-fn and
/// implements `ToTokens`.
///
/// Unfortunately, hygiene is not stable for proc macros yet. So we need to use
/// terrible identifier in the code we emit to make clashes very unlikely. To
/// not repeat the strange identifiers in each `quote!` invocation, we would
/// like to have global constants for them. But we can't have them of type
/// `Ident` because `Ident::new` is not a const-fn and  because `lazy_static`
/// or `thread_local` doesnt work as they rely on deref-coercion to work.
/// That's what this type is for.
struct DomslIdent(&'static str);

impl quote::ToTokens for DomslIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Ident::new(self.0, Span::call_site()).to_tokens(tokens)
    }
}
