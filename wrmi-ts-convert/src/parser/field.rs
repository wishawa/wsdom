use winnow::{
    ascii::multispace1,
    combinator::{alt, delimited, opt, separated_pair, terminated},
    PResult, Parser,
};

use super::{
    ts_type::TsType,
    util::{token, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Field<'a> {
    pub name: &'a str,
    pub readonly: bool,
    pub optional: bool,
    pub ty: TsType<'a>,
}
impl<'a> Parsable<'a> for Field<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        separated_pair(
            (
                opt(terminated("readonly", multispace1)),
                alt((word1, delimited('"', word1, '"'))),
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
