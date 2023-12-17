mod step;
mod tests;
use anyhow::Result;
use std::fs;
use step::Step;

use crate::step::Step::{Add, Remove};

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

fn task1(s: &str) -> usize {
    s.split(',').map(hash).sum()
}

fn task2(s: &str) -> usize {
    s.split(',')
        .map(|s| s.parse::<Step>().unwrap())
        .fold(
            vec![Vec::new(); 256],
            |mut acc: Vec<Vec<(String, u32)>>, step: Step| {
                match step {
                    Remove { label } => acc[hash(&label)].retain(|(l, _)| l != &label),
                    Add { label, foc_l } => {
                        match acc[hash(&label)].iter_mut().find(|(l, _)| l == &label) {
                            Some((_, l)) => *l = foc_l,
                            None => acc[hash(&label)].push((label, foc_l)),
                        }
                    }
                }
                acc
            },
        )
        .into_iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .map(move |(j, (_, foc_l))| (i + 1) * (j + 1) * foc_l as usize)
        })
        .sum()
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    println!("Task 1: {}", task1(s.trim()));
    println!("Task 2: {}", task2(s.trim()));

    Ok(())
}
