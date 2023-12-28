use winnow::{
    combinator::{delimited, preceded, repeat},
    prelude::PResult,
    Parser,
};

use super::{
    comment::WithComment,
    item::Item,
    util::{token, token_word, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Namespace<'a> {
    name: &'a str,
    items: Vec<WithComment<'a, Item<'a>>>,
}

impl<'a> Parsable<'a> for Namespace<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        (
            preceded((token_word("declare"), token_word("namespace")), word1),
            delimited(token('{'), repeat(0.., WithComment::<Item>::parse), '}'),
        )
            .map(|(name, items)| Self { name, items })
            .parse_next(input)
    }
}
