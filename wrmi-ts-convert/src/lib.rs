mod parser;

#[cfg(test)]
mod tests {
    use winnow::Parser;

    use crate::parser::parse_imports;

    #[test]
    fn parse_file() {
        let filepath = "../data/dom.iterable.generated.d.ts";
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

        if crate::parser::parse_all(&mut input).is_err() {
            println!("{input}");
        }
    }
}
