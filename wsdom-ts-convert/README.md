This crate has a library and a binary. The library is for use by the `load_ts!` macro (of `wsdom-macro`).
The binary is used to generate `wsdom::dom` (which is reexported from the `wsdom-dom` crate) because that file is to big that rust-analyzer choke on the macro.
```shell
cargo run ../typescript-defs/used/dom.generated.d.ts ../wsdom-dom/src/lib.rs
```