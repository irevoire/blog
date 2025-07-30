use proc_macro::TokenStream;

mod create_file;
mod text;

#[proc_macro]
pub fn create_file(tokens: TokenStream) -> TokenStream {
    create_file::create_file(tokens.into()).into()
}

#[proc_macro]
pub fn text(tokens: TokenStream) -> TokenStream {
    text::text(tokens.into()).into()
}
