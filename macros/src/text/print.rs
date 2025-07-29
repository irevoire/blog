use proc_macro2::TokenStream;
use quote::quote;

use crate::text::{parser::Statement, Expression};

impl<'a> Statement<'a> {
    pub(super) fn to_token_stream(&self) -> TokenStream {
        let mut ts = TokenStream::new();
        match self {
            Statement::Heading(expression) => {
                let expression = expression.to_token_stream();
                ts.extend(quote! {
                    _text_compositor.heading();
                    #expression
                    _text_compositor.pop();
                });
                ts
            },
            Statement::Expression(expression) => expression.to_token_stream(),
        }
    }
}

impl<'a> Expression<'a> {
    pub(super) fn to_token_stream(&self) -> TokenStream {
        let mut ts = TokenStream::new();
        match self {
            Expression::Bold(expression) => {
                let expression = expression.to_token_stream();
                ts.extend(quote! {
                    _text_compositor.strong();
                    #expression
                    _text_compositor.pop();
                });
                ts
            },
            Expression::Italic(expression) => {
                let expression = expression.to_token_stream();
                ts.extend(quote! {
                    _text_compositor.italic();
                    #expression
                    _text_compositor.pop();
                });
                ts
            },
            Expression::Underline(expression) => {
                let expression = expression.to_token_stream();
                ts.extend(quote! {
                    _text_compositor.underline();
                    #expression
                    _text_compositor.pop();
                });
                ts
            },
            Expression::Simple(text) => {
                ts.extend(quote! {
                    _text_compositor.append(#text);
                });
                ts
            },
        }
    }
}