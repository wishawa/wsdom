use winnow::{
    ascii::multispace0,
    combinator::{delimited, peek, terminated},
    stream::AsChar,
    token::{any, none_of, take_while},
    PResult, Parser,
};

pub(crate) trait Parsable<'a>: Sized {
    fn parse(input: &mut &'a str) -> PResult<Self>;
}

pub(crate) fn token_word<'a, O, S: Parser<&'a str, O, winnow::error::ContextError>>(
    s: S,
) -> impl Parser<&'a str, O, winnow::error::ContextError> {
    delimited(
        multispace0,
        terminated(s, peek(none_of((AsChar::is_alphanum, '_')))),
        multispace0,
    )
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

pub(crate) fn quote_backslash_escape<'a>(
    mut quote: char,
) -> impl Parser<&'a str, &'a str, winnow::error::ContextError> {
    move |input: &mut &'a str| -> PResult<&'a str> {
        let _first = quote.parse_next(input)?;
        let mut escaped = false;
        loop {
            escaped = match (escaped, any.parse_next(input)) {
                (true, Ok(_)) => false,
                (false, Ok('\\')) => true,
                (false, Ok(ch)) if ch == quote => return Ok(""),
                (false, Ok(_)) => false,
                (_, Err(e)) => return Err(e),
            }
        }
    }
}
