use winnow::{
    ascii::{dec_int, hex_uint},
    combinator::{alt, delimited, opt, preceded, repeat, separated0, separated_pair},
    PResult, Parser,
};

use super::{
    comment::WithComment,
    expr::Expr,
    generic::GenericArgs,
    member::Member,
    method::MethodArg,
    util::{quote_backslash_escape, token, token_word, word1, Parsable},
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
    IntLit {
        int: i32,
    },
    BoolLit {
        bool: bool,
    },
    Array {
        item: Box<TsType<'a>>,
    },
    FixedArray {
        types: Vec<TsType<'a>>,
    },
    ObjectIndex {
        obj_index: Box<(TsType<'a>, Expr<'a>)>,
    },
    Interface {
        members: Vec<WithComment<'a, Member<'a>>>,
    },
    KeyOf {
        ty: Box<TsType<'a>>,
    },
    TypeOf {
        expr: Expr<'a>,
    },
    PatternString {
        pattern: &'a str,
    },
    TsIs {
        expr: Expr<'a>,
        ty: Box<TsType<'a>>,
    },
    Namespaced {
        ns: &'a str,
        ty: NamedType<'a>,
    },
}

impl<'a> TsType<'a> {
    fn parse_single(input: &mut &'a str) -> PResult<Self> {
        let mut res = alt((
            // keyof
            preceded(token_word("keyof"), TsType::parse).map(|ty| Self::KeyOf { ty: Box::new(ty) }),
            // typeof
            preceded(token_word("typeof"), Expr::parse).map(|expr| Self::TypeOf { expr }),
            // arrow function
            Self::parse_arow_func,
            // parenthesis
            delimited(token('('), Self::parse, token(')')),
            // string literal
            quote_backslash_escape('"').map(|str| Self::StringLit { str }),
            // pattern string
            quote_backslash_escape('`').map(|content| Self::PatternString { pattern: content }),
            // int literal
            preceded("0x", hex_uint).map(|int: u32| Self::IntLit { int: int as _ }),
            dec_int.map(|int| Self::IntLit { int }),
            separated_pair(Expr::parse, token_word("is"), TsType::parse).map(|(expr, ty)| {
                Self::TsIs {
                    expr,
                    ty: Box::new(ty),
                }
            }),
            separated_pair(word1, token('.'), NamedType::parse)
                .map(|(ns, ty)| Self::Namespaced { ns, ty }),
            // boolean literal
            "false".map(|_| Self::BoolLit { bool: false }),
            "true".map(|_| Self::BoolLit { bool: true }),
            // named
            NamedType::parse.map(|ty| Self::Named { ty }),
            // fixed array
            delimited(
                token('['),
                separated0(TsType::parse, token(',')),
                token(']'),
            )
            .map(|types| Self::FixedArray { types }),
            // interface
            delimited(
                token('{'),
                repeat(0.., WithComment::<Member>::parse),
                token('}'),
            )
            .map(|members| Self::Interface { members }),
        ))
        .parse_next(input)?;

        // array and index
        while let Ok(bracket) =
            delimited(token('['), opt(Expr::parse), token(']')).parse_next(input)
        {
            res = match bracket {
                Some(item) => TsType::ObjectIndex {
                    obj_index: Box::new((res, item)),
                },
                None => TsType::Array {
                    item: Box::new(res),
                },
            };
        }

        Ok(res)
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
