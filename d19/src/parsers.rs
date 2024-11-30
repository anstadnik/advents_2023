use crate::structs;
use anyhow::{anyhow, Result};
use prehash::DefaultPrehasher;
use prehash::Prehashed;
use prehash::Prehasher;
use structs::Category::*;
use structs::Op::*;
use structs::{Condition, Workflow};
use winnow::ascii::dec_uint;
use winnow::combinator::terminated;
use winnow::combinator::{delimited, dispatch, fail, opt, separated, separated_pair};
use winnow::error::StrContext;
use winnow::error::{ContextError, ParseError};
use winnow::token::take;
use winnow::{combinator::success, token::take_while};
use winnow::{prelude::*, seq, token::any};

pub(crate) fn parse_category(s: &mut &str) -> PResult<structs::Category> {
    take(1_u32).parse_to().parse_next(s)
}

pub(crate) fn parse_workflow<'a>(
    s: &'a str,
    ph: &DefaultPrehasher,
) -> Result<(Prehashed<&'a str>, Vec<structs::Workflow<'a>>)> {
    let parse_key = take_while(1.., |c: char| c.is_ascii_alphabetic())
        .map(|s| ph.prehash(s))
        .context(StrContext::Label("Key"));
    let mut parse_op = dispatch! {
        any;
        '<' => success(Lt),
        '>' => success(Gt),
        _ => fail
    }
    .context(StrContext::Label("Op"));
    let mut parse_cond = opt(terminated(
        seq! { Condition {
            category: parse_category,
            op: parse_op,
            tgt: dec_uint
            }
        },
        ':',
    ))
    .context(StrContext::Label("Condition"));
    let mut parse_dest = take_while(1.., |c: char| c.is_ascii_alphabetic())
        .map(|s| ph.prehash(s))
        .context(StrContext::Label("Destination"));
    let parse_workflow = seq! { Workflow {
        cond: parse_cond,
        dest: parse_dest
        }
    }
    .context(StrContext::Label("Workflow"));

    (
        parse_key,
        delimited(
            '{',
            separated(1.., parse_workflow, ',').context(StrContext::Label("Separated workflow")),
            '}',
        )
        .context(StrContext::Label("Delimited workflow")),
    )
        .parse(s)
        .map_err(|e: ParseError<&str, ContextError>| anyhow!(e.to_string()))
}

pub(crate) fn parse_part(s: &str) -> Result<structs::Part> {
    delimited(
        '{',
        separated(
            4,
            separated_pair(
                parse_category.context(StrContext::Label("Category")),
                '=',
                dec_uint.context(StrContext::Label("Unsigned integer")),
            )
            .context(StrContext::Label("Separated pair")),
            ',',
        )
        .context(StrContext::Label("Separated")),
        '}',
    )
    .context(StrContext::Label("Delimited"))
    .parse(s)
    .map(|v: Vec<(structs::Category, u64)>| {
        assert!(v.iter().map(|(c, _)| c).eq(&[X, M, A, S]));
        let a: [(structs::Category, u64); 4] = v.try_into().unwrap();
        structs::Part::new(a.map(|(_, u)| u))
    })
    .map_err(|e: ParseError<&str, ContextError>| anyhow!(e.to_string()))
}

pub(crate) fn parse_input(
    s: &str,
) -> Result<(structs::Workflows, Vec<structs::Part>, Prehashed<&str>)> {
    let (map, input) = s.split_once("\n\n").ok_or(anyhow!("Invalid input"))?;
    let ph = DefaultPrehasher::new();
    Ok((
        map.lines()
            .map(|s| parse_workflow(s, &ph))
            .collect::<Result<_>>()?,
        input.lines().map(parse_part).collect::<Result<_>>()?,
        ph.prehash("in"),
    ))
}
