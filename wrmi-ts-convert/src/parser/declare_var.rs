use winnow::{
    combinator::{alt, delimited, opt, separated_pair},
    PResult, Parser,
};

use super::{
    ts_type::TsType,
    util::{token, token_word, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeclareVar<'a> {
    pub name: &'a str,
    pub ty: TsType<'a>,
}

impl<'a> Parsable<'a> for DeclareVar<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        delimited(
            (
                opt(token_word("declare")),
                token_word(alt(("var", "const"))),
            ),
            separated_pair(word1, token(':'), TsType::parse),
            token(';'),
        )
        .map(|(name, ty)| Self { name, ty })
        .parse_next(input)
    }
}
