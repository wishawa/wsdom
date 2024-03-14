use proc_macro2::TokenStream;

pub(crate) fn load_ts_macro(input: syn::LitStr) -> syn::Result<TokenStream> {
    let filepath = input.value();
    let dts_file = get_file(&filepath, &input)?;
    let res = wsdom_ts_convert::convert(dts_file)
        .map_err(|e| syn::Error::new(input.span(), e.to_string()))?;

    Ok(res)
}

pub(crate) fn load_custom_ts_macro(input: syn::LitStr) -> syn::Result<TokenStream> {
    let filepath = input.value();
    let dts_file = get_file(&filepath, &input)?;
    let res = wsdom_ts_convert::convert_custom(dts_file)
        .map_err(|e| syn::Error::new(input.span(), e.to_string()))?;

    Ok(res)
}

fn get_file(path: &str, input: &syn::LitStr) -> syn::Result<std::fs::File> {
    if !path.ends_with(".d.ts") {
        return Err(syn::Error::new(
            input.span(),
            "must be typescript .d.ts declaration file",
        ));
    }
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
    let full_path = std::path::Path::new(&root).join(&path);

    let dts_file = full_path.is_file().then(|| {
		std::fs::File::open(&full_path).ok()
	}).flatten().ok_or_else(|| syn::Error::new(input.span(), "failed to open file; make sure path is relative to crate root (CARGO_MANIFEST_DIR)"))?;

    Ok(dts_file)
}
