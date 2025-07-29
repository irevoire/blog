use std::{env::current_dir, path::PathBuf, process::Command};

use proc_macro::{token_stream::IntoIter, TokenStream, TokenTree};

pub fn create_file(tokens: TokenStream) -> TokenStream {
    let trunk_dist_dir = PathBuf::from(current_dir().unwrap().join("dist").join(".stage"));
    let _ = std::fs::create_dir_all(&trunk_dist_dir);

    let mut tokens = tokens.into_iter();
    let path = extract_path(&mut tokens).unwrap();

    let file_path = trunk_dist_dir.join(&path);
    let _ = std::fs::create_dir_all(file_path.parent().unwrap());

    let go_back_by = path.chars().filter(|c| *c == '/').count();
    let relative_path_to_index_html = format!("./{}{}", "../".repeat(go_back_by), "index.html");
    Command::new("ln").current_dir(trunk_dist_dir).arg("-sf").arg(relative_path_to_index_html).arg(path).output().unwrap();
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