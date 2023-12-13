#![feature(iter_intersperse)]
mod tests;
use anyhow::Result;
use indicatif::ParallelProgressIterator;
use memoize::memoize;
use rayon::prelude::*;
use std::{fs, iter::repeat};

#[memoize]
fn count_arrangemets(mut symbols: Vec<u8>, nums: Vec<usize>) -> usize {
    if symbols.is_empty() {
        return nums.is_empty() as usize;
    }

    if symbols[0] == b'.' {
        return count_arrangemets(symbols[1..].to_vec(), nums);
    }

    if symbols[0] == b'#' {
        let Some(&n) = nums.first() else {
            return 0;
        };
        if n <= symbols.len()
            && symbols.iter().take(n).all(|c| *c != b'.')
            && symbols.get(n) != Some(&b'#')
        {
            let i = symbols.len().min(n + 1);
            return count_arrangemets(symbols[i..].to_vec(), nums[1..].to_vec());
        } else {
            return 0;
        }
    }

    let s = nums.iter().sum();
    if symbols
        .iter()
        .filter(|c| matches!(**c, b'?' | b'#'))
        .count()
        < s
        || symbols.iter().filter(|c| **c == b'#').count() > s
    {
        return 0;
    }

    let mut ret = count_arrangemets(symbols[1..].to_vec(), nums.clone());
    symbols[0] = b'#';
    ret += count_arrangemets(symbols.clone(), nums);
    symbols[0] = b'?';
    ret
}

fn expand(symbols: &[u8], nums: &[usize]) -> (Vec<u8>, Vec<usize>) {
    let symbols = repeat(symbols.iter())
        .take(5)
        .intersperse([b'?'].iter())
        .flatten()
        .cloned()
        .collect();

    let nums = nums.iter().cycle().take(nums.len() * 5).cloned().collect();
    (symbols, nums)
}

fn main() -> Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let mut inp: Vec<(_, Vec<usize>)> = s
        .lines()
        .map(|l| {
            let (symbols, nums) = l.split_once(' ').unwrap();
            let nums = nums.split(',').map(|n| n.parse().unwrap()).collect();
            (symbols.to_owned().into_bytes(), nums)
        })
        .collect();

    let ans1 = inp
        .par_iter_mut()
        .progress()
        .map(|(symbols, nums)| count_arrangemets(symbols.to_vec(), nums.to_vec()))
        .sum::<usize>();

    println!("Answer to task 1: {}", ans1);

    let ans2 = inp
        .par_iter_mut()
        .progress()
        .map(|(symbols, nums)| expand(symbols, nums))
        .map(|(symbols, nums)| count_arrangemets(symbols.to_vec(), nums.to_vec()))
        .sum::<usize>();

    println!("Answer to task 2: {}", ans2);

    Ok(())
}
