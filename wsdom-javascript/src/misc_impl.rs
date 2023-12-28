use super::Array;
use wsdom_core::{JsCast, ToJs};

impl<'a, T, U, const N: usize> ToJs<Array<T>> for [&'a U; N]
where
    T: JsCast,
    U: ToJs<T>,
{
}
