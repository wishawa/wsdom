use std::fmt::Write;

use crate::UseInJsCode;

impl<U> UseInJsCode for [U]
where
    U: UseInJsCode,
{
    fn serialize_to(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        buf.write_char('[')?;
        for item in self.iter() {
            item.serialize_to(buf)?;
            buf.write_char(',')?;
        }
        buf.write_char(']')?;
        Ok(())
    }
}
