use proc_macro2::TokenStream;
use quote::quote;

use crate::text::parser::{Statement, Expression};

mod parser;
mod print;

pub fn text(tokens: TokenStream) -> TokenStream {
    let s = tokens.to_string();
    println!("{:?}", s);

    let statements = parser::parse(&s);
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        let mut _text_compositor = crate::macros::text::TextCompositor::new(ui.style().clone());
    });
    for statement in statements {
        println!("parsed a first statement: {:?}", statement);
        ts.extend(statement.to_token_stream());
        println!("extended with {:?}, text is now {:?}", statement, ts.to_string());
    }

    println!("Out of loop");
    ts.extend(quote! {
        let _text_compositor = _text_compositor.finish();
        ui.label(_text_compositor);
    });

    println!("final text is {:?}", ts.to_string());
    ts
}
