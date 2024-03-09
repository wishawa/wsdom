use serde::Serialize;

use crate::js::value::JsValue;
use crate::protocol::GET;

/// For values that can be serialized to JS code:
/// - Rust values that implement `serde::Serialize`
/// - WRMI stubs ([JsValue]s)
///
/// This trait is used by [ToJs].
pub trait UseInJsCode {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl UseInJsCode for JsValue {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_id = self.id;
        write!(buf, "{GET}({self_id})").unwrap();
        Ok(())
    }
}

pub struct SerdeToJs<'a, T: ?Sized>(pub &'a T);

impl<'a, T: Serialize + ?Sized> UseInJsCode for SerdeToJs<'a, T> {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct WriteAdapter<'a, 'b>(&'a mut std::fmt::Formatter<'b>);
        impl<'a, 'b> std::io::Write for WriteAdapter<'a, 'b> {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                match self.0.write_str(unsafe {
                    // WriteAdapter is only to be written to by serde_json,
                    // which only writes valid utf8
                    std::str::from_utf8_unchecked(buf)
                }) {
                    Ok(()) => Ok(buf.len()),
                    Err(std::fmt::Error) => Err(std::io::ErrorKind::Other.into()),
                }
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        match serde_json::to_writer(&mut WriteAdapter(buf), self.0) {
            Ok(()) => Ok(()),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

pub(crate) struct UseInJsCodeWriter<'a, T: UseInJsCode + ?Sized>(pub &'a T);

impl<'a, T: UseInJsCode + ?Sized> std::fmt::Display for UseInJsCodeWriter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.serialize_to(f)
    }
}

pub struct RawCodeImmediate<'a>(pub &'a str);
impl<'a> UseInJsCode for RawCodeImmediate<'a> {
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        buf.write_str(self.0)
    }
}

/// Values that can be serialized to JS code satisfying certain types.
///
/// For example, `ToJs<JsNumber>` means serializable to the same type that
/// `JsNumber` serializes to.
pub trait ToJs<JsType>
where
    Self: UseInJsCode,
    JsType: ?Sized,
{
}

impl<T: ?Sized> ToJs<T> for T where T: UseInJsCode {}
