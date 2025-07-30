use std::{env::current_dir, path::PathBuf, process::Command, str::FromStr};

use proc_macro2::{TokenStream, TokenTree, token_stream::IntoIter};
use quote::quote_spanned;

pub fn create_file(tokens: TokenStream) -> TokenStream {
    let trunk_dist_dir = PathBuf::from(current_dir().unwrap().join("dist").join(".stage"));
    let _ = std::fs::create_dir_all(&trunk_dist_dir);

    let mut tokens = tokens.into_iter();
    let paths = extract_path(&mut tokens);

    for path in paths {
        let Ok(path) = path else {
            return path.unwrap_err();
        };
        let file_path = trunk_dist_dir.join(&path);
        let _ = std::fs::create_dir_all(file_path.parent().unwrap());

        let go_back_by = path.components().count().saturating_sub(1);
        let relative_path_to_index_html = format!("./{}{}", "../".repeat(go_back_by), "index.html");
        Command::new("ln")
            .current_dir(&trunk_dist_dir)
            .arg("-sf")
            .arg(relative_path_to_index_html)
            .arg(path)
            .output()
            .unwrap();
    }
    TokenStream::new()
}

fn extract_path(tokens: &mut IntoIter) -> impl Iterator<Item = Result<PathBuf, TokenStream>> {
    let mut expecting_a_comma = false;

    tokens.filter_map(move |token| match token {
        TokenTree::Literal(literal) => {
            if expecting_a_comma {
                return Some(Err(quote_spanned! {
                    literal.span() =>
                    compile_error!("Was expecting a comma `,`");
                }));
            }
            let to_string = literal.to_string();
            if to_string.chars().next() != Some('"') || to_string.chars().last() != Some('"') {
                Some(Err(quote_spanned! {
                    literal.span() =>
                    compile_error!("expected path delimited by `\"`");
                }))
            } else {
                match PathBuf::from_str(to_string.trim_matches('"')) {
                    Ok(path) => {
                        expecting_a_comma = true;
                        Some(Ok(path))
                    }
                    Err(e) => {
                        let e = e.to_string();
                        Some(Err(quote_spanned! {
                            literal.span() =>
                            compile_error!(#e);
                        }))
                    }
                }
            }
        }
        TokenTree::Punct(punct) if expecting_a_comma && punct.as_char() == ',' => {
            expecting_a_comma = false;
            None
        }
        other => {
            let err = if expecting_a_comma {
                "Was expecting a comma"
            } else {
                "Was expecting a path in a string literal"
            };
            Some(Err(quote_spanned! {
                other.span() =>
                compile_error!(#err);
            }))
        }
    })
}
