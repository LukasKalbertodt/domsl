use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::{ItemFn, spanned::Spanned};

use crate::error::Error;


pub(crate) fn run(attr_args: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    // Parse proc macro attribute
    let component_name = parse_name(attr_args)?;

    // ==========================================================================================
    // ===== Parse and verify function
    // ==========================================================================================
    let fn_def: ItemFn = syn::parse2(input).map_err(|e| {
        let msg = format!(
            "could not parse item as function: {}\n   = hint: \
                `#[domsl::component]` can only be attached to functions",
            e,
        );
        Error::new(e.span(), &msg)
    })?;

    let out_type = match fn_def.sig.output {
        syn::ReturnType::Default => {
            // TODO: span
            return Err(Error::spanless("a component's function has to return something"));
        }
        syn::ReturnType::Type(_, t) => t,
    };

    // Misc checks
    let props = [
        (fn_def.sig.asyncness.map(|x| x.span), "a domsl component's function cannot be async"),
        (fn_def.sig.unsafety.map(|x| x.span), "a domsl component's function cannot be unsafe"),
        (
            fn_def.sig.abi.map(|x| x.span()),
            "a domsl component's function cannot have an ABI specified"
        ),
        (
            fn_def.sig.variadic.map(|x| x.span()),
            "a domsl component's function cannot have variadic arguments"
        ),
    ];
    for &(prop, msg) in &props {
        if let Some(prop) = prop {
            return Err(Error::new(prop, msg));
        }
    }

    // Check and interpret function inputs
    let mut children_arg_visited = false;
    let mut fn_inputs = Vec::new();
    let mut call_arguments = Vec::new();
    let mut struct_fields = Vec::new();
    for input in fn_def.sig.inputs {
        // Make sure it's not a receiver parameter
        let mut input = match input {
            syn::FnArg::Receiver(r) => {
                let msg = "receiver argument not allowed in parameter list of a domsl component";
                return Err(Error::new(r.span(), msg));
            }
            syn::FnArg::Typed(input) => input,
        };

        // Make sure the pattern is a simple identifier
        let ident = match *input.pat {
            syn::Pat::Ident(syn::PatIdent {
                by_ref: None, mutability: None, subpat: None, ident, ..
            }) => {
                // `attrs` is ignored here: it will always be empty as the
                // attributes would be parsed as the inputs attributes and not
                // as the pattern's ones.
                ident
            }
            other => {
                let msg = "parameters in domsl components must have simple names \
                    (no other patterns)";
                return Err(Error::new(other.span(), msg));
            }
        };

        let mut is_children_arg = false;
        for attr in input.attrs.drain_filter(|attr| attr.path.is_ident("domsl")) {
            match &*attr.parse_args::<Ident>()?.to_string() {
                "children" => {
                    if children_arg_visited {
                        let msg = "second occurance of #[domsl(children)], but it must be used \
                            at most once";
                        return Err(Error::new(attr.span(), msg));
                    }
                    children_arg_visited = true;
                    is_children_arg = true;
                }
                other => {
                    let msg = format!("unknown domsl attribute '{}'", other);
                    return Err(Error::new(attr.span(), &msg));
                }
            }
        }

        let pair = NameAndType { ident: ident.clone(), ty: input.ty };

        let call_arg = if is_children_arg {
            quote! { children }
        } else {
            struct_fields.push(pair.clone());
            quote! { self.#ident }
        };
        call_arguments.push(call_arg);
        fn_inputs.push(pair);
    }


    // ===========================================================================================
    // ===== Generate the output
    // ===========================================================================================
    let ident = fn_def.sig.ident;
    let visibility = fn_def.vis;
    let generics = fn_def.sig.generics;
    let const_ = fn_def.sig.constness;
    let body = fn_def.block;
    let attrs = fn_def.attrs;

    let doc_string = format!("A domsl component. See [`{}`] for more information.", ident);

    Ok(quote! {
        #[doc = #doc_string]
        #visibility struct #component_name #generics {
            #( #struct_fields ,)*
        }

        impl ::domsl::Component for #component_name {
            type Node = #out_type;
            fn render(
                self,
                document: &::web_sys::Document,
                children: Vec<::web_sys::Node>,
            ) -> Self::Node {
                #ident( #( #call_arguments ,)* )
            }
        }

        #( #attrs )*
        #visibility #const_ fn #ident #generics ( #(#fn_inputs ,)* ) -> #out_type
            #body
    })
}

/// Parses the name of the component from the attribute token stream. E.g.
/// `#[domsl::component(Foo)]` (this would return `Foo`).
fn parse_name(attrs: TokenStream) -> Result<Ident, Error> {
    // TODO: infer name from function name?
    let mut iter = attrs.into_iter();

    let name = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(tt) => {
            let msg = "component name as identifier (e.g. `#[domsl::component(Foo)]`)";
            return Err(Error::expected(msg, tt));
        }
        None => {
            let msg = "missing component name (e.g. `#[domsl::component(MyComponent)]`)";
            return Err(Error::spanless(msg));
        }
    };

    if let Some(tt) = iter.next() {
        let msg = "expected only one identifier, but found this additional token";
        return Err(Error::new(tt.span(), msg));
    }

    Ok(name)
}


#[derive(Clone)]
struct NameAndType {
    ident: Ident,
    ty: Box<syn::Type>,
}

impl ToTokens for NameAndType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, ty } = self;
        tokens.extend(quote! {
            #ident : #ty
        });
    }
}
