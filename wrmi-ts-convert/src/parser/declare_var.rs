use winnow::{
    combinator::{delimited, repeat, separated_pair},
    PResult, Parser,
};

use super::{
    comment::WithComment,
    member::Member,
    util::{token, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeclareVar<'a> {
    pub name: &'a str,
    pub members: Vec<WithComment<'a, Member<'a>>>,
}

impl<'a> Parsable<'a> for DeclareVar<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        delimited(
            (token("declare"), token("var")),
            separated_pair(
                word1,
                token(':'),
                delimited(
                    token('{'),
                    repeat(0.., WithComment::<Member>::parse),
                    token('}'),
                ),
            ),
            token(';'),
        )
        .map(|(name, members)| Self { name, members })
        .parse_next(input)
    }
}
