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

pub fn convert(
    file: std::fs::File,
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    use winnow::Parser;

    let content = std::io::read_to_string(file)?;
    let mut input = &*content;

    let _imports = match parser::parse_imports.parse_next(&mut input) {
        Ok(i) => i,
        Err(e) => {
            return Err(ParseError {
                error: e
                    .into_inner()
                    .expect("complete parsers should not report `ErrMode::Incomplete(_)`"),
                content,
            }
            .into())
        }
    };

    let parsed = match parser::parse_all.parse(input) {
        Ok(i) => i,
        Err(e) => {
            return Err(ParseError {
                error: e.into_inner(),
                content,
            }
            .into())
        }
    };

    let types = generator::generate_all(&parsed);

    Ok(types)
}

#[cfg(test)]
mod tests {
    use winnow::Parser;

    use crate::parser::parse_imports;

    #[test]
    fn parse_file() {
        let filepath = "../data/dom.generated.d.ts";
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
        let full_path = std::path::Path::new(&root).join(&filepath);

        let dts_content = full_path
            .is_file()
            .then(|| {
                std::fs::File::open(&full_path)
                    .ok()
                    .and_then(|f| std::io::read_to_string(f).ok())
            })
            .flatten()
            .unwrap();

        let mut input = &*dts_content;
        let _imports = parse_imports.parse_next(&mut input).unwrap();

        crate::parser::parse_all(&mut input).unwrap();
    }
}
