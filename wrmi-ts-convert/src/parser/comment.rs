use winnow::{
    ascii::{line_ending, multispace0, not_line_ending},
    combinator::{alt, delimited, opt},
    token::take_until0,
    PResult, Parser,
};

use super::util::Parsable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Comment<'a> {
    pub source: &'a str,
}

impl<'a> Parsable<'a> for Comment<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        alt((
            ("/*", take_until0("*/"), "*/").recognize(),
            ("//", not_line_ending, line_ending).recognize(),
        ))
        .parse_next(input)
        .map(|source| Self { source })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WithComment<'a, T> {
    pub comment: Option<Comment<'a>>,
    pub data: T,
}

impl<'a, T: Parsable<'a>> Parsable<'a> for WithComment<'a, T> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        (
            opt(delimited(multispace0, Comment::parse, multispace0)),
            T::parse,
        )
            .map(|(comment, data)| Self { comment, data })
            .parse_next(input)
    }
}
