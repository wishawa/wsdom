use winnow::{
    ascii::multispace0, combinator::delimited, stream::AsChar, token::take_while, PResult, Parser,
};

pub(crate) trait Parsable<'a>: Sized {
    fn parse(input: &mut &'a str) -> PResult<Self>;
}

pub(crate) fn token<'a, O, S: Parser<&'a str, O, winnow::error::ContextError>>(
    s: S,
) -> impl Parser<&'a str, O, winnow::error::ContextError> {
    delimited(multispace0, s, multispace0)
}

pub(crate) fn word0<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(0.., (AsChar::is_alphanum, '_')).parse_next(input)
}
pub(crate) fn word1<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., (AsChar::is_alphanum, '_')).parse_next(input)
}
