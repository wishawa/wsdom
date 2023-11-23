use winnow::{
    combinator::{delimited, opt, separated0, separated_pair},
    Parser,
};

use super::{
    generic::GenericsDeclaration,
    method::MethodArg,
    ts_type::TsType,
    util::{token, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeclareFunction<'a> {
    pub name: &'a str,
    pub generics: GenericsDeclaration<'a>,
    pub args: Vec<MethodArg<'a>>,
    pub ret: TsType<'a>,
}

impl<'a> Parsable<'a> for DeclareFunction<'a> {
    fn parse(input: &mut &'a str) -> winnow::PResult<Self> {
        delimited(
            (token("declare"), token("function")),
            (
                word1,
                opt(GenericsDeclaration::parse),
                separated_pair(
                    delimited(
                        token('('),
                        separated0(MethodArg::parse, token(',')),
                        token(')'),
                    ),
                    token(':'),
                    TsType::parse,
                ),
            ),
            token(';'),
        )
        .parse_next(input)
        .map(|(name, generics, (args, ret))| Self {
            name,
            generics: generics.unwrap_or_default(),
            args,
            ret,
        })
    }
}
