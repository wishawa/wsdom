use super::Array;
use wrmi_core::{JsCast, ToJs};

impl<T, U> ToJs<Array<T>> for [U]
where
    T: JsCast,
    U: ToJs<T>,
{
}