use winnow::{combinator::alt, PResult, Parser};

use super::{
    declare_function::DeclareFunction, declare_var::DeclareVar, interface::Interface,
    namespace::Namespace, type_alias::TypeAlias, util::Parsable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Item<'a> {
    Interface(Interface<'a>),
    DeclareVar(DeclareVar<'a>),
    DeclareFunction(DeclareFunction<'a>),
    TypeAlias(TypeAlias<'a>),
    Namespace(Namespace<'a>),
}

impl<'a> Parsable<'a> for Item<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        alt((
            Interface::parse.map(Self::Interface),
            DeclareVar::parse.map(Self::DeclareVar),
            DeclareFunction::parse.map(Self::DeclareFunction),
            TypeAlias::parse.map(Self::TypeAlias),
            Namespace::parse.map(Self::Namespace),
        ))
        .parse_next(input)
    }
}
