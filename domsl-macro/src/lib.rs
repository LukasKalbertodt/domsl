#![feature(drain_filter)]

extern crate proc_macro;

use proc_macro::{TokenStream as TokenStream1};
use proc_macro2::TokenStream;

use crate::error::Error;

mod component;
mod error;
mod jsx;
mod html;



#[proc_macro]
pub fn jsx(input: TokenStream1) -> TokenStream1 {
    fn run(input: TokenStream) -> Result<TokenStream, Error> {
        let (document, body) = jsx::parse_outer(input)?;
        let item = snax::parse(body.into())?;

        jsx::gen(&item, &document)
    }

    run(input.into())
        .unwrap_or_else(|e| e.expr_error_tokens())
        .into()
}


#[proc_macro_attribute]
pub fn component(attrs: TokenStream1, input: TokenStream1) -> TokenStream1 {
    component::run(attrs.into(), input.into())
        .unwrap_or_else(|e| e.stmt_error_tokens())
        .into()
}
