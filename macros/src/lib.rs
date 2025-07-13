use std::{env::current_dir, path::PathBuf, process::Command};

use proc_macro::{token_stream::IntoIter, TokenStream, TokenTree};

#[proc_macro]
pub fn create_file(tokens: TokenStream) -> TokenStream {
    let trunk_dist_dir = PathBuf::from(current_dir().unwrap().join("dist").join(".stage"));
    let _ = std::fs::create_dir_all(&trunk_dist_dir);

    let mut tokens = tokens.into_iter();
    let path = extract_path(&mut tokens).unwrap();

    let file_path = trunk_dist_dir.join(path);

    let index_html_path = current_dir().unwrap().join("dist").join("index.html");
    let _ = std::fs::create_dir_all(file_path.parent().unwrap());

    // println!("Running: ln -sn {} {}", index_html_path.display(), file_path.display());
    Command::new("ln").arg("-sn").arg(index_html_path).arg(file_path).output().unwrap();
    TokenStream::new()
}

fn extract_path(tokens: &mut IntoIter) -> Option<String> {
    let mut path = String::new();
    for token in tokens {
        match token {
            TokenTree::Ident(ident) => {
                path.push_str(&ident.to_string());
            }
            TokenTree::Punct(punct) => {
                path.push_str(&punct.to_string());
            }
            TokenTree::Literal(literal) => {
                path.push_str(&literal.to_string());
            }
            TokenTree::Group(group) => {
                panic!("Was expecting a path but received {} after {}", group.to_string(), path);
            }
        }
    }
    Some(path)
}