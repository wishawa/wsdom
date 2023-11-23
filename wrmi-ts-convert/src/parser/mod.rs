use winnow::{
    ascii::line_ending,
    combinator::{alt, delimited, repeat},
    token::take_until0,
    PResult, Parser,
};

use crate::parser::util::token;

use self::{
    comment::{Comment, WithComment},
    declare_function::DeclareFunction,
    declare_var::DeclareVar,
    interface::Interface,
    util::Parsable,
};

mod comment;
mod declare_function;
mod declare_var;
mod field;
mod generic;
mod interface;
mod member;
mod method;
mod ts_type;
mod util;

pub(crate) fn parse_all<'a>(input: &mut &'a str) -> PResult<Vec<WithComment<'a, Item<'a>>>> {
    loop {
        if Comment::parse.parse_next(input).is_err() {
            break;
        }
    }
    let mut out = Vec::new();
    while !input.is_empty() {
        out.push(WithComment::<Item>::parse.parse_next(input)?);
    }
    Ok(out)
}

pub(crate) fn parse_imports<'a>(input: &mut &'a str) -> PResult<Vec<&'a str>> {
    fn parse_import<'a>(input: &mut &'a str) -> PResult<&'a str> {
        delimited(
            (
                token("///"),
                token("<"),
                token("reference"),
                token("lib"),
                token('='),
            ),
            delimited('"', take_until0("\""), '"'),
            token("/>"),
        )
        .parse_next(input)
    }
    repeat(0.., parse_import).parse_next(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Item<'a> {
    Interface(Interface<'a>),
    DeclareVar(DeclareVar<'a>),
    DeclareFunction(DeclareFunction<'a>),
}

impl<'a> Parsable<'a> for Item<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        alt((
            Interface::parse.map(Self::Interface),
            DeclareVar::parse.map(Self::DeclareVar),
            DeclareFunction::parse.map(Self::DeclareFunction),
        ))
        .parse_next(input)
    }
}

mod tests {

    #[test]
    fn basic_test() {
        use super::{
            comment::{Comment, WithComment},
            field::Field,
            member::Member,
            method::{Method, MethodName},
            ts_type::{NamedType, TsType},
            DeclareVar, Item, Parsable,
        };
        use winnow::Parser;
        let parsed: WithComment<Item> = Parsable::parse
            .parse(
                &mut r#"
// Hello this is a test
declare var Element: {
    prototype: Element;
    new(): Element;
};
            "#,
            )
            .unwrap();
        assert_eq!(
            parsed,
            WithComment {
                comment: Some(Comment {
                    source: "// Hello this is a test\n"
                }),
                data: Item::DeclareVar(DeclareVar {
                    name: "Element",
                    members: vec![
                        WithComment {
                            comment: None,
                            data: Member::Field(Field {
                                readonly: false,
                                name: "prototype",
                                optional: false,
                                ty: TsType::Named {
                                    ty: NamedType {
                                        generic: Default::default(),
                                        name: "Element"
                                    }
                                }
                            })
                        },
                        WithComment {
                            comment: None,
                            data: Member::Method(Method {
                                name: MethodName::Constructor,
                                generics: Default::default(),
                                args: vec![],
                                ret: TsType::Named {
                                    ty: NamedType {
                                        name: "Element",
                                        generic: Default::default()
                                    }
                                }
                            })
                        }
                    ]
                })
            }
        )
    }
    #[test]
    fn another_test() {
        use super::{Item, Parsable, WithComment};
        use winnow::{combinator::repeat, Parser};

        let _: Vec<_> = repeat(0.., WithComment::<Item>::parse)
            .parse(
                &mut r#"
/**
 * Hahaha
 */
declare var Element: {
    prototype: Element;
    new(): Element;
};

/** [MDN Reference](https://developer.mozilla.org/docs/Web/API/SubmitEvent) */
interface SubmitEvent extends Event {
    /**
     * Returns the element representing the submit button that triggered the form submission, or null if the submission was not triggered by a button.
     *
     * [MDN Reference](https://developer.mozilla.org/docs/Web/API/SubmitEvent/submitter)
     */
    readonly submitter: HTMLElement | null;
}

interface UnderlyingByteSource {
    autoAllocateChunkSize?: number;
    cancel?: UnderlyingSourceCancelCallback;
    pull?: (controller: ReadableByteStreamController) => void | PromiseLike<void>;
    start?: (controller: ReadableByteStreamController) => any;
    type: "bytes";
}
            "#,
            )
            .unwrap();
    }
}
