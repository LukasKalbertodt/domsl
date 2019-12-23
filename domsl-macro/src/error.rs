use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};


pub(crate) struct Error {
    error_tokens: TokenStream,
}

impl Error {
    pub(crate) fn new(span: Span, msg: &str) -> Self {
        let error_tokens = quote_spanned! {span=>
            compile_error!(#msg);
        };
        Self { error_tokens }
    }

    pub(crate) fn expr_error_tokens(self) -> TokenStream {
        let toks = self.error_tokens;
        quote! {
            { #toks }
        }
    }

    pub(crate) fn stmt_error_tokens(self) -> TokenStream {
        self.error_tokens
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

    pub(crate) fn unknown_tag(tag: &Ident) -> Self {
        Self::new(
            tag.span(),
            &format!(
                "unknown HTML tag '<{}>' (maybe you meant to capitalize it to call a component?)",
                tag,
            ),
        )
    }

    pub(crate) fn invalid_attr(attr: &Ident, tag: &str) -> Self {
        Self::new(
            attr.span(),
            &format!(
                "attribute '{}' is not valid on HTML tag '<{}>'",
                attr,
                tag,
            ),
        )
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

impl From<syn::Error> for Error {
    fn from(src: syn::Error) -> Self {
        Self {
            error_tokens: src.to_compile_error(),
        }
    }
}
