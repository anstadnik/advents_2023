use regex::{Regex, RegexSet};
use std::fs;

fn task_1(s: &str) -> u32 {
    s.lines()
        .map(|line| {
            (
                line.chars().find_map(|c| c.to_digit(10)).unwrap(),
                line.chars().rev().find_map(|c| c.to_digit(10)).unwrap(),
            )
        })
        .fold(0, |acc, (a, b)| acc + a * 10 + b)
}

const MAPPING: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn task_2(s: &str) -> u32 {
    let set = RegexSet::new(MAPPING.iter().map(|(s, _)| s)).unwrap();
    let regexes: Vec<_> = MAPPING
        .into_iter()
        .map(|(s, _)| Regex::new(s).unwrap())
        .collect();

    s.lines()
        .map(|line| {
            let matched_regexes: Vec<_> = set
                .matches(line)
                .into_iter()
                .map(|i| {
                    let mut matches = regexes[i].find_iter(line);
                    let m1 = matches.by_ref().next().unwrap();
                    (i, m1.start(), matches.last().unwrap_or(m1).start())
                })
                .collect();
            let first_idx = matched_regexes.iter().min_by_key(|(_, a, _)| a).unwrap().0;
            let last_idx = matched_regexes.iter().max_by_key(|(_, _, b)| b).unwrap().0;
            (MAPPING[first_idx].1, MAPPING[last_idx].1)
        })
        .fold(0, |acc, (a, b)| acc + a * 10 + b)
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("The answer to task 1 is {}", task_1(&s));
    println!("The answer to task 2 is {}", task_2(&s));
}
