use winnow::{
    combinator::{alt, delimited, opt, preceded, separated0, separated_pair},
    PResult, Parser,
};

use super::{
    generic::GenericArgs,
    method::MethodArg,
    util::{token, word1, Parsable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TsType<'a> {
    Named {
        ty: NamedType<'a>,
    },
    ArrowFunc {
        args: Vec<MethodArg<'a>>,
        ret: Option<Box<TsType<'a>>>,
    },
    Union {
        pair: Box<(TsType<'a>, TsType<'a>)>,
    },
    Intesect {
        pair: Box<(TsType<'a>, TsType<'a>)>,
    },
    StringLit {
        str: &'a str,
    },
    Array {
        item: Box<TsType<'a>>,
    },
    FixedArray {
        types: Vec<TsType<'a>>,
    },
}

impl<'a> TsType<'a> {
    fn parse_single(input: &mut &'a str) -> PResult<Self> {
        (
            alt((
                NamedType::parse.map(|ty| Self::Named { ty }),
                Self::parse_arow_func,
                delimited(token('('), Self::parse_arow_func, token(')')),
                delimited('"', word1, '"') //pub TODO:actually allow any string literal
                    .recognize()
                    .map(|str| Self::StringLit { str }),
                delimited(
                    token('['),
                    separated0(TsType::parse, token(',')),
                    token(']'),
                )
                .map(|types| Self::FixedArray { types }),
            )),
            opt((token('['), token(']'))),
        )
            .parse_next(input)
            .map(|(main_res, brackets)| match brackets {
                Some(_) => Self::Array {
                    item: Box::new(main_res),
                },
                None => main_res,
            })
    }
    fn parse_arow_func(input: &mut &'a str) -> PResult<Self> {
        (
            delimited(
                token('('),
                separated0(MethodArg::parse, token(',')),
                token(')'),
            ),
            opt(preceded(token("=>"), TsType::parse)),
        )
            .map(|(args, ret)| Self::ArrowFunc {
                args,
                ret: ret.map(Box::new),
            })
            .parse_next(input)
    }
}
impl<'a> Parsable<'a> for TsType<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        alt((
            separated_pair(TsType::parse_single, token('|'), TsType::parse).map(|pair| {
                Self::Union {
                    pair: Box::new(pair),
                }
            }),
            separated_pair(TsType::parse_single, token('&'), TsType::parse).map(|pair| {
                Self::Intesect {
                    pair: Box::new(pair),
                }
            }),
            TsType::parse_single,
        ))
        .parse_next(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NamedType<'a> {
    pub name: &'a str,
    pub generic: GenericArgs<'a>,
}
impl<'a> Parsable<'a> for NamedType<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        (word1, opt(GenericArgs::parse))
            .parse_next(input)
            .map(|(name, generic_args)| Self {
                name,
                generic: generic_args.unwrap_or_default(),
            })
    }
}
