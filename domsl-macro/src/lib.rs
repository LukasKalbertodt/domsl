extern crate proc_macro;

use proc_macro2::{Ident, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use snax::{SnaxAttribute, SnaxItem};

use crate::{
    error::Error,
};

mod error;



#[proc_macro]
pub fn jsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    run(input.into())
        .unwrap_or_else(|e| e.error_tokens)
        .into()
}

fn run(input: TokenStream) -> Result<TokenStream, Error> {
    let (document, body) = parse_outer(input)?;
    let item = snax::parse(body.into())?;

    let ty_cast = match &item {
        SnaxItem::Tag(tag) => {
            let ty = element_type(&tag.name)?;
            quote! { .dyn_into::<web_sys::#ty>().unwrap() }
        }
        SnaxItem::SelfClosingTag(tag) => {
            let ty = element_type(&tag.name)?;
            quote! { .dyn_into::<web_sys::#ty>().unwrap() }
        }

        // The expressions already evaluates to the correct type.
        SnaxItem::Fragment(_) => quote! {},
        SnaxItem::Content(_) => quote! {},
    };

    let gen_code = gen(&item)?;
    let out = quote! {{
        use wasm_bindgen::{prelude::*, JsCast};
        use web_sys::{Document};
        use domsl::hidden::{AsStrKind, NodeKind, DisplayKind, IterNodeKind, Wrap};

        let #DOCUMENT_IDENT: &Document = &*&#document;

        #gen_code #ty_cast
    }};

    Ok(out)
}

fn parse_outer(input: TokenStream) -> Result<(Ident, TokenStream), Error> {
    let mut iter = input.into_iter();

    let document = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(tt) => return Err(Error::expected("ident", tt)),
        None => return Err(Error::eof()),
    };

    match (iter.next(), iter.next()) {
        (Some(TokenTree::Punct(p0)), Some(TokenTree::Punct(p1)))
            if p0.as_char() == '='
                && p1.as_char() == '>'
                && p0.spacing() == Spacing::Joint
                && p1.spacing() == Spacing::Alone => {}
        (Some(tt), _) => return Err(Error::expected("`=>`", tt)),
        (None, _) => return Err(Error::eof()),
    }

    let body = match iter.next() {
        Some(TokenTree::Group(g)) => g.stream(),
        Some(tt) => return Err(Error::expected("`{}` block", tt)),
        None => return Err(Error::eof()),
    };

    if let Some(tt) = iter.next() {
        return Err(Error::new(tt.span(), "expected end of input, but found this"));
    }

    Ok((document, body))
}

/// Generates an expression that creates a `web_sys::Node` representing the
/// given `item`.
fn gen(item: &SnaxItem) -> Result<TokenStream, Error> {
    let tokens = match item {
        SnaxItem::Tag(tag) => {
            check_name(&tag.name)?;
            let name = tag.name.to_string();
            let set_attrs = set_attributes(&tag.attributes)?;
            let add_children = add_children(&tag.children)?;

            quote! {{
                // This only fails if we pass in a name with incorrect
                // characters, like space. We assure that this is not the case
                // in `check_name`.
                let #NODE_IDENT = #DOCUMENT_IDENT.create_element(#name).unwrap();
                #set_attrs
                #add_children
                web_sys::Node::from(#NODE_IDENT)
            }}
        }
        SnaxItem::SelfClosingTag(tag) => {
            check_name(&tag.name)?;
            let name = tag.name.to_string();
            let set_attrs = set_attributes(&tag.attributes)?;

            quote! {{
                // This only fails if we pass in a name with incorrect
                // characters, like space. We assure that this is not the case
                // in `check_name`.
                let #NODE_IDENT = #DOCUMENT_IDENT.create_element(#name).unwrap();
                #set_attrs
                web_sys::Node::from(#NODE_IDENT)
            }}
        }
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
                (&&&&&&&&Wrap(&#TMP_IDENT)).domsl_into_nodes()
                    .into_node(#TMP_IDENT, &#DOCUMENT_IDENT)
            }}
        }
    };

    Ok(tokens)
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
        let child = gen(c)?;
        Ok(quote! { #NODE_IDENT.append_child(&#child).unwrap(); })
    }).collect()
}

fn check_attribute_name(_name: &Ident) -> Result<(), Error> {
    // TODO
    Ok(())
}

fn check_name(name: &Ident) -> Result<(), Error> {
    element_type(name).map(|_| ())
}

fn element_type(name: &Ident) -> Result<Ident, Error> {
    let type_name = match name.to_string().as_str() {
        "address" | "article" | "aside" | "b" | "code" | "dd" | "dt" | "figcaption"
            | "figure" | "footer" | "header" | "hgroup" | "i" | "main" | "nav"
            | "section" | "u" => "HtmlElement",

        "a" => "HtmlAnchorElement",
        "br" => "HtmlBrElement",
        "div" => "HtmlDivElement",
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => "HtmlHeadingElement",
        "li" => "HtmlLiElement",
        "ol" => "HtmlOlListElement",
        "p" => "HtmlParagraphElement",
        "pre" => "HtmlPreElement",
        "span" => "HtmlSpanElement",
        "ul" => "HtmlUlListElement",

        // TODO: obviously, lots are missing still. This is a good list of all
        // tags: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
        //
        // We might want to error on deprecated tags.

        _ => {
            return Err(Error::new(name.span(), &format!("unknown HTML tag `{}`", name)));
        }
    };

    Ok(Ident::new(type_name, Span::call_site()))
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
