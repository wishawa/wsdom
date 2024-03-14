use parser::{comment::WithComment, item::Item};
use quote::quote;

pub(crate) mod generator;
pub(crate) mod parser;

struct ParseError {
    content: String,
    error: winnow::error::ContextError,
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseError")
            .field("content", &self.content)
            .field("error", &self.error)
            .finish()
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::error::Error for ParseError {}

fn parse_str(content: &str) -> Result<Vec<WithComment<'_, Item<'_>>>, Box<dyn std::error::Error>> {
    use winnow::Parser;
    let mut input = content;

    let _imports = match parser::parse_imports.parse_next(&mut input) {
        Ok(i) => i,
        Err(e) => {
            return Err(ParseError {
                error: e
                    .into_inner()
                    .expect("complete parsers should not report `ErrMode::Incomplete(_)`"),
                content: content.to_string(),
            }
            .into())
        }
    };

    parser::parse_all.parse(input).map_err(|e| {
        ParseError {
            error: e.into_inner(),
            content: content.to_string(),
        }
        .into()
    })
}

pub fn convert(
    file: std::fs::File,
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let content = std::io::read_to_string(file)?;
    let parsed = parse_str(&*content)?;
    let out = generator::generate_all(&parsed, &[]);

    Ok(out)
}

pub fn convert_custom(
    file: std::fs::File,
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let custom_content = std::io::read_to_string(file)?;
    let custom_parsed = parse_str(&*custom_content)?;

    let out = generator::generate_all(&custom_parsed, &[]);

    Ok(quote! {
        use wsdom::__wsdom_load_ts_macro;
        use wsdom::dom::*;
        use wsdom::js::*;
        #out
    })
}
