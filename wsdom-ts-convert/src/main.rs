use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match &*args {
        [_self_path, dts_path, output_path] => {
            work(dts_path, output_path);
        }
        _ => {
            println!("usage: wsdom_ts_convert /path/to/file.d.ts /path/to/output.rs");
            return;
        }
    }
}

fn work(dts_path: &str, output_path: &str) {
    if !dts_path.ends_with(".d.ts") {
        println!("file must be .d.ts");
        return;
    }
    let path = std::path::Path::new(dts_path);
    let Some(dts_file) = path
        .is_file()
        .then(|| std::fs::File::open(&path).ok())
        .flatten()
    else {
        println!("invalid .d.ts file");
        return;
    };
    let Some(mut out_file) = std::fs::File::create(std::path::Path::new(output_path)).ok() else {
        println!("cannot create output file");
        return;
    };

    let res = match wsdom_ts_convert::convert(dts_file) {
        Ok(r) => r,
        Err(e) => {
            println!("conversion error: {e}");
            return;
        }
    };

    let res = res.to_string();
    let write_res = write!(
        out_file,
        "
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]
mod __wsdom_load_ts_macro {{
    pub use ref_cast::RefCast;
    pub use wsdom_core::{{js_types::*, Browser, JsCast, ToJs, UseInJsCode, RawCodeImmediate}};
}}
use wsdom_javascript::*;

{}
",
        &res
    );
    if let Err(e) = write_res {
        println!("writing output failed: {e}");
        return;
    }
    println!("done!");
}
