extern crate proc_macro;

use proc_macro2::{Ident, Spacing, TokenStream, TokenTree};

use crate::error::Error;

mod error;
mod gen;
mod html;



#[proc_macro]
pub fn jsx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    run(input.into())
        .unwrap_or_else(|e| e.error_tokens)
        .into()
}

fn run(input: TokenStream) -> Result<TokenStream, Error> {
    let (document, body) = parse_outer(input)?;
    let item = snax::parse(body.into())?;

    gen::gen(&item, &document)
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
