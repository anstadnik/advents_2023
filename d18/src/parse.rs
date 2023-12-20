use crate::structs::Direction::{self, *};
use anyhow::anyhow;
use winnow::ascii::{dec_uint, hex_uint};
use winnow::error::{ContextError, ParseError};
use winnow::token::{one_of, take, take_until1};
use winnow::Parser;

use anyhow::Result;

pub type Step = (Direction, u64);
pub fn parse_line1(l: &str) -> Result<Step> {
    let parse_direction = one_of(['U', 'D', 'L', 'R']).map(|c| match c {
        'U' => Up,
        'D' => Down,
        'L' => Left,
        'R' => Right,
        _ => unreachable!(),
    });
    let parse_color = take(6_usize);
    (parse_direction, ' ', dec_uint, " (#", parse_color, ')')
        .map(|(d, _, l, _, _, _)| (d, l))
        .parse(l)
        .map_err(|e: ParseError<&str, ContextError>| anyhow!(e.to_string()))
}

pub fn parse_line2(l: &str) -> Result<Step> {
    let parse_direction = hex_uint.map(|d| match d {
        0_u32 => Right,
        1 => Down,
        2 => Left,
        3 => Up,
        _ => unreachable!(),
    });
    (
        take(1_usize),
        ' ',
        take_until1(" "),
        " (#",
        take(5_usize).and_then(hex_uint),
        parse_direction,
        ')',
    )
        .map(|(_, _, _, _, l, d, _)| (d, l))
        .parse(l)
        .map_err(|e: ParseError<&str, ContextError>| anyhow!(e.to_string()))
}

pub fn parse_input1(s: &str) -> Result<Vec<Step>> {
    s.lines().map(parse_line1).collect()
}

pub fn parse_input2(s: &str) -> Result<Vec<Step>> {
    s.lines().map(parse_line2).collect()
}
