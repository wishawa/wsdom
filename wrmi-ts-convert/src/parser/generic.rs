use winnow::{
    combinator::{delimited, opt, preceded, separated1},
    PResult, Parser,
};

use super::{
    ts_type::TsType,
    util::{token, token_word, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct GenericsDeclaration<'a> {
    pub args: Vec<GenericsDeclarationArg<'a>>,
}

impl<'a> Parsable<'a> for GenericsDeclaration<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        delimited(
            token('<'),
            separated1(GenericsDeclarationArg::parse, token(',')),
            token('>'),
        )
        .parse_next(input)
        .map(|args| Self { args })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericsDeclarationArg<'a> {
    pub name: &'a str,
    pub extends: Option<TsType<'a>>,
    pub default: Option<TsType<'a>>,
}
impl<'a> Parsable<'a> for GenericsDeclarationArg<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        (
            word1,
            opt(preceded(token_word("extends"), TsType::parse)),
            opt(preceded(token('='), TsType::parse)),
        )
            .parse_next(input)
            .map(|(name, extends, default)| Self {
                name,
                extends,
                default,
            })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct GenericArgs<'a> {
    pub args: Vec<TsType<'a>>,
}

impl<'a> Parsable<'a> for GenericArgs<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        delimited(
            token('<'),
            separated1(TsType::parse, token(',')),
            token('>'),
        )
        .map(|args| Self { args })
        .parse_next(input)
    }
}
