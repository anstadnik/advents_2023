#![feature(iter_map_windows)]
use anyhow::Result;
use dyn_clone::DynClone;
use std::fs;

trait IteratorClone: Iterator + DynClone {}
impl<T> IteratorClone for T where T: Iterator + DynClone {}
dyn_clone::clone_trait_object!(<T> IteratorClone<Item = T>);

fn find_next(it: Box<dyn IteratorClone<Item = i64> + '_>) -> i64 {
    (it.clone().any(|n| n != 0))
        .then(|| it.clone().last().unwrap() + find_next(Box::new(it.map_windows(|[a, b]| b - a))))
        .unwrap_or_default()
}
fn find_prev(it: Box<dyn IteratorClone<Item = i64> + '_>) -> i64 {
    (it.clone().any(|n| n != 0))
        .then(|| it.clone().next().unwrap() - find_prev(Box::new(it.map_windows(|[a, b]| b - a))))
        .unwrap_or_default()
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let f = |l: &str| l.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let nums: Vec<Vec<i64>> = s.lines().map(f).collect();

    println!(
        "Answer for task 1: {}",
        nums.iter()
            .map(|n| find_next(Box::new(n.iter().cloned())))
            .sum::<i64>()
    );

    println!(
        "Answer for task 2: {}",
        nums.iter()
            .map(|n| find_prev(Box::new(n.iter().cloned())))
            .sum::<i64>()
    );

    Ok(())
}
