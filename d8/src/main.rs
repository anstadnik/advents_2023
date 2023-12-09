#![allow(dead_code)]
use anyhow::{anyhow, Result};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use num::{BigUint, Integer};
use prehash::{DefaultPrehasher, Prehashed, PrehashedMap, Prehasher};
use std::fs;
use std::iter::successors;
use winnow::error::{ContextError, ParseError};
use winnow::{prelude::*, token::take};

fn parse_node(s: &str) -> Result<(&str, (&str, &str))> {
    let pattern = (take(3usize), " = (", take(3usize), ", ", take(3usize), ')');
    pattern
        .map(|(name, _, x, _, y, _)| (name, (x, y)))
        .parse(s)
        .map_err(|e: ParseError<&str, ContextError>| anyhow!(e.to_string()))
}

// type M<'a> = HashMap<&'a str, (&'a str, &'a str)>;
type M<'a> = PrehashedMap<&'a str, (Prehashed<&'a str>, Prehashed<&'a str>)>;

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let mut it = s.lines();
    let path = it.next().ok_or(anyhow!("No path"))?;
    let ph = DefaultPrehasher::new();
    let mapping: M = it
        .skip(1)
        .map(parse_node)
        .map(|r| r.map(|(n, (l, r))| (ph.prehash(n), (ph.prehash(l), ph.prehash(r)))))
        .collect::<Result<_>>()?;

    println!(
        "Answer to task 1: {}",
        task_1(path, &mapping, ph.prehash("AAA"), |s| s == "ZZZ")
    );
    // println!("Answer to task 2: {}", task_2(path, &mapping));
    println!("Answer to task 2: {}", task_2_stupid(path, &mapping));

    Ok(())
}

fn task_1(path: &str, mapping: &M, start: Prehashed<&str>, is_end: impl Fn(&str) -> bool) -> usize {
    let mut steps = path.chars().cycle();
    successors(Some(start), |state| {
        (!is_end(state)).then_some(match steps.next().unwrap() == 'L' {
            true => mapping[state].0,
            false => mapping[state].1,
        })
    })
    .count()
        - 1
}

fn task_2(path: &str, mapping: &M) -> usize {
    let mut states: Vec<_> = mapping
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .collect();

    let style = ProgressStyle::default_spinner()
        .template("{spinner:.green} [{elapsed}] {human_pos} {per_sec}")
        .unwrap();
    let progress = ProgressBar::new_spinner().with_style(style);

    for (n, step) in path.chars().cycle().enumerate().progress_with(progress) {
        if states.iter().all(|s| s.ends_with('Z')) {
            return n;
        }
        if step == 'L' {
            states.iter_mut().for_each(|s| *s = mapping[s].0)
        } else {
            states.iter_mut().for_each(|s| *s = mapping[s].1)
        };
    }
    unreachable!()
}

fn task_2_stupid(path: &str, mapping: &M) -> BigUint {
    mapping
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| BigUint::from(task_1(path, mapping, *k, |s| s.ends_with('Z'))))
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}
