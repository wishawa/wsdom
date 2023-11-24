use winnow::{
    combinator::{opt, preceded},
    prelude::PResult,
    Parser,
};

use super::util::{token, word1, Parsable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Expr<'a> {
    Name { name: &'a str },
    Dot { left: Box<Expr<'a>>, right: &'a str },
}

impl<'a> Parsable<'a> for Expr<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        (word1, opt(preceded(token('.'), word1)))
            .map(|(left, after_dot)| match after_dot {
                Some(right) => Self::Dot {
                    left: Box::new(Self::Name { name: left }),
                    right,
                },
                None => Self::Name { name: left },
            })
            .parse_next(input)
    }
}
