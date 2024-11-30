mod parsers;
mod structs;
use prehash::Prehashed;
use rayon::prelude::*;
use std::fs;

use indicatif::ParallelProgressIterator;
use structs::{Category::*, Part, Workflows};

fn process_part(workflows: &Workflows, part: Part, key: &Prehashed<&str>) -> bool {
    match workflows[key].iter().find_map(|w| w.process(part)).unwrap() {
        structs::ProcessResult::Accepted => true,
        structs::ProcessResult::Rejected => false,
        structs::ProcessResult::Send(key) => process_part(workflows, part, key),
    }
}

fn task(
    workflows: &Workflows,
    parts: impl Iterator<Item = Part> + Send,
    n: u64,
    start: &Prehashed<&str>,
) -> u64 {
    parts
        .par_bridge()
        .progress_count(n)
        .filter(|p| process_part(workflows, *p, start))
        .map(|p| p[X] + p[M] + p[A] + p[S])
        .sum()
}

fn gen_all_parts() -> impl Iterator<Item = Part> {
    (1..=4000).flat_map(|x| {
        (1..=4000).flat_map(move |m| {
            (1..=4000).flat_map(move |a| (1..=4000).map(move |s| Part::new([x, m, a, s])))
        })
    })
}

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let (workflows, parts, start) = parsers::parse_input(&s).unwrap();

    println!(
        "Task 1: {}",
        task(
            &workflows,
            parts.iter().copied(),
            parts.len() as u64,
            &start
        )
    );
    println!(
        "Task 2: {}",
        task(&workflows, gen_all_parts(), 4000_u64.pow(4), &start)
    );
}
