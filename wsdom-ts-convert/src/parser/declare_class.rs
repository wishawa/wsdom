use winnow::{
    combinator::{delimited, opt, preceded, repeat},
    prelude::PResult,
    Parser,
};

use super::{
    comment::WithComment,
    generic::GenericsDeclaration,
    member::Member,
    util::{token, token_word, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeclareClass<'a> {
    pub name: &'a str,
    pub generics: GenericsDeclaration<'a>,
    pub members: Vec<WithComment<'a, Member<'a>>>,
}

impl<'a> Parsable<'a> for DeclareClass<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        preceded(
            (token_word("declare"), token_word("class")),
            (
                word1,
                opt(GenericsDeclaration::parse),
                delimited(
                    token('{'),
                    repeat(0.., WithComment::<Member>::parse),
                    token('}'),
                ),
            ),
        )
        .map(|(name, generics, members)| Self {
            name,
            generics: generics.unwrap_or_default(),
            members,
        })
        .parse_next(input)
    }
}
