extern crate proc_macro;

use proc_macro2::{Ident, Spacing, TokenStream, TokenTree};
use quote::quote;
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

    let gen_code = gen(&item)?;
    let out = quote! {{
        use wasm_bindgen::prelude::*;
        use web_sys::{Document};

        let document: Document = #document;

        #gen_code
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

fn gen(item: &SnaxItem) -> Result<TokenStream, Error> {
    let tokens = match item {
        SnaxItem::Tag(tag) => {
            check_name(&tag.name)?;
            let name = tag.name.to_string();
            let set_attrs = set_attributes(&tag.attributes)?;
            let add_children = tag.children.iter().map(|c| {
                let child = gen(c)?;
                Ok(quote! {
                    elem.append_child(&#child.into()).unwrap();
                })
            }).collect::<Result<TokenStream, Error>>()?;

            quote! {{
                // This only fails if we pass in a name with incorrect
                // characters, like space. We assure that this is not the case
                // in `check_name`.
                let elem = document.create_element(#name).unwrap();
                #set_attrs
                #add_children
                elem
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
                let elem = document.create_element(#name).unwrap();
                #set_attrs
                elem
            }}
        }
        SnaxItem::Fragment(fragment) => {
            println!("frag: {:?}", fragment);
            quote! {}
        }
        SnaxItem::Content(tt) => {
            println!("tt: {:?}", tt);
            quote! {}
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
                Ok(quote! {
                    // This only errors if 'name' contains illegal characters
                    // which we check in `check_attribute_name`.
                    //
                    // TODO: The `to_string()` here is useless for string
                    // literals. Those should be special cased.
                    elem.set_attribute(#name, &#value.to_string()).unwrap();
                })
            }
        }
    }).collect()
}

fn check_attribute_name(_name: &Ident) -> Result<(), Error> {
    // TODO
    Ok(())
}

fn check_name(_name: &Ident) -> Result<(), Error> {
    // TODO
    Ok(())
}
