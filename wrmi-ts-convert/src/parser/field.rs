use winnow::{
    ascii::multispace1,
    combinator::{alt, delimited, opt, separated_pair, terminated},
    PResult, Parser,
};

use super::{
    ts_type::TsType,
    util::{quote_backslash_escape, token, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Field<'a> {
    pub name: FieldName<'a>,
    pub readonly: bool,
    pub optional: bool,
    pub ty: TsType<'a>,
}
impl<'a> Parsable<'a> for Field<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        separated_pair(
            (
                opt(terminated("readonly", multispace1)),
                FieldName::parse,
                opt(token('?')),
            ),
            token(':'),
            TsType::parse,
        )
        .parse_next(input)
        .map(|((readonly, name, optional), ty)| Self {
            readonly: readonly.is_some(),
            name,
            optional: optional.is_some(),
            ty,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FieldName<'a> {
    Name(&'a str),
    Wildcard { name: &'a str, ty: TsType<'a> },
}
impl<'a> Parsable<'a> for FieldName<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        alt((
            word1.map(Self::Name),
            quote_backslash_escape('"').map(Self::Name),
            delimited(
                token('['),
                separated_pair(word1, token(':'), TsType::parse),
                token(']'),
            )
            .map(|(name, ty)| Self::Wildcard { name, ty }),
        ))
        .parse_next(input)
    }
}
