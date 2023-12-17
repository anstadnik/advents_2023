use anyhow::{anyhow, Result};
use std::str::FromStr;
use winnow::ascii::dec_uint;
use winnow::combinator::alt;
use winnow::error::{ContextError, ParseError};
use winnow::prelude::*;
use winnow::token::take_till;

#[derive(Debug, PartialEq)]
pub enum Step {
    Remove { label: String },
    Add { label: String, foc_l: u32 },
}

impl FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let f_e = |e: ParseError<_, ContextError>| anyhow!(e.to_string());
        let p_label = |s: &mut _| {
            take_till(1.., ('=', '-'))
                .map(|s: &str| s.to_string())
                .parse_next(s)
        };
        let p_remove = (p_label, '-').map(|(label, _)| Step::Remove { label });
        let p_add = (p_label, '=', dec_uint).map(|(label, _, foc_l)| Step::Add { label, foc_l });
        alt((p_remove, p_add)).parse(s).map_err(f_e)
    }
}
