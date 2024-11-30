use std::ops::Index;
use std::str::FromStr;
use prehash::{Prehashed, PrehashedMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Part {
    pub(crate) categories: [u64; 4],
}

impl Part {
    pub(crate) fn new(categories: [u64; 4]) -> Self {
        Self { categories }
    }
}

impl Index<Category> for Part {
    type Output = u64;

    fn index(&self, index: Category) -> &Self::Output {
        &self.categories[index as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Op {
    Lt,
    Gt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Condition {
    pub(crate) category: Category,
    pub(crate) op: Op,
    pub(crate) tgt: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Workflow<'a> {
    pub(crate) cond: Option<Condition>,
    pub(crate) dest: Prehashed<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessResult<'a> {
    Send(&'a Prehashed<&'a str>),
    Accepted,
    Rejected,
}

impl Workflow<'_> {
    pub fn process(&self, part: Part) -> Option<ProcessResult> {
        let passes = match self.cond {
            Some(Condition {
                category,
                op: Op::Lt,
                tgt,
            }) => part[category] < tgt,
            Some(Condition {
                category,
                op: _,
                tgt,
            }) => part[category] > tgt,
            None => true,
        };
        passes.then_some(match *self.dest {
            "A" => ProcessResult::Accepted,
            "R" => ProcessResult::Rejected,
            _ => ProcessResult::Send(&self.dest),
        })
    }
}

pub(crate) type Workflows<'a> = PrehashedMap<&'a str, Vec<Workflow<'a>>>;
