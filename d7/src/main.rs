#![feature(variant_count)]
#![allow(dead_code)]
use std::fs;
mod hand1;
mod hand2;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    println!("Answer to task 1: {}", task::<hand1::Hand>(&s));
    println!("Answer to task 2: {}", task::<hand2::Hand>(&s));
}

fn task<T>(s: &str) -> u64
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    T: std::str::FromStr + std::cmp::Ord + Copy,
{
    let mut hands: Vec<(T, u64)> = s
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            (hand.parse().unwrap(), bid.parse().unwrap())
        })
        .collect();
    hands.sort_unstable_by_key(|(h, _)| *h);
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i as u64 + 1) * *b)
        .sum::<u64>()
}
