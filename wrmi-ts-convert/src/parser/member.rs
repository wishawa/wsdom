use winnow::{
    combinator::{alt, terminated},
    PResult, Parser,
};

use super::{
    field::Field,
    method::Method,
    util::{token, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Member<'a> {
    Method(Method<'a>),
    Field(Field<'a>),
}
impl<'a> Parsable<'a> for Member<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        terminated(
            alt((
                Method::parse.map(Self::Method),
                Field::parse.map(Self::Field),
            )),
            token(';'),
        )
        .parse_next(input)
    }
}
