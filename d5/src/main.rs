#![feature(extract_if)]
use anyhow::{anyhow, Result};
use indicatif::ProgressIterator;
use rayon::prelude::*;
use std::fs;

fn parse_seeds(s: &str) -> Result<Vec<i64>> {
    s.strip_prefix("seeds: ")
        .ok_or(anyhow!("Invalid input"))?
        .split_ascii_whitespace()
        .map(|x| Ok(x.parse::<i64>()?))
        .collect()
}

fn transform<'a>(seeds: &'a mut Vec<i64>, s: &str) -> impl Iterator<Item = i64> + 'a {
    let [dst, src, len] = s
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    seeds
        .extract_if(move |&mut x| 0 <= x - src && x - src < len)
        .map(move |x| x + dst - src)
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let mut lines = s.lines();

    let seeds = parse_seeds(lines.next().ok_or(anyhow!("Invalid input"))?)?;

    println!("Answer for task 1: {}", task(lines.clone(), seeds.clone()));

    let seeds = seeds
        .as_parallel_slice()
        .chunks(2)
        .progress()
        .flat_map(|c| c[0]..c[0] + c[1])
        .collect::<Vec<_>>();
    println!("Length of seeds: {}", seeds.len());
    println!("Answer for task 2: {}", task(lines, seeds));
    Ok(())
}

fn task(lines: std::str::Lines<'_>, mut seeds: Vec<i64>) -> i64 {
    let mut new_seeds = Vec::new();
    let n = lines.clone().count();
    for l in lines.progress_count(n as u64) {
        if l.is_empty() {
            continue;
        }
        if l.ends_with("map:") {
            new_seeds.extend_from_slice(&seeds);
            seeds = new_seeds;
            new_seeds = Vec::new();
            continue;
        }
        new_seeds.extend(transform(&mut seeds, l));
    }

    new_seeds.extend_from_slice(&seeds);
    new_seeds.into_par_iter().min().unwrap()
}
