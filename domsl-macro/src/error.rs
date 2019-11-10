use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote_spanned;


pub(crate) struct Error {
    pub(crate) error_tokens: TokenStream,
}

impl Error {
    pub(crate) fn new(span: Span, msg: &str) -> Self {
        let error_tokens = quote_spanned! {span=>
            compile_error!(#msg);
        };
        Self { error_tokens }
    }

    pub(crate) fn expected(expected: &str, found: TokenTree) -> Self {
        Self::new(found.span(), &format!("expected {}, found `{}` instead", expected, found))
    }

    pub(crate) fn spanless(msg: &str) -> Self {
        Self::new(Span::call_site(), msg)
    }

    pub(crate) fn eof() -> Self {
        Self::spanless("unexpected end of input (forgot to close tag?)")
    }
}

impl From<snax::ParseError> for Error {
    fn from(src: snax::ParseError) -> Self {
        match src {
            snax::ParseError::UnexpectedEnd => Self::eof(),
            snax::ParseError::UnexpectedItem(html_token) => {
                Self::spanless(&format!("unexpected item {:?}", html_token))
            }
            snax::ParseError::UnexpectedToken(tt) => {
                Self::new(tt.span(), &format!("unexpected token `{}`", tt))
            }
        }
    }
}
